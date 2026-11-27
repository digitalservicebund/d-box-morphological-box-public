//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

@testable import FITNeo
import XCTest

final class LoginModeTests: XCTestCase {
    func testSupportsOIDCFlow() {
        XCTAssertTrue(LoginMode.oidc(supportsCreatePrompt: true).supportsOIDCFlow)
        XCTAssertTrue(LoginMode.oidc(supportsCreatePrompt: false).supportsOIDCFlow)

        XCTAssertFalse(LoginMode.unknown.supportsOIDCFlow)
        XCTAssertFalse(LoginMode.password.supportsOIDCFlow)
        XCTAssertFalse(LoginMode.unsupported.supportsOIDCFlow)
    }

    func testOIDCAssociatedValue() {
        let oidcWithPrompt = LoginMode.oidc(supportsCreatePrompt: true)
        let oidcWithoutPrompt = LoginMode.oidc(supportsCreatePrompt: false)

        if case .oidc(let supportsCreatePrompt) = oidcWithPrompt {
            XCTAssertTrue(supportsCreatePrompt)
        } else {
            XCTFail("Expected OIDC mode with supportsCreatePrompt: true")
        }

        if case .oidc(let supportsCreatePrompt) = oidcWithoutPrompt {
            XCTAssertFalse(supportsCreatePrompt)
        } else {
            XCTFail("Expected OIDC mode with supportsCreatePrompt: false")
        }
    }
}
