//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

class BaseUITestCase: XCTestCase {
    let app = XCUIApplication()

    override func setUpWithError() throws {
        continueAfterFailure = false

        app.launchEnvironment["BYPASS_AUTH"] = "true"
        app.launchEnvironment["UITESTS_DISABLE_ANIMATIONS"] = "true"
        app.launchEnvironment["UI_TEST_PROCESSES_SCENARIO"] = "openOnly"
    }

    override func tearDownWithError() throws {
        if app.state != .notRunning {
            app.terminate()
        }
    }

    func setTestScenario(_ scenario: String) {
        app.launchEnvironment["UI_TEST_PROCESSES_SCENARIO"] = scenario
    }

    func setMessagesScenario(_ scenario: String) {
        app.launchEnvironment["UI_TEST_MESSAGES_SCENARIO"] = scenario
    }

    func launchAndWaitForProcessesTab() {
        app.launch()

        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0), "Tab bar did not appear")

        let processesTab = tabBar.buttons["Cases"]
        XCTAssertTrue(processesTab.waitForExistence(timeout: 5.0), "Processes tab did not appear")
    }

    func launchAndNavigateToMessages() {
        app.launch()

        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0), "Tab bar did not appear")

        let processesTab = tabBar.buttons["Cases"]
        XCTAssertTrue(processesTab.waitForExistence(timeout: 5.0), "Processes tab did not appear")

        let list = app.scrollViews["ProcessesView.List"]
        XCTAssertTrue(list.waitForExistence(timeout: 5.0), "Process list did not appear")

        let processRow = app.staticTexts["License Application"]
        XCTAssertTrue(processRow.waitForExistence(timeout: 5.0), "Process row not found")
        processRow.tap()
    }

    func launchAndNavigateToMessageDetails(messageTitle: String? = nil) {
        launchAndNavigateToMessages()

        let scrollView = app.scrollViews["MessagesOverviewView.ScrollView"]
        XCTAssertTrue(scrollView.waitForExistence(timeout: 5.0), "Messages scroll view did not appear")

        let messageRow: XCUIElement = if let messageTitle {
            app.descendants(matching: .any).staticTexts[messageTitle]
        } else {
            app.descendants(matching: .any).matching(identifier: "MessageSummaryRow.Content").firstMatch
        }
        XCTAssertTrue(messageRow.waitForExistence(timeout: 5.0), "Message row not found")
        messageRow.tap()
    }

    func waitForElementToDisappear(_ element: XCUIElement, timeout: TimeInterval) -> Bool {
        let predicate = NSPredicate(format: "exists == false")
        let expectation = XCTNSPredicateExpectation(predicate: predicate, object: element)
        return XCTWaiter().wait(for: [expectation], timeout: timeout) == .completed
    }
}
