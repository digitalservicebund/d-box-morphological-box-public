//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK
import OSLog

final class SessionStore: ClientSessionDelegate {
    private let logger = Logger.logger(for: SessionStore.self)

    private var previousSessionUserId: String {
        get { UserDefaults.standard.string(forKey: "previousSessionUserId") ?? "" }
        set { UserDefaults.standard.set(newValue, forKey: "previousSessionUserId") }
    }

    func sessionFromKeychain() throws -> Session {
        if let keychainData = try KeychainWrapper().load(forKey: previousSessionUserId) as Data? {
            let sessionData = try JSONDecoder().decode(UserSessionProxy.self,
                                                       from: keychainData) as UserSessionProxy
            logger.info("Retrieved previous session data from keychain")

            return sessionData.matrixSession()
        } else {
            logger.error("Failed to retrieve session data")
            throw KeychainError.sessionNotFound(userId: nil)
        }
    }

    func retrieveSessionFromKeychain(userId: String) throws -> Session {
        let session = try sessionFromKeychain()
        if session.userId != userId {
            throw KeychainError.sessionNotFound(userId: userId)
        }
        return session
    }

    func saveSessionInKeychain(session: MatrixRustSDK.Session) {
        do {
            let keychainData = try JSONEncoder().encode(UserSessionProxy(session: session))
            try KeychainWrapper().persist(keychainData, forKey: session.userId)

            previousSessionUserId = session.userId

            logger.debug("Saved session for \(session.userId) in the keychain")
        } catch {
            logger.error("Failed to save session for \(session.userId) in the keychain: \(error)")
        }
    }
}
