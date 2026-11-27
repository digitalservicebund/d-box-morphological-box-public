//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation

class AppSettings {
    static let appDisplayName = L10n.App.displayName
    static let appIdentifier = Bundle.main.bundleIdentifier ?? "dev.fitko.neo"
    static let baseURI = "https://neo.fitko.dev"
    static let oidcRedirectURI = "fit-neo://neo.fitko.dev?oauth2redirect"

    static var homeserver: String {
        fromInfoPlist(for: "TARGET_HOMESERVER")
    }

    static var federalPortalURL: String {
        fromInfoPlist(for: "FEDERAL_PORTAL_URL")
    }

    private static func fromInfoPlist(for key: String) -> String {
        guard let object = Bundle.main.object(forInfoDictionaryKey: key) as? String else {
            fatalError("Missing expected string for key: \(key)")
        }
        return object
    }
}
