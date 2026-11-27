//
// SPDX-FileCopyrightText: 2024 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation

struct OIDCConfiguration {
    let clientName: String
    let redirectURI: URL
    let clientURI: URL
    let logoURI: URL
    let tosURI: URL
    let policyURI: URL
    let staticRegistrations: [String: String]
}

#if canImport(MatrixRustSDK)
import MatrixRustSDK

extension OIDCConfiguration {
    var rustValue: OAuthConfiguration {
        OAuthConfiguration(clientName: clientName,
                           redirectUri: redirectURI.absoluteString,
                           clientUri: clientURI.absoluteString,
                           logoUri: logoURI.absoluteString,
                           tosUri: tosURI.absoluteString,
                           policyUri: policyURI.absoluteString,
                           staticRegistrations: staticRegistrations)
    }
}
#endif
