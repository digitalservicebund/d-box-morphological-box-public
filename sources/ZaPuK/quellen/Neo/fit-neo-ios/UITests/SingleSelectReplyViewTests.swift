//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class SingleSelectReplyViewTests: BaseUITestCase {
    @MainActor
    func testPollEditorVisibleAfterExpanding() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0), "Reply toggle button not found")
        toggleButton.tap()

        let editor = app.scrollViews["SingleSelectReplyEditor"]
        XCTAssertTrue(editor.waitForExistence(timeout: 3.0), "Single-select poll editor not shown after expanding")
    }

    @MainActor
    func testPollOptionsDisplayed() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let firstOption = app.buttons["SingleSelectReplyEditor.Option.1"]
        let secondOption = app.buttons["SingleSelectReplyEditor.Option.2"]
        let thirdOption = app.buttons["SingleSelectReplyEditor.Option.3"]
        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]

        XCTAssertTrue(firstOption.waitForExistence(timeout: 3.0), "Poll option 1 not displayed")
        XCTAssertTrue(secondOption.exists, "Poll option 2 not displayed")
        XCTAssertTrue(thirdOption.exists, "Poll option 3 not displayed")
        XCTAssertTrue(customOption.exists, "Custom option not displayed")
    }

    @MainActor
    func testSubmitButtonDisabledUntilOptionSelected() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertFalse(submitButton.isEnabled, "Submit button should be disabled before any option is selected")
    }

    @MainActor
    func testSelectingOptionEnablesSubmit() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let secondOption = app.buttons["SingleSelectReplyEditor.Option.2"]
        XCTAssertTrue(secondOption.waitForExistence(timeout: 3.0))
        secondOption.tap()

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(submitButton.isEnabled, "Submit button should be enabled after selecting an option")
        XCTAssertTrue(secondOption.isSelected, "Selected option should expose the isSelected accessibility trait")
    }

    @MainActor
    func testSelectionIsMutuallyExclusive() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let firstOption = app.buttons["SingleSelectReplyEditor.Option.1"]
        let thirdOption = app.buttons["SingleSelectReplyEditor.Option.3"]
        XCTAssertTrue(firstOption.waitForExistence(timeout: 3.0))
        XCTAssertTrue(thirdOption.exists)

        firstOption.tap()
        XCTAssertTrue(firstOption.isSelected, "First option should be selected after tap")
        XCTAssertFalse(thirdOption.isSelected, "Third option should not be selected after tapping the first option")

        thirdOption.tap()
        XCTAssertTrue(thirdOption.isSelected, "Third option should be selected after tap")
        XCTAssertFalse(firstOption.isSelected,
                       "First option should be deselected when switching to a different option")
    }

    @MainActor
    func testPollQuestionDisplayed() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let questionText = app.staticTexts
            .containing(NSPredicate(format: "label CONTAINS %@", "How tall was the dragon"))
        XCTAssertTrue(questionText.firstMatch.waitForExistence(timeout: 5.0),
                      "Poll question not displayed on message details")
    }

    @MainActor
    func testSelectingCustomtOptionShowsTextEditor() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]
        XCTAssertTrue(customOption.waitForExistence(timeout: 3.0))
        customOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0),
                      "Text comment editor should appear after selecting the comment option")
    }

    @MainActor
    func testRegularOptionDoesNotShowTextEditor() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let secondOption = app.buttons["SingleSelectReplyEditor.Option.2"]
        XCTAssertTrue(secondOption.waitForExistence(timeout: 3.0))
        secondOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertFalse(textEditor.waitForExistence(timeout: 1.0),
                       "Text comment editor should not be shown when a predefined option is selected")
    }

    @MainActor
    func testSwitchingToCustomOptionDisablesSubmit() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let secondOption = app.buttons["SingleSelectReplyEditor.Option.2"]
        XCTAssertTrue(secondOption.waitForExistence(timeout: 3.0))
        secondOption.tap()

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: true),
                      "Submit button should be enabled after selecting a predefined answer")

        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]
        XCTAssertTrue(customOption.waitForExistence(timeout: 3.0))
        customOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: false),
                      "Submit button should be disabled until a comment is entered")
    }

    @MainActor
    func testTypingCustomEnablesSubmit() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]
        XCTAssertTrue(customOption.waitForExistence(timeout: 3.0))
        customOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        textEditor.tap()
        textEditor.typeText("This is my free text comment")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: true),
                      "Submit button should be enabled after entering a comment")
    }

    @MainActor
    func testWhitespaceCustomKeepsSubmitDisabled() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]
        XCTAssertTrue(customOption.waitForExistence(timeout: 3.0))
        customOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        textEditor.tap()
        textEditor.typeText("     ")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: false),
                      "Submit button should stay disabled for a whitespace-only comment")
    }

    @MainActor
    func testSwitchingFromCommentToOptionHidesEditor() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let customOption = app.buttons["SingleSelectReplyEditor.Option.dev.fitko.neo.poll.custom.v1"]
        XCTAssertTrue(customOption.waitForExistence(timeout: 3.0))
        customOption.tap()

        let textEditor = app.textViews["SingleSelectReplyEditor.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        textEditor.tap()
        textEditor.typeText("Some comment text")

        let firstOption = app.buttons["SingleSelectReplyEditor.Option.1"]
        XCTAssertTrue(firstOption.waitForExistence(timeout: 3.0))
        firstOption.tap()

        XCTAssertTrue(waitForElementToDisappear(textEditor, timeout: 3.0),
                      "Text comment editor should be hidden after selecting a predefined option")
        XCTAssertTrue(firstOption.isSelected, "Selected predefined option should expose the isSelected trait")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: true),
                      "Submit button should be enabled after selecting a predefined option")
    }

    @MainActor
    func testSubmitPollShowsConfirmation() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        expandPollEditor()

        let secondOption = app.buttons["SingleSelectReplyEditor.Option.2"]
        XCTAssertTrue(secondOption.waitForExistence(timeout: 3.0))
        secondOption.tap()

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(waitForEnabled(submitButton, enabled: true))
        submitButton.tap()

        let title = app.staticTexts["ReplyConfirmationView.Title"]
        XCTAssertTrue(title.waitForExistence(timeout: 3.0), "Confirmation dialog did not appear for a poll answer")

        let preview = app.staticTexts["ReplyConfirmationView.PreviewContent"]
        XCTAssertTrue(preview.waitForExistence(timeout: 3.0), "Confirmation preview not visible")
        XCTAssertEqual(preview.label, "Between 1 and 2 meters",
                       "Confirmation preview should show the selected poll answer")
    }

    @MainActor
    func testAnsweredPollShowsSubmittedSelection() throws {
        setMessagesScenario("singlePollMessageAnswered")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")

        let answersList = app.scrollViews["FreeTextReplyView.AnswersList"]
        XCTAssertTrue(answersList.waitForExistence(timeout: 5.0),
                      "Submitted answer view should be shown for an already-answered poll")

        let selectedAnswer = app.descendants(matching: .any)
            .matching(NSPredicate(format: "label == %@ AND selected == true", "Between 1 and 2 meters"))
            .firstMatch
        XCTAssertTrue(selectedAnswer.waitForExistence(timeout: 5.0),
                      "The previously submitted poll selection should be displayed as selected")
    }

    @MainActor
    func testAccessibilityAudit() throws {
        setMessagesScenario("singlePollMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let editor = app.scrollViews["SingleSelectReplyEditor"]
        XCTAssertTrue(editor.waitForExistence(timeout: 3.0))

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .dynamicType ||
                issue.auditType == .contrast ||
                issue.auditType == .textClipped {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }

    @MainActor
    private func expandPollEditor() {
        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0), "Reply toggle button not found")
        toggleButton.tap()

        let editor = app.scrollViews["SingleSelectReplyEditor"]
        XCTAssertTrue(editor.waitForExistence(timeout: 3.0), "Single-select poll editor not shown after expanding")
    }

    @MainActor
    private func waitForEnabled(_ element: XCUIElement, enabled: Bool, timeout: TimeInterval = 3.0) -> Bool {
        let predicate = NSPredicate(format: "isEnabled == %@", NSNumber(value: enabled))
        let expectation = XCTNSPredicateExpectation(predicate: predicate, object: element)
        return XCTWaiter().wait(for: [expectation], timeout: timeout) == .completed
    }
}
