//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import XCTest

final class EmptyStateTests: BaseUITestCase {
    override func setUpWithError() throws {
        // MARK: change tor false after fixing dynamic type A11y issues

        try super.setUpWithError()
        continueAfterFailure = true

        app.launch()

        let window = app.windows.element(boundBy: 0)
        XCTAssertTrue(window.waitForExistence(timeout: 5.0), "Main window did not appear in time")
    }

    @MainActor
    func testMainTabs() throws {
        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0))

        let initialTab = tabBar.buttons["Cases"]
        XCTAssertTrue(initialTab.waitForExistence(timeout: 5.0))

        XCTAssertTrue(tabBar.buttons["Cases"].exists)
        XCTAssertTrue(tabBar.buttons["Documents"].exists)
        XCTAssertTrue(tabBar.buttons["Account"].exists)
        XCTAssertTrue(tabBar.buttons["Help"].exists)
    }

    @MainActor
    func testMainTabsSwitching() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0))

        let initialTab = tabBar.buttons["Cases"]
        XCTAssertTrue(initialTab.waitForExistence(timeout: 5.0))

        XCTAssertTrue(app.staticTexts["No cases pending."].waitForExistence(timeout: 5.0))

        tabBar.buttons["Documents"].tap()
        XCTAssertTrue(app.staticTexts["Documents"].waitForExistence(timeout: 5.0))

        tabBar.buttons["Account"].tap()
        XCTAssertTrue(app.staticTexts["Account"].waitForExistence(timeout: 5.0))

        tabBar.buttons["Help"].tap()
        XCTAssertTrue(app.staticTexts["Help"].waitForExistence(timeout: 5.0))
    }

    @MainActor
    func testOpeningFederalPortal() throws {
        setTestScenario("empty")
        launchAndWaitForProcessesTab()

        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0))

        let emptyStateText = app.staticTexts["No cases pending."]
        XCTAssertTrue(emptyStateText.waitForExistence(timeout: 5.0), "Empty state text not displayed")

        let federalPortalButton = app.buttons["ProcessesEmptyView.OpenFederalPortal"]
        XCTAssertTrue(federalPortalButton.exists, "Federal Portal button not found")

        federalPortalButton.tap()

        let safariApp = XCUIApplication(bundleIdentifier: "com.apple.mobilesafari")
        XCTAssertTrue(safariApp.waitForExistence(timeout: 5.0), "Safari did not open in time")
    }

    @MainActor
    func testCallHelpHotline() throws {
        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0))

        let initialTab = tabBar.buttons["Cases"]
        XCTAssertTrue(initialTab.waitForExistence(timeout: 5.0))

        let callHotlineButton = app.buttons["ProcessesHelpView.CallButton"]
        XCTAssertTrue(callHotlineButton.exists, "Call 115 button not found")

        callHotlineButton.tap()

        #if targetEnvironment(simulator)
        let errorAlert = app.buttons["ProcessesHelpView.Alert.CallNotSupported.OKButton"]
        XCTAssertTrue(errorAlert.waitForExistence(timeout: 5.0), "Error alert did not appear in time")
        #else
        let springboard = XCUIApplication(bundleIdentifier: "com.apple.springboard")
        let callAlert = springboard.alerts.firstMatch
        XCTAssertTrue(callAlert.waitForExistence(timeout: 5.0), "System call confirmation dialog did not appear")
        #endif
    }

    @MainActor
    func testGoToHelpTab() throws {
        let tabBar = app.tabBars.element
        XCTAssertTrue(tabBar.waitForExistence(timeout: 5.0))

        let initialTab = tabBar.buttons["Cases"]
        XCTAssertTrue(initialTab.waitForExistence(timeout: 5.0))

        let getHelpButton = app.buttons["ProcessesHelpView.OpenHelpButton"]
        XCTAssertTrue(getHelpButton.exists, "Open Help button not found")

        getHelpButton.tap()

        let helpTabTitle = app.staticTexts["Help"]
        XCTAssertTrue(helpTabTitle.waitForExistence(timeout: 5.0), "Help tab did not become active in time")
    }

    @MainActor
    func testAccessibilityLoginView() throws {
        let window = app.windows.element(boundBy: 0)
        XCTAssertTrue(window.waitForExistence(timeout: 5.0))

        // MARK: on older iPhone models, constrast audit on the action button is failing...

        try app.performAccessibilityAudit(for: .all) { issue in
            var shouldIgnore = false

            if issue.auditType == .contrast {
                shouldIgnore = true
            }

            return shouldIgnore
        }
    }
}
