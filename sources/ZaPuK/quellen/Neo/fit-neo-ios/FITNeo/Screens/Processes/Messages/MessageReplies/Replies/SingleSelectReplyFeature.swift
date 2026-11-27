//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

@Reducer
struct SingleSelectReplyFeature {
    static let customOptionID = "dev.fitko.neo.poll.custom.v1"

    @ObservableState
    struct State: Equatable, ReplyEditorState {
        var options: [NeoPollAnswer] = []
        var selectedOptionID = ""
        var comment = ""

        var isCustomSelected: Bool {
            selectedOptionID == SingleSelectReplyFeature.customOptionID
        }

        var replyContent: [NeoContent] {
            if isCustomSelected {
                comment.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
                    ? []
                    : [.poll(selections: [], custom: comment, replyTo: nil)]
            } else if selectedOptionID.isEmpty {
                []
            } else {
                [.poll(selections: [selectedOptionID], custom: nil, replyTo: nil)]
            }
        }
    }

    enum Action {
        case optionSelected(String)
        case commentChanged(String)
    }

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .optionSelected(let id):
                state.selectedOptionID = id
                state.comment = ""
                return .none

            case .commentChanged(let text):
                state.comment = text
                return .none
            }
        }
    }
}
