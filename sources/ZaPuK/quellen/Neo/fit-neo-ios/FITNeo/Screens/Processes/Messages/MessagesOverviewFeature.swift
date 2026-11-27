//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK
import SwiftUI

typealias TimelineDiffs = [TimelineDiff]

extension TimelineItem: @retroactive Equatable {
    public static func == (lhs: MatrixRustSDK.TimelineItem, rhs: MatrixRustSDK.TimelineItem) -> Bool {
        lhs.uniqueId() == rhs.uniqueId()
    }
}

@Reducer
struct MessagesOverviewFeature {
    @ObservableState
    struct State: Equatable {
        @Presents var messageDetails: MessageDetailsFeature.State?

        var process: ProcessRoom
        var readMessageIds: Set<String> = []
        var showProcessDetails = false
        var selectedAttachmentURL: URL?
        var attachmentURLs: [URL] = []
        var isDownloadingAttachment = false
        var isContentLoading = true

        var dialogs: [NeoDialogProxy] = []
        var _previewEntries: [NeoDialogProxy]?

        var entries: [NeoDialogProxy] {
            let items = _previewEntries ?? dialogs
            return items.map { message in
                var msg = message
                msg.unread = !readMessageIds.contains(msg.id)
                return msg
            }
        }

        var effectiveDueDate: Date? {
            let messageDueDates = entries.compactMap(\.dueDate)
            return messageDueDates.min()
        }
    }

    enum Action {
        case onAppear
        case caseLoaded([NeoDialog])
        case toggleProcessDetails
        case selectMessage(NeoDialogProxy)
        case messageDetails(PresentationAction<MessageDetailsFeature.Action>)
        case previewAttachments([AttachmentType])
        case attachmentsDownloaded([URL])
        case dismissAttachmentPreview
        case onMessageUpdates(TimelineDiffs)
        case markAsRead(String)
        case replyNowTapped(NeoDialogProxy)
    }

    @Dependency(\.messageServiceClient) var messageServiceClient
    @Dependency(\.timelineClient) var timelineClient
    @Dependency(\.mediaServiceClient) var mediaServiceClient

    private enum CancelID { case timeline }

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .onAppear:
                let process = state.process
                return .run { send in
                    let _case = try await messageServiceClient.loadCase(process.id)
                    await send(.caseLoaded(_case.dialogs))

                    for await diffs in timelineClient.observe(process) {
                        await send(.onMessageUpdates(diffs))
                    }
                }
                .cancellable(id: CancelID.timeline, cancelInFlight: true)

            case let .caseLoaded(dialogs):
                state.dialogs = dialogs.reversed().map(NeoDialogProxy.init)
                state.isContentLoading = false
                if let presentedId = state.messageDetails?.dialog.id,
                   let updated = state.dialogs.first(where: { $0.id == presentedId }) {
                    let priorAnswerCount = state.messageDetails?.dialog.answers.count ?? 0
                    state.messageDetails?.dialog = updated
                    state.messageDetails?.replyFeature.answers = updated.answers
                    if updated.answers.count > priorAnswerCount,
                       state.messageDetails?.replyConfirmationFeature.submitStatus == .sending {
                        state.messageDetails?.replyConfirmationFeature.submitStatus = .sent
                    }
                }
                return .none

            case .toggleProcessDetails:
                state.showProcessDetails.toggle()
                return .none

            case let .selectMessage(entry):
                state.readMessageIds.insert(entry.id)
                state.messageDetails = MessageDetailsFeature.State(caseId: state.process.id,
                                                                   authority: state.process.authorityName,
                                                                   dialog: entry)
                return .none

            case .messageDetails:
                return .none

            case let .previewAttachments(attachments):
                guard !attachments.isEmpty else { return .none }
                state.isDownloadingAttachment = true
                return .run { send in
                    do {
                        let urls = try await withThrowingTaskGroup(of: (Int, URL).self) { group in
                            for (index, attachment) in attachments.enumerated() {
                                group.addTask { try await (index, mediaServiceClient.download(attachment)) }
                            }
                            var results = [(Int, URL)]()
                            for try await result in group {
                                results.append(result)
                            }
                            return results.sorted { $0.0 < $1.0 }.map(\.1)
                        }
                        await send(.attachmentsDownloaded(urls))
                    } catch {
                        await send(.dismissAttachmentPreview)
                    }
                }

            case let .attachmentsDownloaded(urls):
                state.isDownloadingAttachment = false
                state.attachmentURLs = urls
                state.selectedAttachmentURL = urls.first
                return .none

            case .dismissAttachmentPreview:
                state.isDownloadingAttachment = false
                state.selectedAttachmentURL = nil
                state.attachmentURLs = []
                return .none

            case let .markAsRead(eventId):
                state.readMessageIds.insert(eventId)
                return .none

            case .onMessageUpdates:
                let process = state.process
                state.isContentLoading = true

                return .run { send in
                    let _case = try await messageServiceClient.loadCase(process.id)
                    await send(.caseLoaded(_case.dialogs))
                }

            case .replyNowTapped(let entry):
                let replyState = ReplyFeature.State(isExpanded: true)
                state.readMessageIds.insert(entry.id)
                state.messageDetails = MessageDetailsFeature.State(caseId: state.process.id,
                                                                   authority: state.process.authorityName,
                                                                   dialog: entry,
                                                                   replyFeature: replyState)
                return .none
            }
        }
        .ifLet(\.$messageDetails, action: \.messageDetails) {
            MessageDetailsFeature()
        }
    }
}

struct MessagesOverviewInitialStateClient {
    var initialState: @Sendable (ProcessRoom) -> MessagesOverviewFeature.State
}

extension MessagesOverviewInitialStateClient: DependencyKey {
    static let liveValue = MessagesOverviewInitialStateClient { process in
        #if DEBUG
        if let scenarioName = ProcessInfo.processInfo.environment["UI_TEST_MESSAGES_SCENARIO"],
           let scenario = MessagesTestScenario(rawValue: scenarioName) {
            return scenario.makeState(process: process)
        }
        #endif
        return MessagesOverviewFeature.State(process: process)
    }

    static let testValue = MessagesOverviewInitialStateClient { process in
        MessagesOverviewFeature.State(process: process)
    }
}

extension DependencyValues {
    var messagesOverviewInitialState: MessagesOverviewInitialStateClient {
        get { self[MessagesOverviewInitialStateClient.self] }
        set { self[MessagesOverviewInitialStateClient.self] = newValue }
    }
}

extension MessagesOverviewFeature.State {
    static func initial(process: ProcessRoom) -> MessagesOverviewFeature.State {
        @Dependency(\.messagesOverviewInitialState) var client
        return client.initialState(process)
    }
}
