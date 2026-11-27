// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK
import SwiftUI

@Reducer
struct MessageDetailsFeature {
    @ObservableState
    struct State: Equatable {
        var expandedHeader = false
        var caseId: String
        var authority: String
        var dialog: NeoDialogProxy
        var openBrowserFeature = OpenBrowserFeature.State()
        var replyFeature = ReplyFeature.State()
        var replyConfirmationFeature = ReplyConfirmationFeature.State()
        var selectedAttachmentURL: URL?
        var attachmentURLs: [URL] = []
        var isDownloadingAttachment = false
    }

    enum Action {
        case onAppear
        case toggleHeader
        case openBrowserAction(OpenBrowserFeature.Action)
        case replyAction(ReplyFeature.Action)
        case replyConfirmationAction(ReplyConfirmationFeature.Action)
        case previewAttachment(AttachmentType)
        case attachmentDownloaded(URL)
        case dismissAttachmentPreview
        case replyFailed(String)
    }

    static let replyTimeout: Duration = .seconds(10)

    @Dependency(\.mediaServiceClient) var mediaServiceClient
    @Dependency(\.messageServiceClient) var messageServiceClient
    @Dependency(\.continuousClock) var clock

    private enum CancelID { case replyTimeout }

    var body: some Reducer<State, Action> {
        Scope(state: \.openBrowserFeature, action: \.openBrowserAction) {
            OpenBrowserFeature()
        }

        Scope(state: \.replyFeature, action: \.replyAction) {
            ReplyFeature()
        }

        Scope(state: \.replyConfirmationFeature, action: \.replyConfirmationAction) {
            ReplyConfirmationFeature()
        }

        Reduce { state, action in
            switch action {
            case .onAppear:
                if state.replyFeature.content == nil {
                    state.replyFeature.content = state.dialog.inquiryContent
                    state.replyFeature.editor = ReplyEditor.State.make(for: state.dialog.inquiryContent)
                }
                state.replyFeature.dueDate = state.dialog.dueDate
                state.replyFeature.answers = state.dialog.answers
                return .none

            case .toggleHeader:
                state.expandedHeader.toggle()
                return .none

            case .openBrowserAction:
                return .none

            case .replyAction(.submitReply):
                guard !state.replyFeature.isConfirmingReply else {
                    return .none
                }

                guard state.replyFeature.isValidReply else {
                    return .none
                }

                state.replyFeature.isConfirmingReply = true
                state.replyConfirmationFeature.inquiryContent = state.replyFeature.content
                state.replyConfirmationFeature.previewContent = state.replyFeature.replyContent
                state.replyConfirmationFeature.submitStatus = .unsent
                return .none

            case .replyAction:
                return .none

            case .replyConfirmationAction(.confirmSend):
                state.replyConfirmationFeature.submitStatus = .sending
                let caseId = state.caseId
                let dialogId = state.dialog.id
                let text = state.replyConfirmationFeature.previewContent
                return .merge(.run { send in
                    do {
                        _ = try await messageServiceClient.sendAnswer(caseId, dialogId, text)
                    } catch {
                        await send(.replyFailed(error.localizedDescription))
                    }
                },
                .run { [clock] send in
                    try await clock.sleep(for: Self.replyTimeout)
                    await send(.replyFailed("Timeout sending the reply"))
                }
                .cancellable(id: CancelID.replyTimeout, cancelInFlight: true))

            case .replyConfirmationAction(.cancelSend):
                state.replyFeature.isConfirmingReply = false
                state.replyConfirmationFeature.submitStatus = .unsent
                return .none

            case .replyConfirmationAction(.sendSuccess):
                state.replyFeature.isConfirmingReply = false
                state.replyFeature.editor = ReplyEditor.State.make(for: state.replyFeature.content)
                return .none

            case .replyConfirmationAction:
                return .none

            case .replyFailed:
                guard state.replyConfirmationFeature.submitStatus == .sending else { return .none }
                state.replyConfirmationFeature.submitStatus = .failed
                state.replyFeature.isConfirmingReply = false
                return .cancel(id: CancelID.replyTimeout)

            case .previewAttachment(let attachment):
                state.isDownloadingAttachment = true
                return .run { send in
                    do {
                        let url = try await mediaServiceClient.download(attachment)
                        await send(.attachmentDownloaded(url))
                    } catch {
                        await send(.dismissAttachmentPreview)
                    }
                }

            case .attachmentDownloaded(let url):
                state.isDownloadingAttachment = false
                state.attachmentURLs = [url]
                state.selectedAttachmentURL = url
                return .none

            case .dismissAttachmentPreview:
                state.isDownloadingAttachment = false
                state.selectedAttachmentURL = nil
                state.attachmentURLs = []
                return .none
            }
        }
    }
}
