//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
// SPDX-FileCopyrightText: 2022-2024 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK
import OSLog

class AuthenticationService: AuthenticationServiceProtocol {
    private let logger = Logger.logger(for: AuthenticationService.self)

    var clientFactory: MatrixClientFactory
    var client: ClientProtocol?
    var loginMode: LoginMode = .unknown

    private let sessionStore = SessionStore()

    init(clientFactory factory: MatrixClientFactory) {
        clientFactory = factory
    }

    func configure(for homeserverAddress: String,
                   flow: AuthenticationFlow) async -> Result<Void, AuthenticationServiceError> {
        do {
            guard let client = try await makeNewClient(homeserverAddress: homeserverAddress) else {
                return .failure(.invalidHomeserverAddress)
            }

            var prefersPasswordFlow = false
            #if DEBUG
            if ProcessInfo.processInfo.environment["PASSWORD_AUTH"] == "true" {
                prefersPasswordFlow = true
            }
            #endif

            let loginDetails = await client.homeserverLoginDetails()
            loginMode = if loginDetails.supportsOauthLogin(), !prefersPasswordFlow {
                .oidc(
                    supportsCreatePrompt: loginDetails
                        .supportedOauthPrompts()
                        .contains(.login)
                )
            } else if loginDetails.supportsPasswordLogin(), prefersPasswordFlow {
                .password
            } else {
                .unsupported
            }

            if flow == .login, loginMode == .unsupported {
                return .failure(.loginNotSupported)
            }

            self.client = client

            return .success(())
        } catch ClientBuildError.WellKnownDeserializationError(let error) {
            logger.error("The user entered a server with an invalid well-known file: \(error)")
            return .failure(.invalidWellKnown(error))
        } catch ClientBuildError.SlidingSyncVersion(let error) {
            logger.error("User entered a homeserver that isn't configured for sliding sync: \(error)")
            return .failure(.slidingSyncNotAvailable)
        } catch {
            logger.error("Failed configuring a server: \(error)")
            return .failure(.invalidHomeserverAddress)
        }
    }

    func restore(_ session: Session) async -> Result<Void, AuthenticationServiceError> {
        do {
            guard let builder = try await makeClientBuilder(session.homeserverUrl) else {
                return .failure(.invalidHomeserverAddress)
            }

            let client = try await builder.build()
            try await client.restoreSession(session: session)

            self.client = client

            return .success(())
        } catch {
            logger.error("Failed restoring previous session: \(error)")
            return .failure(.invalidSession)
        }
    }

    func urlForOIDCLogin(loginHint: String?,
                         oidcConfig: OIDCConfiguration)
        async -> Result<OIDCAuthorizationDataProxy, AuthenticationServiceError> {
        guard let client else { return .failure(.oidcError(.urlFailure)) }

        do {
            // The create prompt is broken: https://github.com/element-hq/matrix-authentication-service/issues/3429
            // let prompt: OidcPrompt = flow == .register ? .create : .consent
            let oidcData = try await client.urlForOauth(oauthConfiguration: oidcConfig.rustValue,
                                                        prompt: .consent,
                                                        loginHint: loginHint,
                                                        deviceId: nil,
                                                        additionalScopes: nil)
            return .success(OIDCAuthorizationDataProxy(underlyingData: oidcData))
        } catch {
            logger.error("Failed to get URL for OIDC login: \(error)")
            return .failure(.oidcError(.urlFailure))
        }
    }

    func abortOIDCLogin(data: OIDCAuthorizationDataProxy) async {
        guard let client else { return }
        logger.debug("Aborting OIDC login.")
        await client.abortOauthAuth(authorizationData: data.underlyingData)
    }

    func loginWithOIDCCallback(_ callbackURL: URL) async -> Result<Bool, AuthenticationServiceError> {
        guard let client else { return .failure(.failedLoggingIn) }
        do {
            try await client.loginWithOauthCallback(callbackUrl: callbackURL.absoluteString)
            return .success(true)
        } catch OAuthError.Cancelled {
            return .failure(.oidcError(.userCancellation))
        } catch {
            logger.error("Login with OIDC failed: \(error)")
            return .failure(.failedLoggingIn)
        }
    }

    private func makeClientBuilder(_ homeserverAddress: String) async throws -> ClientBuilder? {
        guard let documentsURL = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first,
              let cachesURL = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask).first else {
            logger.error("Could not find document or cache directory.")
            return nil
        }

        var builder = clientFactory.makeClientBuilder(homeserverAddress,
                                                      documentsURL.path,
                                                      cachesURL.path)

        builder = builder
            .setSessionDelegate(sessionDelegate: sessionStore)
            .crossProcessLockConfig(
                crossProcessLockConfig: CrossProcessLockConfig
                    .multiProcess(holderName: AppSettings.appIdentifier)
            )

        return builder
    }

    private func makeNewClient(homeserverAddress: String) async throws -> ClientProtocol? {
        FileManager.default.clearAppData()

        if let builder = try await makeClientBuilder(homeserverAddress) {
            return try await builder.build()
        }
        return nil
    }
}
