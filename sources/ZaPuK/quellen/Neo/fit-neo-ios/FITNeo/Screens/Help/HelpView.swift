//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct HelpView: View {
    let store: StoreOf<HelpFeature>

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading) {
                    NeoEmptyView()
                }
            }
            .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            .navigationTitle(L10n.Navigation.Tab.help)
            .onAppear {
                store.send(.onAppear)
            }
        }
    }
}

struct HelpView_Previews: PreviewProvider {
    static var previews: some View {
        HelpView(store: Store(initialState: HelpFeature.State()) {
            HelpFeature()
        })
    }
}
