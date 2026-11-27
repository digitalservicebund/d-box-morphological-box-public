//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SwiftUI

struct ApplicationTabs: View {
    let store: StoreOf<AuthenticatedFeature>
    let onLogout: () -> Void

    private var selectedTab: Binding<NeoTabs> {
        Binding(get: { store.selectedTab },
                set: { store.send(.tabSelectionChanged($0)) })
    }

    var body: some View {
        TabView(selection: selectedTab) {
            Tab(NeoTabs.processes.name,
                systemSymbol: NeoTabs.processes.symbol,
                value: NeoTabs.processes) {
                    ProcessesView(store: store.scope(state: \.processes, action: \.processes))
                }
                .accessibilityIdentifier("Tab.Processes")
            Tab(NeoTabs.documents.name,
                systemSymbol: NeoTabs.documents.symbol,
                value: NeoTabs.documents) {
                    DocumentsView(store: store.scope(state: \.documents, action: \.documents))
                }
                .accessibilityIdentifier("Tab.Documents")
            Tab(NeoTabs.account.name,
                systemSymbol: NeoTabs.account.symbol,
                value: NeoTabs.account) {
                    AccountView(store: store.scope(state: \.account, action: \.account)) {
                        onLogout()
                    }
                }
                .accessibilityIdentifier("Tab.Account")
            Tab(NeoTabs.help.name,
                systemSymbol: NeoTabs.help.symbol,
                value: NeoTabs.help) {
                    HelpView(store: store.scope(state: \.help, action: \.help))
                }
                .accessibilityIdentifier("Tab.Help")
        }
    }
}
