//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SFSafeSymbols
import SwiftUI

enum NeoTabs: Equatable, Hashable {
    case processes
    case documents
    case account
    case help

    var name: String {
        switch self {
        case .processes: String(L10n.Navigation.Tab.processes)
        case .documents: String(L10n.Navigation.Tab.documents)
        case .account: String(L10n.Navigation.Tab.account)
        case .help: String(L10n.Navigation.Tab.help)
        }
    }

    var symbol: SFSymbol {
        switch self {
        case .processes: SFSymbol.envelope
        case .documents: SFSymbol.textPage
        case .account: SFSymbol.person
        case .help: SFSymbol.questionmarkCircle
        }
    }
}
