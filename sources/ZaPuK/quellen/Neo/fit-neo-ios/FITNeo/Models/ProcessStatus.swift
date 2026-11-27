//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation

enum ProcessStatus: Equatable, Hashable {
    case processSent
    case processReceived
    case processUnderReview
    case processActivityRequired
    case processCompleted

    var detailInfo: String {
        switch self {
        case .processSent: String(L10n.Screen.StatusInfoListView.Info.processSent)
        case .processReceived: String(L10n.Screen.StatusInfoListView.Info.processReceived)
        case .processUnderReview: String(L10n.Screen.StatusInfoListView.Info.processUnderReview)
        case .processActivityRequired: String(
                L10n.Screen.StatusInfoListView.Info.processActivityRequired
            )
        case .processCompleted: String(L10n.Screen.StatusInfoListView.Info.processCompleted)
        }
    }
}
