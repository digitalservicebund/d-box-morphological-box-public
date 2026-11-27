//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
// SPDX-FileCopyrightText: 2022-2024 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

/// The supported forms of login that a homeserver allows.
enum LoginMode: Equatable {
    /// The login mode hasn't been determined yet.
    case unknown
    /// The homeserver supports login via OpenID Connect.
    case oidc(supportsCreatePrompt: Bool)
    /// The homeserver supports login with a password.
    case password
    /// The homeserver only allows login with unsupported mechanisms. Use fallback instead.
    case unsupported

    var supportsOIDCFlow: Bool {
        switch self {
        case .oidc: true
        default: false
        }
    }

    var supportsPasswordFlow: Bool {
        switch self {
        case .password: true
        default: false
        }
    }
}
