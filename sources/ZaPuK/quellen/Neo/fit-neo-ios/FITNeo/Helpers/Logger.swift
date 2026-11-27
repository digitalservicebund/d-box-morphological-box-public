//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK
import OSLog

public extension Logger {
    private static let subsystem = AppSettings.appIdentifier

    static func enableMatrixSDKLogging() {
        let tracingConfiguration = TracingConfiguration(logLevel: .debug,
                                                        traceLogPacks: [],
                                                        extraTargets: [],
                                                        writeToStdoutOrSystem: true,
                                                        writeToFiles: nil,
                                                        sentryConfig: nil)
        do {
            try initPlatform(config: tracingConfiguration, useLightweightTokioRuntime: false)
        } catch {
            fatalError("Failed configuring logger with error: \(error)")
        }
    }

    static func logger(for type: Any.Type) -> Logger {
        Logger(subsystem: subsystem, category: String(describing: type))
    }

    static func logger(for instance: Any) -> Logger {
        logger(for: type(of: instance))
    }

    static func logger(category: String) -> Logger {
        Logger(subsystem: subsystem, category: category)
    }
}
