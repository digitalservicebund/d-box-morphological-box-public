//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import SwiftUI

struct InquiryPollView: View {
    let content: NeoContent

    var body: some View {
        switch content {
        case .inquiryPoll(let poll):
            Group {
                Text(L10n.Screen.MessageDetailsView.caseInquiry)
                    .textStyle(.titleDefault)
                    .frame(maxWidth: .infinity, alignment: .leading)

                if let label = poll.question.plain {
                    Text(label)
                        .textStyle(.bodyDefaultRegular)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding(.top, 8)
            .accessibilityIdentifier("InquiryPollView")

        default:
            NeoEmptyView()
        }
    }
}
