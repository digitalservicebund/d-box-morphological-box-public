//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
// SPDX-FileCopyrightText: 2024 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

import AuthenticationServices
import OSLog

/// Presents a web authentication session for an OIDC request.
@MainActor
class WebAuthPresenter: NSObject {
    private let logger = Logger.logger(for: WebAuthPresenter.self)

    private let authenticationService: AuthenticationServiceProtocol
    private let oidcRedirectURL: URL
    private let presentationAnchor: UIWindow

    private var activeSession: ASWebAuthenticationSession?

    init(authenticationService: AuthenticationServiceProtocol,
         oidcRedirectURL: URL,
         presentationAnchor: UIWindow) {
        self.authenticationService = authenticationService
        self.oidcRedirectURL = oidcRedirectURL
        self.presentationAnchor = presentationAnchor
        super.init()
    }

    /// Presents a web authentication session for the supplied data.
    func authenticate(using oidcData: OIDCAuthorizationDataProxy) async -> Result<Bool, AuthenticationServiceError> {
        guard let loginURL = oidcData.url else {
            logger.error("The OIDC authorization data did not contain a valid login URL.")
            await authenticationService.abortOIDCLogin(data: oidcData)
            return .failure(.oidcError(.urlFailure))
        }

        let (url, error) = await withCheckedContinuation { continuation in
            let session = ASWebAuthenticationSession(url: loginURL,
                                                     callback: .oidcRedirectURL(oidcRedirectURL)) { url, error in
                continuation.resume(returning: (url, error))
            }

            session.prefersEphemeralWebBrowserSession = false
            session.presentationContextProvider = self

            activeSession = session
            session.start()
        }

        activeSession = nil

        guard let url else {
            // Check for user cancellation to avoid showing an alert in that instance.
            if let nsError = error as? NSError,
               nsError.domain == ASWebAuthenticationSessionErrorDomain,
               nsError.code == ASWebAuthenticationSessionError.canceledLogin.rawValue,
               // If there's a failure reason then the cancellation wasn't made by the user.
               nsError.localizedFailureReason == nil {
                // No need to show an error here, just abort and return a failure.
                await authenticationService.abortOIDCLogin(data: oidcData)
                return .failure(.oidcError(.userCancellation))
            }

            let errorDescription = error.map(String.init(describing:)) ?? "Unknown error"
            logger.debug("Missing callback URL from the web authentication session: \(errorDescription)")

            await authenticationService.abortOIDCLogin(data: oidcData)
            return .failure(.oidcError(.unknown))
        }

        startLoading(delay: .milliseconds(50))
        defer { stopLoading() }

        switch await authenticationService.loginWithOIDCCallback(url) {
        case .success(let userSession):
            return .success(userSession)
        case .failure(.oidcError(.userCancellation)):
            // No need to show an error here, just return the failure.
            return .failure(.oidcError(.userCancellation))
        case .failure(let error):
            logger.error("Error occurred: \(error)")
            return .failure(error)
        }
    }

    func cancel() {
        activeSession?.cancel()
    }

    private static let loadingIndicatorID = "\(WebAuthPresenter.self)-Loading"

    private func startLoading(delay: Duration? = nil) {
        logger.debug("Start loading...")
    }

    private func stopLoading() {
        logger.debug("Stop loading...")
    }
}

// MARK: ASWebAuthenticationPresentationContextProviding

extension WebAuthPresenter: ASWebAuthenticationPresentationContextProviding {
    func presentationAnchor(for session: ASWebAuthenticationSession) -> ASPresentationAnchor { presentationAnchor }
}

extension ASWebAuthenticationSession.Callback {
    static func oidcRedirectURL(_ url: URL) -> Self {
        if url.scheme == "https", let host = url.host() {
            .https(host: host, path: url.path())
        } else if let scheme = url.scheme {
            .customScheme(scheme)
        } else {
            fatalError("Invalid OIDC redirect URL: \(url)")
        }
    }
}
