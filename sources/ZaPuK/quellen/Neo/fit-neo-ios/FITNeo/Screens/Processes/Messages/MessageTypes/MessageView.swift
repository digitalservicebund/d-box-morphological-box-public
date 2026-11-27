//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import SwiftUI

struct MessageView: View {
    let content: NeoContent

    var body: some View {
        switch content {
        case .message(let text, _):
            if let message = text.plain {
                Text(message)
                    .textStyle(.bodyDefaultRegular)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.top, 16)
                    .padding(.bottom, 16)
                    .accessibilityIdentifier("MessageView")
            }

        default:
            NeoEmptyView()
        }
    }
}
