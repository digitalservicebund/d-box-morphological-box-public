//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class MessagesOverviewViewTests: BaseUITestCase {
    @MainActor
    func testEmptyStateDisplayed() throws {
        setMessagesScenario("empty")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0), "Messages scroll view did not appear")

        let emptyState = app.otherElements["MessagesOverviewView.EmptyState"]
        XCTAssertTrue(emptyState.waitForExistence(timeout: 5.0), "Empty state not displayed")

        let noMessagesText = app.staticTexts["No messages received yet."]
        XCTAssertTrue(noMessagesText.exists, "Empty state title not found")
    }

    @MainActor
    func testEmptyStateShowsHelpSection() throws {
        setMessagesScenario("empty")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        scrollView.swipeUp()

        let helpTitle = app.staticTexts["Do you need help?"]
        XCTAssertTrue(helpTitle.waitForExistence(timeout: 2.0), "Help section not displayed")
    }

    @MainActor
    func testEmptyStateProcessDetailsButton() throws {
        setMessagesScenario("empty")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let detailsButton = app.buttons["MessagesOverviewView.ProcessDetailsButton"]
        XCTAssertTrue(detailsButton.exists, "Process details button not found")
    }

    @MainActor
    func testEmptyStateNoMessageRows() throws {
        setMessagesScenario("empty")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let unreadRows = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Unread")
        let readRows = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Read")
        XCTAssertEqual(unreadRows.count, 0, "No unread message rows should exist in empty state")
        XCTAssertEqual(readRows.count, 0, "No read message rows should exist in empty state")
    }

    @MainActor
    func testMultipleMessagesDisplayed() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0), "Messages scroll view did not appear")

        let messageRows = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Unread")
        XCTAssertNotEqual(messageRows.count, 0, "Message rows should exist")
        XCTAssertTrue(messageRows.firstMatch.waitForExistence(timeout: 5.0), "Message rows not found")
    }

    @MainActor
    func testMultipleMessagesSenderDisplayed() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let sender = app.staticTexts["Central Licensing Office"]
        XCTAssertTrue(sender.waitForExistence(timeout: 5.0), "Message sender not displayed")
    }

    @MainActor
    func testMultipleMessagesTitlesDisplayed() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Application Received"].waitForExistence(timeout: 5.0),
                      "First message title not found")
        XCTAssertTrue(app.staticTexts["Documents Required"].exists,
                      "Second message title not found")
    }

    @MainActor
    func testMultipleMessagesScrollToSeeAll() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        scrollView.swipeUp()

        let thirdMessageTitle = app.staticTexts["Documents Submitted"]
        XCTAssertTrue(thirdMessageTitle.waitForExistence(timeout: 2.0), "Third message not visible after scrolling")
    }

    @MainActor
    func testMultipleMessagesShowsReply() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        scrollView.swipeUp()

        let replyText = app.staticTexts["Thank you, we have received your documents."]
        XCTAssertTrue(replyText.waitForExistence(timeout: 2.0), "Reply content not displayed")
    }

    @MainActor
    func testMultipleMessagesHelpSection() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        scrollView.swipeUp()
        scrollView.swipeUp()

        let helpTitle = app.staticTexts["Do you need help?"]
        XCTAssertTrue(helpTitle.waitForExistence(timeout: 2.0), "Help section not found below messages")
    }

    @MainActor
    func testProcessDetailsButtonNavigates() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let detailsButton = app.buttons["MessagesOverviewView.ProcessDetailsButton"]
        XCTAssertTrue(detailsButton.waitForExistence(timeout: 2.0))
        detailsButton.tap()

        let detailsTitle = app.navigationBars["Case details"]
        XCTAssertTrue(detailsTitle.waitForExistence(timeout: 2.0), "Process details view did not appear")
    }

    @MainActor
    func testBackNavigationReturnsToProcesses() throws {
        setMessagesScenario("empty")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let backButton = app.navigationBars.buttons.element(boundBy: 0)
        XCTAssertTrue(backButton.waitForExistence(timeout: 2.0))
        backButton.tap()

        let processList = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(processList.waitForExistence(timeout: 5.0), "Did not return to processes list")
    }

    @MainActor
    func testAccessibilityAudit() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .textClipped ||
                issue.auditType == .dynamicType ||
                issue.auditType == .contrast {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }

    @MainActor
    func testTappingMessageMarksAsRead() throws {
        setMessagesScenario("multipleMessages")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let unreadRows = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Unread")
        XCTAssertTrue(unreadRows.firstMatch.waitForExistence(timeout: 5.0))
        let unreadCountBefore = unreadRows.count

        let messageContent = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Content")
        XCTAssertTrue(messageContent.firstMatch.waitForExistence(timeout: 5.0))
        messageContent.firstMatch.tap()

        let backButton = app.navigationBars.buttons.element(boundBy: 0)
        XCTAssertTrue(backButton.waitForExistence(timeout: 2.0))
        backButton.tap()

        let readRows = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Read")
        XCTAssertTrue(readRows.firstMatch.waitForExistence(timeout: 5.0),
                      "At least one message should be marked as read after tapping")

        let unreadRowsAfter = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Unread")
        XCTAssertLessThan(unreadRowsAfter.count, unreadCountBefore,
                          "Unread count should decrease after reading a message")
    }

    @MainActor
    func testDueDateLabelDisplayedForRequiresAnswer() throws {
        setMessagesScenario("messagesWithDueDate")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let dueDateLabel = app.descendants(matching: .any).matching(identifier: "DueDateLabel")
        XCTAssertTrue(dueDateLabel.firstMatch.waitForExistence(timeout: 5.0),
                      "Due date label should be displayed for message requiring answer")
    }

    @MainActor
    func testReplyNowButtonDisplayedForRequiresAnswer() throws {
        setMessagesScenario("messagesWithDueDate")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let replyNowButton = app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.ReplyNowButton")
        XCTAssertTrue(replyNowButton.firstMatch.waitForExistence(timeout: 5.0),
                      "Reply now button should be displayed for message requiring answer")
    }

    @MainActor
    func testProcessDueDateAlertDisplayed() throws {
        setMessagesScenario("messagesWithDueDate")
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0))

        let dueDateStatus = app.descendants(matching: .any).matching(identifier: "DueDateStatusView")
        XCTAssertTrue(dueDateStatus.firstMatch.waitForExistence(timeout: 5.0),
                      "Process-level due date status view should be displayed when messages have due dates")
    }
}
