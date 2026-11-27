// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SwiftUI

struct AppView: View {
    @State private var store: StoreOf<AppFeature>

    init() {
        var initialState = AppFeature.State()
        #if DEBUG
        if ProcessInfo.processInfo.environment["BYPASS_AUTH"] == "true" {
            initialState.login.loginState = .loggedIn
        }
        #endif
        _store = State(wrappedValue: Store(initialState: initialState) {
            AppFeature()
        })
    }

    var body: some View {
        Group {
            if store.login.loginState == .loggedIn {
                HomeView(store: store)
            } else {
                LoginView(store: store.scope(state: \.login, action: \.login))
            }
        }
        .accentColor(Asset.Theme.actionsDefault.swiftUIColor)
    }
}
