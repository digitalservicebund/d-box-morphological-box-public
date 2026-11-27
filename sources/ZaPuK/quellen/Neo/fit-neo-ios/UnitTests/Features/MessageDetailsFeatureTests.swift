//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class MessageDetailsFeatureTests: XCTestCase {
    private typealias F = NeoDialogFixtures

    private func makeDialog(id: String = "msg-1",
                            sender: String = "Central Office",
                            text: String = "Your application has been received.",
                            fileBlocks: [NeoBuildingBlock] = [],
                            answers: [NeoEnvelope] = []) -> NeoDialogProxy {
        let timestamp = F.ts()
        var blocks = [F.messageBlock(id: "\(id)-block", sender: sender, timestamp: timestamp, text: text)]
        blocks.append(contentsOf: fileBlocks)
        return F.dialog(id: id, timestamp: timestamp, blocks: blocks, answers: answers)
    }

    private func freeTextReply(_ text: String,
                               isConfirmingReply: Bool = false) -> ReplyFeature.State {
        var state = ReplyFeature.State(content: .inquiryMessage(title: NeoText(plain: "Title", html: nil),
                                                                label: NeoText(plain: "Label", html: nil)),
                                       isConfirmingReply: isConfirmingReply)
        state.editor = .freeText(FreeTextReplyFeature.State(textContent: text))
        return state
    }

    private func messageContent(_ text: String) -> [NeoContent] {
        [.message(text: NeoText(plain: text, html: nil), replyTo: nil)]
    }

    private func pollAnswers() -> [NeoPollAnswer] {
        [
            NeoPollAnswer(id: "1", text: NeoText(plain: "Less than 1 meter", html: nil)),
            NeoPollAnswer(id: "2", text: NeoText(plain: "Between 1 and 2 meters", html: nil)),
            NeoPollAnswer(id: SingleSelectReplyFeature.customOptionID,
                          text: NeoText(plain: "Something else", html: nil))
        ]
    }

    private func pollContent() -> NeoContent {
        .inquiryPoll(poll: NeoPollContent(multiSelect: false,
                                          question: NeoText(plain: "How tall was the dragon?", html: nil),
                                          answers: pollAnswers()))
    }

    private func pollReply(selectedOptionID: String = "",
                           comment: String = "",
                           isConfirmingReply: Bool = false) -> ReplyFeature.State {
        var state = ReplyFeature.State(content: pollContent(), isConfirmingReply: isConfirmingReply)
        state.editor = .singleSelect(SingleSelectReplyFeature.State(options: pollAnswers(),
                                                                    selectedOptionID: selectedOptionID,
                                                                    comment: comment))
        return state
    }

    func testToggleHeaderExpandsHeader() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog)
        ) {
            MessageDetailsFeature()
        }

        await store.send(.toggleHeader) {
            $0.expandedHeader = true
        }
    }

    func testToggleHeaderCollapsesHeader() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(expandedHeader: true,
                                                      caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog)
        ) {
            MessageDetailsFeature()
        }

        await store.send(.toggleHeader) {
            $0.expandedHeader = false
        }
    }

    func testDialogWithFileBlocksHasAttachments() {
        let timestamp = F.ts()
        let fileBlocks = [
            F.fileBlock(id: "file-1", sender: "Central Office", timestamp: timestamp,
                        name: "doc.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "file-2", sender: "Central Office", timestamp: timestamp,
                        name: "image.png", mimetype: "image/png", size: 1_024_000)
        ]
        let dialog = makeDialog(id: "msg-attach", sender: "Central Office",
                                text: "See attached", fileBlocks: fileBlocks)

        let attachments = dialog.message.attachments()
        XCTAssertEqual(attachments.count, 2)
        XCTAssertEqual(attachments[0].filename, "doc.pdf")
        XCTAssertEqual(attachments[1].filename, "image.png")
    }

    func testSubmitReplyCallsServiceAndStaysInSending() async {
        let dialog = makeDialog()
        let captured = LockIsolated<(sent: Bool, caseId: String, dialogId: String, text: [NeoContent])>(
            (sent: false, caseId: "", dialogId: "", text: [])
        )
        let clock = TestClock()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("My reply"))
        ) {
            MessageDetailsFeature()
        } withDependencies: {
            $0.continuousClock = clock
            $0.messageServiceClient.sendAnswer = { caseId, dialogId, text in
                captured.withValue {
                    $0 = (sent: true, caseId: caseId, dialogId: dialogId, text: text)
                }
                return "event-id-456"
            }
        }

        await store.send(.replyAction(.submitReply)) {
            $0.replyFeature.isConfirmingReply = true
            $0.replyConfirmationFeature.inquiryContent = $0.replyFeature.content
            $0.replyConfirmationFeature.previewContent = self.messageContent("My reply")
            $0.replyConfirmationFeature.submitStatus = .unsent
        }

        await store.send(.replyConfirmationAction(.confirmSend)) {
            $0.replyConfirmationFeature.submitStatus = .sending
        }

        XCTAssertEqual(store.state.replyConfirmationFeature.submitStatus, .sending)

        let result = captured.value
        XCTAssertTrue(result.sent)
        XCTAssertEqual(result.caseId, "!test-room:example.org")
        XCTAssertEqual(result.dialogId, dialog.id)
        XCTAssertEqual(result.text, messageContent("My reply"))

        // Drain the in-flight timeout effect so the test exits cleanly.
        await clock.advance(by: MessageDetailsFeature.replyTimeout)
        await store.receive(\.replyFailed) {
            $0.replyConfirmationFeature.submitStatus = .failed
            $0.replyFeature.isConfirmingReply = false
        }
    }

    func testSubmitReplyFailurePreservesContent() async {
        let dialog = makeDialog()
        let clock = TestClock()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("My reply"))
        ) {
            MessageDetailsFeature()
        } withDependencies: {
            $0.continuousClock = clock
            $0.messageServiceClient.sendAnswer = { _, _, _ in
                throw MessageServiceError.invalidContent
            }
        }

        await store.send(.replyAction(.submitReply)) {
            $0.replyFeature.isConfirmingReply = true
            $0.replyConfirmationFeature.inquiryContent = $0.replyFeature.content
            $0.replyConfirmationFeature.previewContent = self.messageContent("My reply")
            $0.replyConfirmationFeature.submitStatus = .unsent
        }

        await store.send(.replyConfirmationAction(.confirmSend)) {
            $0.replyConfirmationFeature.submitStatus = .sending
        }

        await store.receive(\.replyFailed) {
            $0.replyConfirmationFeature.submitStatus = .failed
            $0.replyFeature.isConfirmingReply = false
        }

        XCTAssertEqual(store.state.replyFeature.replyContent, messageContent("My reply"))

        await clock.advance(by: MessageDetailsFeature.replyTimeout)
    }

    func testSubmitPollReplyCallsServiceWithPollContent() async {
        let dialog = makeDialog()
        let captured = LockIsolated<(sent: Bool, caseId: String, dialogId: String, text: [NeoContent])>(
            (sent: false, caseId: "", dialogId: "", text: [])
        )
        let clock = TestClock()
        let pollReplyContent: [NeoContent] = [.poll(selections: ["2"], custom: nil, replyTo: nil)]

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: pollReply(selectedOptionID: "2"))
        ) {
            MessageDetailsFeature()
        } withDependencies: {
            $0.continuousClock = clock
            $0.messageServiceClient.sendAnswer = { caseId, dialogId, text in
                captured.withValue {
                    $0 = (sent: true, caseId: caseId, dialogId: dialogId, text: text)
                }
                return "event-id-789"
            }
        }

        await store.send(.replyAction(.submitReply)) {
            $0.replyFeature.isConfirmingReply = true
            $0.replyConfirmationFeature.inquiryContent = $0.replyFeature.content
            $0.replyConfirmationFeature.previewContent = pollReplyContent
            $0.replyConfirmationFeature.submitStatus = .unsent
        }

        await store.send(.replyConfirmationAction(.confirmSend)) {
            $0.replyConfirmationFeature.submitStatus = .sending
        }

        let result = captured.value
        XCTAssertTrue(result.sent)
        XCTAssertEqual(result.caseId, "!test-room:example.org")
        XCTAssertEqual(result.dialogId, dialog.id)
        XCTAssertEqual(result.text, pollReplyContent,
                       "Service should receive the selected poll answer as poll content")

        await clock.advance(by: MessageDetailsFeature.replyTimeout)
        await store.receive(\.replyFailed) {
            $0.replyConfirmationFeature.submitStatus = .failed
            $0.replyFeature.isConfirmingReply = false
        }
    }

    func testSubmitCustomOptionSendsCustomComment() async {
        let dialog = makeDialog()
        let captured = LockIsolated<[NeoContent]>([])
        let clock = TestClock()
        let customContent: [NeoContent] = [.poll(selections: [], custom: "My own answer", replyTo: nil)]

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: pollReply(selectedOptionID: SingleSelectReplyFeature.customOptionID,
                                                                              comment: "My own answer"))
        ) {
            MessageDetailsFeature()
        } withDependencies: {
            $0.continuousClock = clock
            $0.messageServiceClient.sendAnswer = { _, _, text in
                captured.withValue { $0 = text }
                return "event-id-custom"
            }
        }

        await store.send(.replyAction(.submitReply)) {
            $0.replyFeature.isConfirmingReply = true
            $0.replyConfirmationFeature.inquiryContent = $0.replyFeature.content
            $0.replyConfirmationFeature.previewContent = customContent
            $0.replyConfirmationFeature.submitStatus = .unsent
        }

        await store.send(.replyConfirmationAction(.confirmSend)) {
            $0.replyConfirmationFeature.submitStatus = .sending
        }

        XCTAssertEqual(captured.value, customContent,
                       "A custom poll answer should be sent as poll content with a custom comment")

        await clock.advance(by: MessageDetailsFeature.replyTimeout)
        await store.receive(\.replyFailed) {
            $0.replyConfirmationFeature.submitStatus = .failed
            $0.replyFeature.isConfirmingReply = false
        }
    }

    func testSendSuccessResetsConfirmationAndEditor() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: pollReply(selectedOptionID: "2", isConfirmingReply: true),
                                                      replyConfirmationFeature: ReplyConfirmationFeature.State(previewContent: [.poll(selections: ["2"], custom: nil, replyTo: nil)],
                                                                                                               submitStatus: .sending))
        ) {
            MessageDetailsFeature()
        }

        XCTAssertEqual(store.state.replyFeature.replyContent,
                       [.poll(selections: ["2"], custom: nil, replyTo: nil)],
                       "Precondition: a poll option is selected before the send succeeds")

        await store.send(.replyConfirmationAction(.sendSuccess)) {
            $0.replyConfirmationFeature.submitStatus = .sent
            $0.replyFeature.isConfirmingReply = false
            $0.replyFeature.editor = ReplyEditor.State.make(for: $0.replyFeature.content)
        }

        XCTAssertTrue(store.state.replyFeature.replyContent.isEmpty,
                      "Editor should be reset (selection cleared) after a successful send")
    }

    func testSubmitReplyWhileConfirmingIsIgnored() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("text",
                                                                                  isConfirmingReply: true))
        ) {
            MessageDetailsFeature()
        }

        await store.send(.replyAction(.submitReply))
    }

    func testCancelSendDismissesConfirmation() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("My reply", isConfirmingReply: true),
                                                      replyConfirmationFeature: ReplyConfirmationFeature.State(previewContent: messageContent("My reply")))
        ) {
            MessageDetailsFeature()
        }

        await store.send(.replyConfirmationAction(.cancelSend)) {
            $0.replyFeature.isConfirmingReply = false
            $0.replyConfirmationFeature.submitStatus = .unsent
        }

        XCTAssertEqual(store.state.replyFeature.replyContent, messageContent("My reply"))
    }

    func testSubmitReplyWithWhitespaceOnlyIsIgnored() async {
        let dialog = makeDialog()

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("   \n  "))
        ) {
            MessageDetailsFeature()
        }

        await store.send(.replyAction(.submitReply))
    }

    func testOnAppearPropagatesPollContentToReplyFeature() async {
        let timestamp = F.ts()
        let pollBlock = F.pollBlock(id: "poll-block",
                                    sender: "Central Licensing Office",
                                    timestamp: timestamp,
                                    multiSelect: false,
                                    question: "How tall was the dragon?",
                                    answers: [
                                        "1": "Less than 1 meter",
                                        "2": "Between 1 and 2 meters"
                                    ])
        let dialog = F.dialog(id: "poll-dialog", timestamp: timestamp, blocks: [pollBlock])

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog)
        ) {
            MessageDetailsFeature()
        }

        await store.send(.onAppear) {
            $0.replyFeature.content = dialog.inquiryContent
            $0.replyFeature.editor = ReplyEditor.State.make(for: dialog.inquiryContent)
            $0.replyFeature.dueDate = dialog.dueDate
            $0.replyFeature.answers = dialog.answers
        }

        guard case .inquiryPoll(let poll) = store.state.replyFeature.content else {
            XCTFail("Expected replyFeature.content to be an inquiryPoll")
            return
        }
        XCTAssertFalse(poll.multiSelect)
        XCTAssertEqual(poll.answers.count, 2)
        XCTAssertEqual(poll.question.plain, "How tall was the dragon?")

        if case .singleSelect = store.state.replyFeature.editor {} else {
            XCTFail("Expected a single-select editor for poll content")
        }
    }

    func testOnAppearPropagatesInquiryMessageContentToReplyFeature() async {
        let timestamp = F.ts()
        let inquiry = F.inquiryBlock(id: "inquiry-block",
                                     sender: "Central Licensing Office",
                                     timestamp: timestamp,
                                     title: "Application Confirmation",
                                     label: "Please confirm.")
        let dialog = F.dialog(id: "inquiry-dialog", timestamp: timestamp, blocks: [inquiry])

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog)
        ) {
            MessageDetailsFeature()
        }

        await store.send(.onAppear) {
            $0.replyFeature.content = dialog.inquiryContent
            $0.replyFeature.editor = ReplyEditor.State.make(for: dialog.inquiryContent)
            $0.replyFeature.dueDate = dialog.dueDate
            $0.replyFeature.answers = dialog.answers
        }

        guard case .inquiryMessage = store.state.replyFeature.content else {
            XCTFail("Expected replyFeature.content to be an inquiryMessage")
            return
        }

        if case .freeText = store.state.replyFeature.editor {} else {
            XCTFail("Expected a free-text editor for inquiry message content")
        }
    }

    func testOnAppearDoesNotOverwriteExistingReplyContent() async {
        let timestamp = F.ts()
        let pollBlock = F.pollBlock(id: "poll-block",
                                    sender: "Central Licensing Office",
                                    timestamp: timestamp,
                                    multiSelect: false,
                                    question: "How tall was the dragon?",
                                    answers: [
                                        "1": "Less than 1 meter",
                                        "2": "Between 1 and 2 meters"
                                    ])
        let dialog = F.dialog(id: "poll-dialog", timestamp: timestamp, blocks: [pollBlock])

        let store = TestStore(
            initialState: MessageDetailsFeature.State(caseId: "!test-room:example.org",
                                                      authority: "Test Authority",
                                                      dialog: dialog,
                                                      replyFeature: freeTextReply("Preserved comment"))
        ) {
            MessageDetailsFeature()
        }

        // dialog has no dueDate/answers and existing content is non-nil, so onAppear is a no-op.
        await store.send(.onAppear)

        guard case .inquiryMessage = store.state.replyFeature.content else {
            XCTFail("Existing replyFeature.content should not be overwritten on re-appear")
            return
        }
        XCTAssertEqual(store.state.replyFeature.replyContent, messageContent("Preserved comment"),
                       "User-entered reply content should be preserved on re-appear")
    }
}
