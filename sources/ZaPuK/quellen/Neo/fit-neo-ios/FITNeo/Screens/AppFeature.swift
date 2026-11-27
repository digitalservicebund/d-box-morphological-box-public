//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture

@Reducer
struct AppFeature {
    @ObservableState
    struct State: Equatable {
        var login = LoginFeature.State()
        var authenticated = AuthenticatedFeature.State()
    }

    enum Action {
        case login(LoginFeature.Action)
        case authenticated(AuthenticatedFeature.Action)
    }

    var body: some Reducer<State, Action> {
        Scope(state: \.login, action: \.login) {
            LoginFeature()
        }

        Scope(state: \.authenticated, action: \.authenticated) {
            AuthenticatedFeature()
        }

        Reduce { state, action in
            switch action {
            case .login(.logout):
                // Discard the entire authenticated session so nothing from the previous
                // user state leaks into the next one.
                state.authenticated = AuthenticatedFeature.State()
                return .send(.authenticated(.processes(.reset)))

            case .login, .authenticated:
                return .none
            }
        }
    }
}
