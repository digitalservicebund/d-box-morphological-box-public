//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct DocumentsView: View {
    let store: StoreOf<DocumentsFeature>

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading) {
                    NeoEmptyView()
                }
            }
            .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            .navigationTitle(L10n.Navigation.Tab.documents)
            .onAppear {
                store.send(.onAppear)
            }
        }
    }
}

struct DocumentsView_Previews: PreviewProvider {
    static var previews: some View {
        DocumentsView(store: Store(initialState: DocumentsFeature.State()) {
            DocumentsFeature()
        })
    }
}
