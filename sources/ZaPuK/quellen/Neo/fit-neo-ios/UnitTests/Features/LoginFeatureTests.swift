//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import XCTest

@MainActor
final class LoginFeatureTests: XCTestCase {
    func testInitialState() {
        let store = TestStore(initialState: LoginFeature.State()) {
            LoginFeature()
        }

        XCTAssertEqual(store.state.loginState, .loggedOut)
    }

    func testAuthSucceededChangesStateToLoggedIn() async {
        let store = TestStore(
            initialState: LoginFeature.State(loginState: .loggingIn)
        ) {
            LoginFeature()
        }

        await store.send(.authSucceeded) {
            $0.loginState = .loggedIn
            $0.loginError = nil
        }
    }

    func testAuthFailedChangesStateToLoggedOut() async {
        let store = TestStore(
            initialState: LoginFeature.State(loginState: .loggingIn)
        ) {
            LoginFeature()
        }

        await store.send(.authFailed(.failedLoggingIn)) {
            $0.loginError = .failedLoggingIn
            $0.loginState = .loggedOut
        }
    }

    func testUserCancellationDoesNotSurfaceError() async {
        let store = TestStore(
            initialState: LoginFeature.State(loginState: .loggingIn)
        ) {
            LoginFeature()
        }

        // Cancelling the OIDC flow returns to the logged-out state without raising an error alert.
        await store.send(.authFailed(.oidcError(.userCancellation))) {
            $0.loginState = .loggedOut
        }
    }

    func testLoginButtonTappedSetsLoggingInState() async {
        let store = TestStore(initialState: LoginFeature.State()) {
            LoginFeature()
        }
        store.exhaustivity = .off

        await store.send(.loginButtonTapped) {
            $0.loginState = .loggingIn
        }
    }

    func testOnAppearSetsRestoringSessionState() async {
        let store = TestStore(initialState: LoginFeature.State()) {
            LoginFeature()
        }
        store.exhaustivity = .off

        await store.send(.onAppear) {
            $0.loginState = .restoringSession
        }
    }

    func testLogoutSetsLoggedOutState() async {
        let store = TestStore(
            initialState: LoginFeature.State(loginState: .loggedIn)
        ) {
            LoginFeature()
        } withDependencies: {
            $0.loginClient.destroySession = { }
        }

        await store.send(.logout)
        await store.receive(.logoutCompleted) {
            $0.loginState = .loggedOut
        }
    }
}
