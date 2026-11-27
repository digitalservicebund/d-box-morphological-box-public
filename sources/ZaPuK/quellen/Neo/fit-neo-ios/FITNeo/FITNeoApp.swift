// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

@main
struct FITNeoApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    @Environment(\.scenePhase) private var scenePhase
    private var appCoordinator: AppCoordinator!

    init() {
        appCoordinator = AppCoordinator(appDelegate: appDelegate)
    }

    var body: some Scene {
        WindowGroup {
            appCoordinator.toPresentable()
        }
        .onChange(of: scenePhase) { _, newPhase in
            switch newPhase {
            case .active:
                Task { await AppState.shared.resumeSync() }
            case .background:
                AppState.shared.pauseSync()
            default:
                break
            }
        }
    }
}
