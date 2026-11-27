//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK
import SwiftUI

enum DocumentsState: Equatable {
    case placeholder
}

@Reducer
struct DocumentsFeature {
    @ObservableState
    struct State: Equatable {
        var documentsState = DocumentsState.placeholder
    }

    enum Action {
        case onAppear
    }

    var body: some Reducer<State, Action> {
        Reduce { _, action in
            switch action {
            case .onAppear:
                .none
            }
        }
    }
}
