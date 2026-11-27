//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct NeoDueDateLabel: View {
    let date: Date

    var body: some View {
        let dueDate = date.formatted(date: .numeric, time: .omitted)
        let dueDateInfo = NeoGenericInfo(title: L10n.Screen.Warning.messageDueDate(dueDate),
                                         type: .warning,
                                         withSymbol: false)
        NeoInfoView(info: dueDateInfo)
            .accessibilityIdentifier("DueDateLabel")
    }
}

struct NeoDueDateLabelWithSymbol: View {
    let date: Date

    var body: some View {
        let dueDate = date.formatted(date: .numeric, time: .omitted)
        let dueDateInfo = NeoGenericInfo(title: L10n.Screen.Warning.messageDueDate(dueDate),
                                         type: .warning,
                                         withSymbol: true)
        NeoInfoView(info: dueDateInfo)
            .accessibilityIdentifier("DueDateLabelWithSymbol")
    }
}

struct NeoDueDateStatusView: View {
    let date: Date

    var body: some View {
        let dueDate = date.formatted(date: .numeric, time: .omitted)
        let alert = NeoGenericInfo(title: L10n.Status.processActivityRequired,
                                   subtitle: L10n.Screen.Warning.processDueDate(dueDate),
                                   type: .warning,
                                   withSymbol: true)
        NeoInfoView(info: alert, showInfo: true)
            .accessibilityIdentifier("DueDateStatusView")
    }
}
