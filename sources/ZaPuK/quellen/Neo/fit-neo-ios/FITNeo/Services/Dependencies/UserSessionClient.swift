//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture

struct UserSessionClient {
    var displayName: @Sendable () async -> String?
}

extension UserSessionClient: DependencyKey {
    static let liveValue = UserSessionClient {
        await MainActor.run { try? AppState.shared.userDisplayName() }
    }

    static let testValue = UserSessionClient { nil }
}

extension DependencyValues {
    var userSessionClient: UserSessionClient {
        get { self[UserSessionClient.self] }
        set { self[UserSessionClient.self] = newValue }
    }
}
