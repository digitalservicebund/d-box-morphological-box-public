//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

@Reducer
struct FreeTextReplyFeature {
    @ObservableState
    struct State: Equatable, ReplyEditorState {
        var template = ""
        var textContent = ""

        var replyContent: [NeoContent] {
            textContent.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
                ? []
                : [.message(text: NeoText(plain: textContent, html: nil), replyTo: nil)]
        }
    }

    enum Action {
        case contentChanged(String)
        case contentCleared
        case setTemplate(String)
        case addReplyTemplate
    }

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .contentChanged(let content):
                state.textContent = content
                return .none

            case .contentCleared:
                state.textContent = ""
                return .none

            case .setTemplate(let displayName):
                state.template = L10n.Screen.Message.Reply.template(displayName)
                return .none

            case .addReplyTemplate:
                state.textContent = state.template
                return .none
            }
        }
    }
}
