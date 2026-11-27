//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
// SPDX-FileCopyrightText: 2022-2024 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import Foundation
import MatrixRustSDK

/// Represents a particular authentication flow.
enum AuthenticationFlow {
    /// The flow for signing in to an existing account.
    case login
    /// The flow for creating a new account.
    case register
}

enum AuthenticationServiceError: Error, Equatable {
    /// An error occurred during OIDC authentication.
    case oidcError(OIDCError)

    case invalidServer
    case invalidCredentials
    case invalidSession
    case invalidHomeserverAddress
    case invalidWellKnown(String)
    case slidingSyncNotAvailable
    case loginNotSupported
    case registrationNotSupported
    case accountDeactivated
    case failedLoggingIn
    case sessionTokenRefreshNotSupported
    case failedUsingWebCredentials
}

protocol AuthenticationServiceProtocol {
    // The login mode
    var loginMode: LoginMode { get }

    // The client interface
    var client: ClientProtocol? { get }

    /// Sets up the service for login on the specified homeserver address.
    func configure(for homeserverAddress: String,
                   flow: AuthenticationFlow)
        async -> Result<Void, AuthenticationServiceError>

    /// Restores a previously stored session
    func restore(_ session: Session) async -> Result<Void, AuthenticationServiceError>

    /// Performs login using OIDC for the current homeserver.
    func urlForOIDCLogin(loginHint: String?,
                         oidcConfig: OIDCConfiguration)
        async -> Result<OIDCAuthorizationDataProxy, AuthenticationServiceError>

    /// Asks the SDK to abort an ongoing OIDC login if we didn't get a callback to complete the request with.
    func abortOIDCLogin(data: OIDCAuthorizationDataProxy) async

    /// Completes an OIDC login that was started using ``urlForOIDCLogin``.
    func loginWithOIDCCallback(_ callbackURL: URL) async -> Result<Bool, AuthenticationServiceError>
}

enum OIDCError: Error {
    /// Failed to get the URL that should be presented for login.
    case urlFailure
    /// The user cancelled the login.
    case userCancellation
    /// OIDC isn't supported on the currently configured server.
    case notSupported
    /// An unknown error occurred.
    case unknown
}

struct OIDCAuthorizationDataProxy: Hashable {
    let underlyingData: OAuthAuthorizationData

    var url: URL? {
        URL(string: underlyingData.loginUrl())
    }
}

extension OAuthAuthorizationData: @retroactive Hashable {
    public static func == (lhs: MatrixRustSDK.OAuthAuthorizationData,
                           rhs: MatrixRustSDK.OAuthAuthorizationData) -> Bool {
        lhs.loginUrl() == rhs.loginUrl()
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(loginUrl())
    }
}
