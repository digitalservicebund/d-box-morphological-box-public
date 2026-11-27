//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

@Reducer
enum ReplyEditor {
    case freeText(FreeTextReplyFeature)
    case singleSelect(SingleSelectReplyFeature)
    case unsupported(UnsupportedReplyFeature)
}

protocol ReplyEditorState: Equatable {
    var replyContent: [NeoContent] { get }
}

extension ReplyEditor.State: ReplyEditorState {
    var replyContent: [NeoContent] {
        switch self {
        case .freeText(let state): state.replyContent
        case .singleSelect(let state): state.replyContent
        case .unsupported(let state): state.replyContent
        }
    }
}

extension ReplyEditor.State {
    static func make(for content: NeoContent?) -> ReplyEditor.State {
        switch content {
        case .inquiryMessage:
            .freeText(FreeTextReplyFeature.State())
        case .inquiryPoll(let poll):
            .singleSelect(SingleSelectReplyFeature.State(options: poll.answers))
        default:
            .unsupported(UnsupportedReplyFeature.State())
        }
    }
}

protocol ReplyEditorView: View {
    associatedtype Feature: Reducer where Feature.State: ReplyEditorState
    associatedtype AnswerContent: View
    associatedtype ConfirmationContent: View

    init(store: StoreOf<Feature>)

    @ViewBuilder static func answerView(_ envelope: NeoEnvelope, inquiry: NeoContent?) -> AnswerContent
    @ViewBuilder static func confirmationView(_ contents: [NeoContent],
                                              inquiry: NeoContent?) -> ConfirmationContent
}
