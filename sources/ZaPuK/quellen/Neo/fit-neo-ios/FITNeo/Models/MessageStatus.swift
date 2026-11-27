//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SFSafeSymbols
import SwiftUI

enum MessageStatus {
    case received
    case requiresAnswer
    case answered
}

struct MessageStatusType {
    private let status: MessageStatus

    init(status: MessageStatus) {
        self.status = status
    }

    var symbol: SFSymbol {
        switch status {
        case .answered:
            .checkmarkCircleFill
        case .received:
            .checkmarkCircleFill
        case .requiresAnswer:
            .exclamationmarkTriangleFill
        }
    }

    var color: Color {
        switch status {
        case .answered:
            Asset.Theme.feedbackSuccess.swiftUIColor
        case .received:
            Asset.Theme.feedbackLoader.swiftUIColor
        case .requiresAnswer:
            Asset.Theme.feedbackWarning.swiftUIColor
        }
    }

    var backgroundColor: Color {
        switch status {
        case .answered:
            Asset.Theme.feedbackSuccessBackground.swiftUIColor
        case .received:
            Asset.Theme.layoutBackgroundHued.swiftUIColor
        case .requiresAnswer:
            Asset.Theme.feedbackWarningBackground.swiftUIColor
        }
    }
}
