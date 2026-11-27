//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import DeviceKit
import Foundation
import UIKit

enum UserAgentHelper {
    static func userAgentString() -> String {
        guard let clientVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String else {
            return UserAgentHelper
                .userAgentStringWithProductVersion(AppSettings.appDisplayName,
                                                   "unknown")
        }

        return UserAgentHelper.userAgentStringWithProductVersion(AppSettings.appDisplayName,
                                                                 clientVersion)
    }

    static func userAgentStringWithProductVersion(_ product: String, _ version: String) -> String {
        String(format: "%@/%@ (%@; iOS %@)",
               product,
               version,
               Device.current.safeDescription,
               UIDevice.current.systemVersion)
    }
}
