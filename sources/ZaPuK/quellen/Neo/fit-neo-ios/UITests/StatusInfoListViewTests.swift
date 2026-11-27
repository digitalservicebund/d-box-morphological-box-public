//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class StatusInfoListViewTests: BaseUITestCase {
    @MainActor
    func testStatusListViewIsDisplayed() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0), "Process list not displayed")

        let process1 = app.staticTexts["License Application"]
        XCTAssertTrue(process1.exists, "First process name not found")
        process1.tap()

        let statusView = app.staticTexts["Case Sent"]
        XCTAssertTrue(statusView.waitForExistence(timeout: 5.0), "Process detail view not displayed")
        statusView.tap()

        let statusText = app.staticTexts["Status Explanation"]
        XCTAssertTrue(statusText.waitForExistence(timeout: 5.0), "Status detail view not displayed")
    }

    @MainActor
    func testIfActiveStatusIsExpanded() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0), "Process list not displayed")

        let process1 = app.staticTexts["License Application"]
        process1.tap()

        let statusView = app.staticTexts["Case Sent"]
        statusView.tap()

        let statusText = app.staticTexts["Status Explanation"]
        XCTAssertTrue(statusText.waitForExistence(timeout: 5.0), "Status detail view not displayed")

        let sentTitle = app.staticTexts["Case Sent"]
        XCTAssertTrue(sentTitle.exists, "Process Sent item not found on list")

        let sentText = app.staticTexts["StatusInfoListView.Text.Case Sent"]
        XCTAssertTrue(sentText.exists, "Process Sent detail not shown on list")
    }

    @MainActor
    func testIfStatusCanBeCollapsedAndExpanded() throws {
        setTestScenario("openOnly")
        launchAndWaitForProcessesTab()

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0), "Process list not displayed")

        let process2 = app.staticTexts["Permit Request"]
        process2.tap()

        let statusView = app.staticTexts["Case Received"]
        statusView.tap()

        let statusText = app.staticTexts["Status Explanation"]
        XCTAssertTrue(statusText.waitForExistence(timeout: 5.0), "Status detail view not displayed")

        let receivedDetail = app.staticTexts["StatusInfoListView.Text.Case Received"]
        XCTAssertTrue(receivedDetail.exists, "Active status detail not shown on list")

        let sentTitle = app.staticTexts["Case Sent"]
        XCTAssertTrue(sentTitle.exists, "Process Sent item not shown on list")

        var sentDetail = app.staticTexts["StatusInfoListView.Text.Case Sent"]
        XCTAssertFalse(sentDetail.exists, "Process Sent item detail not shown on list")

        let sentButton = app.buttons["StatusInfoListView.Toggle.Case Sent"]
        XCTAssertTrue(sentButton.exists, "Process Sent toggle not shown in list")
        sentButton.tap()

        sentDetail = app.staticTexts["StatusInfoListView.Text.Case Sent"]
        XCTAssertTrue(sentDetail.exists, "Process sent item detail should be on list")

        let receivedButton = app.buttons["StatusInfoListView.Toggle.Case Received"]

        receivedButton.tap()
        XCTAssertFalse(receivedDetail.exists, "Process Received item detail should be hidden")
    }
}
