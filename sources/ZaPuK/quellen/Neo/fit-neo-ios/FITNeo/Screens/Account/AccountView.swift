//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import OSLog
import SFSafeSymbols
import SwiftUI

struct AccountView: View {
    let store: StoreOf<AccountFeature>
    let onLogoutTapped: () -> Void

    init(store: StoreOf<AccountFeature>, onLogoutTapped: @escaping () -> Void = { }) {
        self.store = store
        self.onLogoutTapped = onLogoutTapped
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading) {
                    NeoEmptyView()
                }
                .frame(maxWidth: .infinity)
            }
            .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button {
                        onLogoutTapped()
                    } label: {
                        Image(systemSymbol: .rectanglePortraitAndArrowRight)
                    }
                    .tint(Asset.Theme.actionsDefault.swiftUIColor)
                }
            }
            .navigationTitle(L10n.Navigation.Tab.account)
            .onAppear {
                store.send(.onAppear)
            }
        }
    }
}

struct AccountView_Previews: PreviewProvider {
    static var previews: some View {
        AccountView(store: Store(initialState: AccountFeature.State()) {
            AccountFeature()
        })
    }
}
