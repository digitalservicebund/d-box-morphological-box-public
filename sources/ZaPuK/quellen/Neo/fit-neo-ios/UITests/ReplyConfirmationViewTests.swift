//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class ReplyConfirmationViewTests: BaseUITestCase {
    private let replyText = "Ladies and gentlemen, here is my answer."

    private func expandReplyEditorAndType(_ text: String) {
        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0), "Reply toggle button not found")
        toggleButton.tap()

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0), "Reply text editor not found")
        textEditor.tap()
        textEditor.typeText(text)
    }

    private func tapSubmitToShowConfirmation() {
        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0), "Reply submit button not found")
        XCTAssertTrue(submitButton.isEnabled, "Submit button should be enabled before tapping")
        submitButton.tap()
    }

    @MainActor
    func testConfirmationDialogAppearsAfterSubmit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()
        expandReplyEditorAndType(replyText)
        tapSubmitToShowConfirmation()

        let title = app.staticTexts["ReplyConfirmationView.Title"]
        XCTAssertTrue(title.waitForExistence(timeout: 3.0), "Dialog title not visible")

        let confirmButton = app.buttons["ReplyConfirmationView.ConfirmButton"]
        XCTAssertTrue(confirmButton.exists, "Confirm button not visible")

        let cancelButton = app.buttons["ReplyConfirmationView.CancelButton"]
        XCTAssertTrue(cancelButton.exists, "Cancel button not visible")
    }

    @MainActor
    func testConfirmationDialogShowsTypedPreview() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()
        expandReplyEditorAndType(replyText)
        tapSubmitToShowConfirmation()

        let preview = app.staticTexts["ReplyConfirmationView.PreviewContent"]
        XCTAssertTrue(preview.waitForExistence(timeout: 3.0), "Preview content text not visible")
        XCTAssertEqual(preview.label, replyText, "Preview should match the typed reply content")
    }

    @MainActor
    func testCancelButtonDismissesDialogAndPreservesContent() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()
        expandReplyEditorAndType(replyText)
        tapSubmitToShowConfirmation()

        let cancelButton = app.buttons["ReplyConfirmationView.CancelButton"]
        XCTAssertTrue(cancelButton.waitForExistence(timeout: 3.0))
        cancelButton.tap()

        let title = app.staticTexts["ReplyConfirmationView.Title"]
        XCTAssertTrue(waitForElementToDisappear(title, timeout: 3.0),
                      "Confirmation dialog should be dismissed after cancel")

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        XCTAssertEqual(textEditor.value as? String, replyText,
                       "Reply content should be preserved after cancelling the confirmation")
    }

    @MainActor
    func testSubmitDisabledWhileConfirming() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()
        expandReplyEditorAndType(replyText)
        tapSubmitToShowConfirmation()

        let title = app.staticTexts["ReplyConfirmationView.Title"]
        XCTAssertTrue(title.waitForExistence(timeout: 3.0))

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertFalse(submitButton.isEnabled,
                       "Submit button should be disabled while confirmation dialog is shown")
    }

    @MainActor
    func testAccessibilityAudit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()
        expandReplyEditorAndType(replyText)
        tapSubmitToShowConfirmation()

        let title = app.staticTexts["ReplyConfirmationView.Title"]
        XCTAssertTrue(title.waitForExistence(timeout: 3.0), "Confirmation dialog did not appear")

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .dynamicType ||
                issue.auditType == .textClipped ||
                issue.auditType == .contrast {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }
}
