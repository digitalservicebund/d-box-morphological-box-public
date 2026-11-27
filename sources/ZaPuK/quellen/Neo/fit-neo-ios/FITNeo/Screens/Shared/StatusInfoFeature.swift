//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import SwiftUI

@Reducer
struct StatusInfoFeature {
    @ObservableState
    struct State: Equatable {
        var activeStatus: ProcessStatus
        var expandedStatus: Set<ProcessStatus> = []
    }

    enum Action {
        case onAppear
        case collapse(ProcessStatus)
        case expand(ProcessStatus)
    }

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .onAppear:
                state.expandedStatus.insert(state.activeStatus)
                return .none

            case .collapse(let status):
                state.expandedStatus.remove(status)
                return .none

            case .expand(let status):
                state.expandedStatus.insert(status)
                return .none
            }
        }
    }
}
