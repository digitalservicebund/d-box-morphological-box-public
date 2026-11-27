//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import SwiftUI

enum LoginState: Equatable {
    case restoringSession
    case loggedIn
    case loggingIn
    case loggedOut
}

@Reducer
struct LoginFeature {
    @ObservableState
    struct State: Equatable {
        var loginState = LoginState.loggedOut
        var loginError: AuthenticationServiceError?
    }

    enum Action: Equatable {
        case onAppear
        case loginButtonTapped
        case authSucceeded
        case authFailed(AuthenticationServiceError)
        case logout
        case logoutCompleted
    }

    @Dependency(\.loginClient) var loginClient

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .onAppear:
                state.loginState = .restoringSession
                return .run { [self] send in
                    #if DEBUG
                    if ProcessInfo.processInfo.environment["BYPASS_AUTH"] == "true" {
                        return
                    }
                    #endif

                    let result = await loginClient.tryRestoreSession()
                    switch result {
                    case .success:
                        await send(.authSucceeded)
                    case .failure(let error):
                        await send(.authFailed(error))
                    case .noSession:
                        await send(.logout)
                    }
                }

            case .loginButtonTapped:
                state.loginError = nil
                state.loginState = .loggingIn
                return .run { [self] send in
                    let result = await loginClient.tryLogin()
                    switch result {
                    case .success:
                        await send(.authSucceeded)
                    case .failure(let error):
                        await send(.authFailed(error))
                    case .noSession:
                        await send(.logout)
                    }
                }

            case .authSucceeded:
                state.loginError = nil
                state.loginState = .loggedIn
                return .run { [self] _ in
                    await loginClient.saveCurrentSession()
                }

            case let .authFailed(error):
                state.loginState = .loggedOut
                if case .oidcError(.userCancellation) = error {
                    state.loginError = nil
                } else {
                    state.loginError = error
                }
                return .none

            case .logout:
                return .run { [self] send in
                    await loginClient.destroySession()
                    await send(.logoutCompleted)
                }

            case .logoutCompleted:
                state.loginState = .loggedOut
                return .none
            }
        }
    }
}

struct LoginClient {
    var tryRestoreSession: @Sendable () async -> AuthResult
    var tryLogin: @Sendable () async -> AuthResult
    var saveCurrentSession: @Sendable () async -> Void
    var destroySession: @Sendable () async -> Void
}

extension LoginClient: DependencyKey {
    static let liveValue: LoginClient = {
        let coordinator = LoginCoordinator()
        return LoginClient(tryRestoreSession: { await coordinator.tryRestoreSession() },
                           tryLogin: { await coordinator.tryLogin() },
                           saveCurrentSession: { await coordinator.saveCurrentSession() },
                           destroySession: { await coordinator.destroySession() })
    }()

    static let testValue = LoginClient(tryRestoreSession: { .noSession },
                                       tryLogin: { .noSession },
                                       saveCurrentSession: { },
                                       destroySession: { })
}

extension DependencyValues {
    var loginClient: LoginClient {
        get { self[LoginClient.self] }
        set { self[LoginClient.self] = newValue }
    }
}
