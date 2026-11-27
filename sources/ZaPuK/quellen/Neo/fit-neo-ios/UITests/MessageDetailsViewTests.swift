//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class MessageDetailsViewTests: BaseUITestCase {
    @MainActor
    func testNavigateToMessageDetails() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0), "Message details view did not appear")
    }

    @MainActor
    func testMessageSenderDisplayed() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let senderText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS %@", "City Hall"))
        XCTAssertTrue(senderText.firstMatch.waitForExistence(timeout: 5.0), "Sender name not displayed")
    }

    @MainActor
    func testMessageContentDisplayed() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let contentText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS %@", "application has been received"))
        XCTAssertTrue(contentText.firstMatch.waitForExistence(timeout: 5.0), "Message content not displayed")
    }

    @MainActor
    func testInformativeMessageViewDisplayed() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let informativeView = app.descendants(matching: .any).matching(identifier: "InquiryMessageView")
        XCTAssertTrue(informativeView.firstMatch.waitForExistence(timeout: 5.0),
                      "Informative message view not displayed")
    }

    @MainActor
    func testToggleHeaderButton() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let toggleButton = app.buttons["MessageDetailsView.ToggleHeader"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5.0), "Toggle header button not found")

        toggleButton.tap()

        let expandedContent = app.staticTexts.containing(NSPredicate(format: "label CONTAINS %@", "Öffnungszeiten"))
        XCTAssertTrue(expandedContent.firstMatch.waitForExistence(timeout: 2.0),
                      "Header details not shown after expanding")
    }

    @MainActor
    func testMessageDetailsViewDisplaysAttachmentSection() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessageDetails(messageTitle: "Documents Required")

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        let attachmentsTitle = app.staticTexts["Attachments"]
        XCTAssertTrue(attachmentsTitle.waitForExistence(timeout: 5.0), "Attachments section title not displayed")
    }

    @MainActor
    func testAccessibilityAudit() throws {
        setMessagesScenario("singleMessage")
        launchAndNavigateToMessageDetails()

        let detailsScrollView = app.scrollViews["MessageDetailsView.ScrollView"]
        XCTAssertTrue(detailsScrollView.waitForExistence(timeout: 5.0))

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .textClipped {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }
}
