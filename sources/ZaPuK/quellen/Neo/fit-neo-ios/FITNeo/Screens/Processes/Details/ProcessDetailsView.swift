//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct ProcessDetailsView: View {
    let store: StoreOf<ProcessDetailsFeature>

    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                NeoEmptyView()
            }
        }
        .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
        .navigationTitle(L10n.Screen.ProcessDetailsView.title)
        .onAppear {
            store.send(.onAppear)
        }
    }
}

struct ProcessDetailsView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationStack {
            ProcessDetailsView(store: Store(initialState: ProcessDetailsFeature.State()) {
                ProcessDetailsFeature()
            })
        }
    }
}
