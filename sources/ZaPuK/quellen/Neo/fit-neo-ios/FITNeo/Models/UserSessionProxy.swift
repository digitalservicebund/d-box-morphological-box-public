//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK

class UserSessionProxy: Codable {
    private let accessToken: String
    private let refreshToken: String?
    private let userId: String
    private let deviceId: String
    private let homeserverUrl: String
    private let oidcData: String?

    init(session: Session) {
        accessToken = session.accessToken
        refreshToken = session.refreshToken
        userId = session.userId
        deviceId = session.deviceId
        homeserverUrl = session.homeserverUrl
        oidcData = session.oauthData
    }

    func matrixSession() -> Session {
        Session(accessToken: accessToken,
                refreshToken: refreshToken,
                userId: userId,
                deviceId: deviceId,
                homeserverUrl: homeserverUrl,
                oauthData: oidcData,
                slidingSyncVersion: .native)
    }
}
