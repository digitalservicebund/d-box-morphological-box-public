//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

enum SubmitReplyStatus {
    case unsent
    case sending
    case sent
    case failed
}

@Reducer
struct ReplyConfirmationFeature {
    @ObservableState
    struct State: Equatable {
        var inquiryContent: NeoContent?
        var previewContent: [NeoContent] = []
        var submitStatus: SubmitReplyStatus = .unsent
    }

    enum Action {
        case onAppear([NeoContent])
        case cancelSend
        case confirmSend
        case sendSuccess
        case sendError
    }

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .onAppear(let reply):
                state.previewContent = reply
                return .none

            case .confirmSend:
                state.submitStatus = .sending
                return .none

            case .cancelSend:
                state.submitStatus = .unsent
                return .none

            case .sendSuccess:
                state.submitStatus = .sent
                return .none

            case .sendError:
                state.submitStatus = .failed
                return .none
            }
        }
    }
}
