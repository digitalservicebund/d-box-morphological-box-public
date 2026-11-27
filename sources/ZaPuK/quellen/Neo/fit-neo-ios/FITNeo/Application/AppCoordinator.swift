//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import OSLog
import SwiftUI

class AppCoordinator {
    private let appDelegate: AppDelegate

    init(appDelegate: AppDelegate) {
        Logger.enableMatrixSDKLogging()

        self.appDelegate = appDelegate
    }

    @MainActor func toPresentable() -> AnyView {
        AppState.shared.setCoordinator(self)

        return AnyView(AppView())
    }
}
