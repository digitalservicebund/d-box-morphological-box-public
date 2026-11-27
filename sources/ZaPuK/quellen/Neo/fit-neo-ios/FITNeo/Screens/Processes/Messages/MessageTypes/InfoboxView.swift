//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import SwiftUI

struct InfoboxView: View {
    let content: NeoContent

    var body: some View {
        switch content {
        case .infobox(let title, let infobox, let hyperlink):
            let infoBoxView = NeoInfoView(text: title,
                                          infobox: infobox,
                                          hyperlink: hyperlink)

            HStack {
                VStack(alignment: .leading) {
                    infoBoxView
                }
            }
            .padding(.top, 32)
            .padding(.bottom, 16)
            .accessibilityIdentifier("InfoboxView")

        default:
            NeoEmptyView()
        }
    }
}
