//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import SwiftUI

struct InquiryMessageView: View {
    let content: NeoContent

    var body: some View {
        switch content {
        case .inquiryMessage(let title, let label):
            Group {
                if let title = title?.plain {
                    Text(title)
                        .textStyle(.titleDefault)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let label = label?.plain {
                    Text(label)
                        .textStyle(.bodyDefaultRegular)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding(.top, 8)
            .accessibilityIdentifier("InquiryMessageView")

        default:
            NeoEmptyView()
        }
    }
}
