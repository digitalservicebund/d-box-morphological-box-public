//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class FreeTextReplyViewTests: BaseUITestCase {
    @MainActor
    func testReplyOverlayVisibleOnMessageDetails() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0), "Reply toggle button not found")

        let title = app.staticTexts["FreeTextReplyView.Title"]
        XCTAssertTrue(title.exists, "Reply title not found")
    }

    @MainActor
    func testExpandShowsTextEditorAndSubmit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0), "TextEditor not shown after expanding")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0), "Submit button not shown after expanding")
    }

    @MainActor
    func testCollapseHidesTextEditorAndSubmit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))

        // Expand
        toggleButton.tap()
        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))

        // Collapse
        toggleButton.tap()
        sleep(1) // wait for animation
        XCTAssertFalse(textEditor.exists, "TextEditor should be hidden after collapsing")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertFalse(submitButton.exists, "Submit button should be hidden after collapsing")
    }

    @MainActor
    func testTemplateButtonVisibleWhenEmpty() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let templateButton = app.buttons["FreeTextReplyView.TemplateButton"]
        XCTAssertTrue(templateButton.waitForExistence(timeout: 3.0), "Template button should be visible when content is empty")
    }

    @MainActor
    func testClearButtonAppearsAfterTyping() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        textEditor.tap()
        textEditor.typeText("Test reply content")

        let clearButton = app.buttons["FreeTextReplyView.ClearButton"]
        XCTAssertTrue(clearButton.waitForExistence(timeout: 3.0), "Clear button should appear after typing")

        let templateButton = app.buttons["FreeTextReplyView.TemplateButton"]
        XCTAssertFalse(templateButton.exists, "Template button should be hidden when content is present")
    }

    @MainActor
    func testSubmitButtonDisabledWhenEmpty() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertFalse(submitButton.isEnabled, "Submit button should be disabled when reply is empty")
    }

    @MainActor
    func testSubmitButtonEnabledAfterTyping() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))
        textEditor.tap()
        textEditor.typeText("My reply")

        let submitButton = app.buttons["FreeTextReplyView.SubmitButton"]
        XCTAssertTrue(submitButton.waitForExistence(timeout: 3.0))
        XCTAssertTrue(submitButton.isEnabled, "Submit button should be enabled after typing content")
    }

    @MainActor
    func testAccessibilityAudit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        // Expand the reply overlay so all elements are visible for the audit
        let toggleButton = app.buttons["FreeTextReplyView.ToggleButton"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0))
        toggleButton.tap()

        let textEditor = app.textViews["FreeTextReplyView.TextEditor"]
        XCTAssertTrue(textEditor.waitForExistence(timeout: 3.0))

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
}
