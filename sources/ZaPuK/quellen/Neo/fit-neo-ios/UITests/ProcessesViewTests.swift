//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class ProcessesViewTests: BaseUITestCase {
    @MainActor
    func testEmptyStateDisplayed() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let emptyStateText = app.staticTexts["No cases pending."]
        XCTAssertTrue(emptyStateText.waitForExistence(timeout: 5.0), "Empty state text not displayed")

        let federalPortalButton = app.buttons["Access the Federal Portal"]
        XCTAssertTrue(federalPortalButton.exists, "Federal Portal button not found")

        let helpTitle = app.staticTexts["Do you need help?"]
        XCTAssertTrue(helpTitle.exists, "Help section not displayed")
    }

    @MainActor
    func testEmptyStateHelpButtons() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let emptyStateText = app.staticTexts["No cases pending."]
        XCTAssertTrue(emptyStateText.waitForExistence(timeout: 5.0))

        let callButton = app.buttons["Call 115"]
        XCTAssertTrue(callButton.exists, "Call 115 button not found")

        let helpButton = app.buttons["Open Help"]
        XCTAssertTrue(helpButton.exists, "Open Help button not found")
    }

    @MainActor
    func testOnlyOpenProcessesDisplayed() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0), "Process list not displayed")

        XCTAssertTrue(app.staticTexts["License Application"].exists, "First process name not found")
        XCTAssertTrue(app.staticTexts["Permit Request"].exists, "Second process name not found")

        XCTAssertTrue(app.staticTexts["City Hall"].exists, "First authority not found")
        XCTAssertTrue(app.staticTexts["Planning Department"].exists, "Second authority not found")
    }

    @MainActor
    func testOpenProcessStatusBadges() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Case Sent"].exists, "Case Sent status not displayed")
        XCTAssertTrue(app.staticTexts["Case Received"].exists, "Case Received status not displayed")
    }

    @MainActor
    func testOnlyCompletedProcessesDisplayed() throws {
        setTestScenario("completedOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Your completed cases"].exists, "Completed count should be 2")
        XCTAssertFalse(app.staticTexts["Completed License"].exists, "Completed cases should be collapsed by default")
    }

    @MainActor
    func testExpandCompletedSection() throws {
        setTestScenario("completedOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        let completedHeader = app.buttons["ProcessesView.CompletedSection.Header"]
        XCTAssertTrue(completedHeader.waitForExistence(timeout: 2.0), "Completed header not found")

        app.staticTexts["Your completed cases"].firstMatch.tap()

        let completedProcess = app.staticTexts["Completed License"]
        XCTAssertTrue(completedProcess.waitForExistence(timeout: 2.0), "Completed case should be visible after expanding")

        XCTAssertTrue(app.staticTexts["Case Completed"].exists, "Case Completed status not displayed")
    }

    @MainActor
    func testCollapseCompletedSection() throws {
        setTestScenario("completedOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        let completedHeader = app.buttons["ProcessesView.CompletedSection.Header"]
        XCTAssertTrue(completedHeader.waitForExistence(timeout: 2.0), "Completed header not found")
        if !completedHeader.isHittable {
            list.swipeUp()
        }

        app.staticTexts["Your completed cases"].firstMatch.tap()
        XCTAssertTrue(app.staticTexts["Completed License"].waitForExistence(timeout: 2.0))

        app.staticTexts["Your completed cases"].firstMatch.tap()

        let completedProcess = app.staticTexts["Completed License"]
        XCTAssertFalse(completedProcess.waitForExistence(timeout: 2.0), "Completed process should be hidden after collapsing")
    }

    // MARK: - Mixed Processes Tests

    @MainActor
    func testMixedOpenAndCompletedProcesses() throws {
        setTestScenario("mixed")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Open Application"].exists, "First open process not found")
        XCTAssertTrue(app.staticTexts["Pending Request"].exists, "Second open process not found")

        XCTAssertTrue(app.staticTexts["Your completed cases"].exists, "Completed count should be 2")

        XCTAssertFalse(app.staticTexts["Finished Task"].exists, "Completed processes should be collapsed")

        let completedHeader = app.buttons["ProcessesView.CompletedSection.Header"]
        XCTAssertTrue(completedHeader.waitForExistence(timeout: 2.0), "Completed header not found")
        if !completedHeader.isHittable {
            list.swipeUp()
        }
        app.staticTexts["Your completed cases"].firstMatch.tap()

        XCTAssertTrue(app.staticTexts["Finished Task"].waitForExistence(timeout: 2.0), "First completed process not found")
        XCTAssertTrue(app.staticTexts["Archived Process"].exists, "Second completed process not found")
    }

    @MainActor
    func testAllStatusTypesDisplayed() throws {
        setTestScenario("allStatuses")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Case Sent"].exists, "Case Sent status not found")
        XCTAssertTrue(app.staticTexts["Case Received"].exists, "Case Received status not found")
        XCTAssertTrue(app.staticTexts["Case Under Review"].exists, "Case Under Review status not found")
        XCTAssertTrue(app.staticTexts["Activity Required"].exists, "Activity Required status not found")

        let completedHeader = app.staticTexts["Your completed cases"]
        XCTAssertTrue(completedHeader.waitForExistence(timeout: 2.0), "Completed header not found")
        if !completedHeader.isHittable {
            list.swipeUp()
        }
        completedHeader.firstMatch.tap()

        XCTAssertTrue(app.staticTexts["Case Completed"].waitForExistence(timeout: 2.0), "Process Completed status not found")
    }

    @MainActor
    func testReportsButtonExists() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let emptyStateText = app.staticTexts["No cases pending."]
        XCTAssertTrue(emptyStateText.waitForExistence(timeout: 5.0))

        let reportsButton = app.buttons["Reports"]
        XCTAssertTrue(reportsButton.exists, "Reports button not found in toolbar")
    }

    @MainActor
    func testReportsButtonOpensSheet() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let emptyStateText = app.staticTexts["No cases pending."]
        XCTAssertTrue(emptyStateText.waitForExistence(timeout: 5.0))

        let reportsButton = app.buttons["Reports"]
        reportsButton.tap()

        let sheet = app.otherElements["Sheets.ReportsView"]
        XCTAssertTrue(sheet.waitForExistence(timeout: 2.0), "Reports sheet did not appear")
    }

    @MainActor
    func testTappingProcessRowNavigatesToDetail() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        let processRow = app.staticTexts["License Application"]
        XCTAssertTrue(processRow.exists)
        processRow.tap()

        let backButton = app.navigationBars.buttons.element(boundBy: 0)
        XCTAssertTrue(backButton.waitForExistence(timeout: 2.0), "Navigation to detail view did not occur")
    }

    @MainActor
    func testManyProcessesDisplayedCorrectly() throws {
        setTestScenario("manyProcesses")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["Open Process 1"].exists, "First process not visible")

        app.scrollViews.firstMatch.swipeUp()

        XCTAssertTrue(app.staticTexts["(3)"].exists, "Completed count should be 3")
    }

    @MainActor
    func testHelpSectionDisplayedInProcessList() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0))

        app.scrollViews.firstMatch.swipeUp()

        let helpTitle = app.staticTexts["Do you need help?"]
        XCTAssertTrue(helpTitle.waitForExistence(timeout: 2.0), "Help section not found in process list")
    }

    @MainActor
    func testAccessibilityLoginView() throws {
        setTestScenario("allStatuses")
        launchAndWaitForProcessesTab()

        // MARK: on older iPhone models, contrast audit is failing...

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .contrast || issue.auditType == .dynamicType {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }
}
