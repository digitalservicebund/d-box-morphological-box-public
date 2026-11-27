//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import OSLog
import UIKit

enum AuthResult: Equatable {
    case success
    case failure(AuthenticationServiceError)
    case noSession
}

final class LoginCoordinator: Sendable {
    private let logger = Logger.logger(for: LoginCoordinator.self)
    private let sessionStore = SessionStore()

    func tryRestoreSession() async -> AuthResult {
        do {
            let session = try sessionStore.sessionFromKeychain()

            switch await AppState.shared.authService.restore(session) {
            case .success:
                await setAppStateClient()
                return .success
            case .failure(let error):
                logger.error("Unable to restore existing session: \(error)")
                return .failure(error)
            }
        } catch {
            logger.error("No session available in the keychain: \(error)")
            return .noSession
        }
    }

    func saveCurrentSession() async {
        do {
            let session = try await AppState.shared.getSession()
            sessionStore.saveSessionInKeychain(session: session)
            logger.info("Saved client session to keychain")
        } catch {
            logger.error("Unable to save client session: \(error)")
        }
    }

    func tryLogin() async -> AuthResult {
        switch await AppState.shared.authService
            .configure(for: AppSettings.homeserver,
                       flow: .login) {
        case .success:
            await setAppStateClient()
            if await AppState.shared.authService.loginMode.supportsOIDCFlow {
                return await startOidcFlow()
            } else if await AppState.shared.authService.loginMode.supportsPasswordFlow {
                return await startPasswordFlow()
            }
            return .failure(.loginNotSupported)
        case .failure(let error):
            logger.error("Unable to authenticate: \(error)")
            return .failure(error)
        }
    }

    private func setAppStateClient() async {
        if let client = await AppState.shared.authService.client {
            await AppState.shared.setClient(client)
        }
    }

    func destroySession() async {
        await AppState.shared.destroyClient()
    }

    private func startOidcFlow() async -> AuthResult {
        // MARK: these URLs need to be updated

        guard let redirectURI = URL(string: AppSettings.oidcRedirectURI),
              let clientURI = URL(string: AppSettings.baseURI),
              let logoURI = URL(string: AppSettings.baseURI),
              let tosURI = URL(string: AppSettings.baseURI),
              let policyURI = URL(string: AppSettings.baseURI) else {
            logger.debug("Invalid OIDC URL configuration")
            return .failure(.oidcError(.urlFailure))
        }

        let oidcConfiguration = OIDCConfiguration(clientName: AppSettings.appDisplayName,
                                                  redirectURI: redirectURI,
                                                  clientURI: clientURI,
                                                  logoURI: logoURI,
                                                  tosURI: tosURI,
                                                  policyURI: policyURI,
                                                  staticRegistrations: [:])

        switch await AppState.shared.authService
            .urlForOIDCLogin(loginHint: AppSettings.appDisplayName,
                             oidcConfig: oidcConfiguration) {
        case .success(let oidcData):
            if let window = await AppDelegate.topWindow() {
                return await presentAuthentication(oidcData: oidcData,
                                                   presentationAnchor: window)
            }
            return .failure(.oidcError(.unknown))
        case .failure(let error):
            logger.debug("OIDC failure")
            return .failure(error)
        }
    }

    @MainActor private func presentAuthentication(oidcData: OIDCAuthorizationDataProxy,
                                                  presentationAnchor: UIWindow) async -> AuthResult {
        guard let oidcRedirectUrl = URL(string: AppSettings.oidcRedirectURI) else {
            return .failure(.oidcError(.urlFailure))
        }
        let presenter = WebAuthPresenter(authenticationService: AppState.shared.authService,
                                         oidcRedirectURL: oidcRedirectUrl,
                                         presentationAnchor: presentationAnchor)

        switch await presenter.authenticate(using: oidcData) {
        case .success:
            logger.debug("Successfully authenticated!")
            return .success
        case .failure(let error):
            logger.error("Authentication failed: \(error)")
            return .failure(error)
        }
    }

    // this is just for testing a quick auth flow without OIDC
    private func startPasswordFlow() async -> AuthResult {
        guard let client = await AppState.shared.authService.client else {
            logger.debug("Authentication failed")
            return .failure(.failedLoggingIn)
        }

        // MARK: obviously change this

        let username = ""
        let password = ""

        do {
            try await client
                .login(username: username,
                       password: password,
                       initialDeviceName: AppSettings.appIdentifier,
                       deviceId: nil)
            logger.debug("Successfully authenticated!")
            return .success
        } catch {
            logger.error("Authentication failed: \(error)")
            return .failure(.failedLoggingIn)
        }
    }
}
