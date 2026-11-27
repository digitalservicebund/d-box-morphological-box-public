//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK

@Reducer
struct UnsupportedReplyFeature {
    @ObservableState
    struct State: Equatable, ReplyEditorState {
        var replyContent: [NeoContent] { [] }
    }

    enum Action { }

    var body: some Reducer<State, Action> {
        EmptyReducer()
    }
}
