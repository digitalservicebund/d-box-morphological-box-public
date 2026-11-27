//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class MessagesOverviewFeatureTests: XCTestCase {
    private typealias F = NeoDialogFixtures

    private func makeProcess(id: String = "process-1",
                             name: String = "Test Process",
                             authority: String = "Test Authority") -> ProcessRoom {
        ProcessRoom(id: id, name: name, authority: authority)
    }

    private func makeDialog(id: String = "msg-1",
                            sender: String = "sender",
                            text: String = "Hello",
                            timestamp: UInt64? = nil,
                            answers: [NeoEnvelope] = []) -> NeoDialogProxy {
        let ts = timestamp ?? F.ts()
        return F.dialog(id: id, timestamp: ts, blocks: [
            F.messageBlock(id: "\(id)-block", sender: sender, timestamp: ts, text: text)
        ], answers: answers)
    }

    private func neoDialog(id: String,
                           sender: String = "Central Office",
                           text: String = "Your application has been received.",
                           answers: [NeoEnvelope] = []) -> NeoDialog {
        let ts = F.ts()
        return NeoDialog(id: id,
                         message: NeoEnvelope(id: "\(id)-env",
                                              timestamp: ts,
                                              blocks: [F.messageBlock(id: "\(id)-block",
                                                                      sender: sender,
                                                                      timestamp: ts,
                                                                      text: text)]),
                         answers: answers)
    }

    private func answerEnvelope(id: String = "answer-1",
                                text: String = "Between 1 and 2 meters") -> NeoEnvelope {
        let ts = F.ts()
        return F.envelope(id: id, timestamp: ts, blocks: [
            F.messageBlock(id: "\(id)-block", sender: "citizen", timestamp: ts, text: text)
        ])
    }

    private func presentedDetails(for dialog: NeoDialogProxy,
                                  process: ProcessRoom,
                                  submitStatus: SubmitReplyStatus) -> MessageDetailsFeature.State {
        var details = MessageDetailsFeature.State(caseId: process.id,
                                                  authority: process.authorityName,
                                                  dialog: dialog)
        details.replyConfirmationFeature.submitStatus = submitStatus
        return details
    }

    func testCaseLoadedPopulatesDialogsAndStopsLoading() async {
        let first = neoDialog(id: "msg-1", text: "First")
        let second = neoDialog(id: "msg-2", text: "Second")

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: makeProcess())
        ) {
            MessagesOverviewFeature()
        }

        await store.send(.caseLoaded([first, second])) {
            $0.dialogs = [NeoDialogProxy(second), NeoDialogProxy(first)]
            $0.isContentLoading = false
        }
    }

    func testCaseLoadedShowsAnswerAfterSend() async {
        let process = makeProcess()
        let initialDialog = NeoDialogProxy(neoDialog(id: "msg-1"))

        let answer = answerEnvelope()
        let loaded = neoDialog(id: "msg-1", answers: [answer])
        let updatedDialog = NeoDialogProxy(loaded)

        var state = MessagesOverviewFeature.State(process: process)
        state.messageDetails = presentedDetails(for: initialDialog, process: process, submitStatus: .sending)

        let store = TestStore(initialState: state) {
            MessagesOverviewFeature()
        }

        await store.send(.caseLoaded([loaded])) {
            $0.dialogs = [updatedDialog]
            $0.isContentLoading = false
            $0.messageDetails?.dialog = updatedDialog
            $0.messageDetails?.replyFeature.answers = [answer]
            $0.messageDetails?.replyConfirmationFeature.submitStatus = .sent
        }
    }

    func testInitialState() {
        let process = makeProcess()

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: process)
        ) {
            MessagesOverviewFeature()
        }

        XCTAssertEqual(store.state.process, process)
        XCTAssertFalse(store.state.showProcessDetails)
        XCTAssertNil(store.state.messageDetails)
        XCTAssertNil(store.state.selectedAttachmentURL)
        XCTAssertEqual(store.state.attachmentURLs, [])
        XCTAssertFalse(store.state.isDownloadingAttachment)
        XCTAssertTrue(store.state.entries.isEmpty)
        XCTAssertTrue(store.state.isContentLoading)
        XCTAssertNil(store.state.effectiveDueDate)
    }

    func testToggleProcessDetails() async {
        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: makeProcess())
        ) {
            MessagesOverviewFeature()
        }

        await store.send(.toggleProcessDetails) {
            $0.showProcessDetails = true
        }

        await store.send(.toggleProcessDetails) {
            $0.showProcessDetails = false
        }
    }

    func testSelectMessagePresentsMessageDetails() async {
        let dialog = makeDialog(id: "msg-1", sender: "alice", text: "Hello")
        let process = makeProcess()

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: process,
                                                        _previewEntries: [dialog])
        ) {
            MessagesOverviewFeature()
        }

        await store.send(.selectMessage(dialog)) {
            $0.readMessageIds.insert("msg-1")
            $0.messageDetails = MessageDetailsFeature.State(caseId: process.id,
                                                            authority: process.authorityName,
                                                            dialog: dialog)
        }

        await store.send(.messageDetails(.dismiss)) {
            $0.messageDetails = nil
        }
    }

    func testPreviewAttachmentsDownloadsAllAndSetsURLs() async {
        let attachments = [
            AttachmentType(url: "/path/to/file1.pdf", filename: "file1.pdf", filesize: 0),
            AttachmentType(url: "/path/to/file2.png", filename: "file2.png", filesize: 0)
        ]
        let expectedURLs = attachments.map { URL(fileURLWithPath: $0.url) }

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: makeProcess())
        ) {
            MessagesOverviewFeature()
        } withDependencies: {
            $0.mediaServiceClient.download = { attachment in URL(fileURLWithPath: attachment.url) }
        }

        await store.send(.previewAttachments(attachments)) {
            $0.isDownloadingAttachment = true
        }

        await store.receive(\.attachmentsDownloaded) {
            $0.isDownloadingAttachment = false
            $0.attachmentURLs = expectedURLs
            $0.selectedAttachmentURL = expectedURLs.first
        }
    }

    func testPreviewThenDismissAttachments() async {
        let attachments = [
            AttachmentType(url: "/path/to/document.pdf", filename: "document.pdf", filesize: 0)
        ]
        let expectedURLs = attachments.map { URL(fileURLWithPath: $0.url) }

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: makeProcess())
        ) {
            MessagesOverviewFeature()
        } withDependencies: {
            $0.mediaServiceClient.download = { attachment in URL(fileURLWithPath: attachment.url) }
        }

        await store.send(.previewAttachments(attachments)) {
            $0.isDownloadingAttachment = true
        }

        await store.receive(\.attachmentsDownloaded) {
            $0.isDownloadingAttachment = false
            $0.attachmentURLs = expectedURLs
            $0.selectedAttachmentURL = expectedURLs.first
        }

        await store.send(.dismissAttachmentPreview) {
            $0.isDownloadingAttachment = false
            $0.selectedAttachmentURL = nil
            $0.attachmentURLs = []
        }
    }

    func testEntriesUsesPreviewEntriesWhenSet() {
        let previewEntries = [
            makeDialog(id: "1", sender: "alice", text: "First message"),
            makeDialog(id: "2", sender: "bob", text: "Second message")
        ]

        let state = MessagesOverviewFeature.State(process: makeProcess(),
                                                  _previewEntries: previewEntries)

        XCTAssertEqual(state.entries.count, 2)
    }

    func testInitialStateWithEntries() {
        let process = makeProcess(id: "process-42", name: "Permit Application", authority: "City Hall")
        let entries = [
            makeDialog(id: "m1", sender: "authority", text: "Your permit is approved"),
            makeDialog(id: "m2", sender: "citizen", text: "Thank you")
        ]

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: process,
                                                        showProcessDetails: true,
                                                        _previewEntries: entries)
        ) {
            MessagesOverviewFeature()
        }

        XCTAssertEqual(store.state.process.id, "process-42")
        XCTAssertEqual(store.state.process.processName, "Permit Application")
        XCTAssertTrue(store.state.showProcessDetails)
        XCTAssertEqual(store.state.entries.count, 2)
    }

    func testDialogWithAnswersAndFileBlocks() {
        let timestamp = F.ts()
        let replyEnvelope = F.envelope(id: "reply-1-env", timestamp: timestamp, blocks: [
            F.messageBlock(id: "reply-1-block", sender: "bob", timestamp: timestamp, text: "Reply text")
        ])
        let dialog = F.dialog(id: "msg-1", timestamp: timestamp, blocks: [
            F.messageBlock(id: "msg-1-block", sender: "alice", timestamp: timestamp, text: "See attached"),
            F.fileBlock(id: "msg-1-file", sender: "alice", timestamp: timestamp,
                        name: "file.pdf", mimetype: "application/pdf", size: 0)
        ], answers: [replyEnvelope])

        XCTAssertFalse(dialog.answers.isEmpty)
        let attachments = dialog.message.attachments()
        XCTAssertEqual(attachments.count, 1)
        XCTAssertEqual(attachments.first?.filename, "file.pdf")
    }

    func testMessagesMarkAsRead() async {
        let dialogs = [
            makeDialog(id: "1", sender: "alice", text: "First"),
            makeDialog(id: "2", sender: "bob", text: "Second"),
            makeDialog(id: "3", sender: "carol", text: "Third")
        ]

        let store = TestStore(
            initialState: MessagesOverviewFeature.State(process: makeProcess(),
                                                        _previewEntries: dialogs)
        ) {
            MessagesOverviewFeature()
        }

        await store.send(.markAsRead("1")) {
            $0.readMessageIds.insert("1")
        }

        XCTAssertFalse(store.state.entries.first { $0.id == "1" }!.unread,
                       "First message should be read")
        XCTAssertTrue(store.state.entries.first { $0.id == "2" }!.unread,
                      "Second message should be unread")
        XCTAssertTrue(store.state.entries.first { $0.id == "3" }!.unread,
                      "Third message should be unread")
    }

    func testEffectiveDueDateIsNilWhenNoDialogsHaveDueDates() {
        let dialogs = [
            makeDialog(id: "1", sender: "alice", text: "No due date"),
            makeDialog(id: "2", sender: "bob", text: "Also no due date")
        ]

        let state = MessagesOverviewFeature.State(process: makeProcess(),
                                                  _previewEntries: dialogs)

        XCTAssertNil(state.effectiveDueDate,
                     "effectiveDueDate should be nil when no dialogs have due dates")
    }

    func testEffectiveDueDateWithSingleDialog() {
        let dueDate = Date.now.addingTimeInterval(60 * 60 * 24)
        var dialog = makeDialog(id: "1", sender: "alice", text: "Action needed")
        dialog.dueDate = dueDate

        let state = MessagesOverviewFeature.State(process: makeProcess(),
                                                  _previewEntries: [dialog])

        XCTAssertEqual(state.effectiveDueDate, dueDate)
    }

    func testEffectiveDueDateReturnsEarliestDueDate() {
        let earlierDate = Date.now.addingTimeInterval(60 * 60 * 24)
        let laterDate = Date.now.addingTimeInterval(10 * 60 * 60 * 24)

        var dialog1 = makeDialog(id: "1", sender: "alice", text: "Later deadline")
        dialog1.dueDate = laterDate

        var dialog2 = makeDialog(id: "2", sender: "bob", text: "Earlier deadline")
        dialog2.dueDate = earlierDate

        let dialog3 = makeDialog(id: "3", sender: "carol", text: "No deadline")

        let state = MessagesOverviewFeature.State(process: makeProcess(),
                                                  _previewEntries: [dialog1, dialog2, dialog3])

        XCTAssertEqual(state.effectiveDueDate, earlierDate,
                       "effectiveDueDate should return the earliest due date")
    }
}
