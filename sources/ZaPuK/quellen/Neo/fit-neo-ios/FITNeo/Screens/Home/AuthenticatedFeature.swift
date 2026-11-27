//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture

@Reducer
struct AuthenticatedFeature {
    @ObservableState
    struct State: Equatable {
        var selectedTab: NeoTabs = .processes
        var processes = ProcessesViewFeature.State.initial()
        var documents = DocumentsFeature.State()
        var account = AccountFeature.State()
        var help = HelpFeature.State()
    }

    enum Action {
        case tabSelectionChanged(NeoTabs)
        case processes(ProcessesViewFeature.Action)
        case documents(DocumentsFeature.Action)
        case account(AccountFeature.Action)
        case help(HelpFeature.Action)
    }

    var body: some Reducer<State, Action> {
        Scope(state: \.processes, action: \.processes) {
            ProcessesViewFeature()
        }

        Scope(state: \.documents, action: \.documents) {
            DocumentsFeature()
        }

        Scope(state: \.account, action: \.account) {
            AccountFeature()
        }

        Scope(state: \.help, action: \.help) {
            HelpFeature()
        }

        Reduce { state, action in
            switch action {
            case let .tabSelectionChanged(tab):
                state.selectedTab = tab
                return .none

            case .processes(.openHelpButtonTapped):
                state.selectedTab = .help
                return .none

            case .processes, .documents, .account, .help:
                return .none
            }
        }
    }
}
