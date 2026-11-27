//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

@Reducer
struct ReplyFeature {
    @ObservableState
    struct State: Equatable {
        var content: NeoContent?
        var answers: [NeoEnvelope] = []
        var displayName = ""
        var dueDate: Date?
        var isExpanded = false
        var editor: ReplyEditor.State
        var isConfirmingReply = false
        var isSendingReply = false

        var replyContent: [NeoContent] {
            editor.replyContent
        }

        var isValidReply: Bool {
            editor.replyContent.isEmpty == false
        }

        init(content: NeoContent? = nil,
             answers: [NeoEnvelope] = [],
             displayName: String = "",
             dueDate: Date? = nil,
             isExpanded: Bool = false,
             isConfirmingReply: Bool = false,
             isSendingReply: Bool = false) {
            self.content = content
            self.answers = answers
            self.displayName = displayName
            self.dueDate = dueDate
            self.isExpanded = isExpanded
            self.isConfirmingReply = isConfirmingReply
            self.isSendingReply = isSendingReply
            editor = ReplyEditor.State.make(for: content)
        }
    }

    enum Action {
        case onAppear
        case displayNameResponse(String)
        case toggleExpanded
        case editor(ReplyEditor.Action)
        case submitReply
    }

    @Dependency(\.userSessionClient) var userSessionClient

    var body: some Reducer<State, Action> {
        Scope(state: \.editor, action: \.editor) {
            ReplyEditor.body
        }

        Reduce { state, action in
            switch action {
            case .onAppear:
                return .run { send in
                    if let name = await userSessionClient.displayName() {
                        await send(.displayNameResponse(name))
                    }
                }

            case .displayNameResponse(let displayName):
                state.displayName = displayName
                return .none

            case .toggleExpanded:
                state.isExpanded.toggle()
                return .none

            case .editor:
                return .none

            case .submitReply:
                return .none
            }
        }
    }
}
