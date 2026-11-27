//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SwiftUI

struct HomeView: View {
    let store: StoreOf<AppFeature>

    var body: some View {
        ApplicationTabs(store: store.scope(state: \.authenticated, action: \.authenticated)) {
            store.send(.login(.logout))
        }
    }
}
