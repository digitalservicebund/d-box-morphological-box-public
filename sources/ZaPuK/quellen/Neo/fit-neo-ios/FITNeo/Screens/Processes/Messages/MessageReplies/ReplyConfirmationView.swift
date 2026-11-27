//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct ReplyConfirmationView: View {
    let store: StoreOf<ReplyConfirmationFeature>

    @ViewBuilder
    private var replyView: some View {
        switch store.inquiryContent {
        case .inquiryMessage:
            FreeTextReplyEditorView.confirmationView(store.previewContent, inquiry: store.inquiryContent)
        case .inquiryPoll:
            SingleSelectReplyEditorView.confirmationView(store.previewContent, inquiry: store.inquiryContent)
        default:
            UnsupportedReplyEditorView.confirmationView([], inquiry: store.inquiryContent)
        }
    }

    @State private var previewContentHeight: CGFloat = 32

    @ViewBuilder
    func dialogHeader() -> some View {
        VStack(alignment: .leading) {
            Text(L10n.Screen.Message.Reply.Confirm.title)
                .textStyle(.bodyDefaultSemiBold)
                .accessibilityIdentifier("ReplyConfirmationView.Title")

            Text(L10n.Screen.Message.Reply.Confirm.details)
                .textStyle(.bodyDefaultRegular)
                .accessibilityIdentifier("ReplyConfirmationView.Details")
        }
        .padding(.horizontal, 8)
        .padding(.top, 8)
        .padding(.bottom, 24)
    }

    @ViewBuilder
    func replyPreview() -> some View {
        ScrollView {
            VStack(alignment: .leading) {
                Text(L10n.Screen.Message.Reply.Confirm.answer)
                    .textStyle(.bodyDefaultSemiBold)
                    .padding(.top, 16)
                    .padding(.bottom, 8)
                    .padding(.horizontal, 16)

                replyView
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.horizontal, 16)
                    .padding(.bottom, 8)
                    .accessibilityIdentifier("ReplyConfirmationView.PreviewContent")
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(
                RoundedRectangle(cornerRadius: 8)
                    .fill(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            )
            .onGeometryChange(for: CGFloat.self) { proxy in
                proxy.size.height
            } action: { height in
                previewContentHeight = height
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .frame(height: min(previewContentHeight, 300))
        .scrollMoreIndicator()
    }

    @ViewBuilder
    func optionButtons() -> some View {
        VStack {
            Button {
                store.send(.confirmSend)
            } label: {
                Text(L10n.Screen.Message.Reply.Submit.button)
                    .frame(maxWidth: .infinity)
            }
            .modifier(NeoPrimaryButtonStyleModifier())
            .accessibilityIdentifier("ReplyConfirmationView.ConfirmButton")

            Button {
                store.send(.cancelSend)
            } label: {
                Text(L10n.Screen.Message.Reply.Confirm.CheckDetails.button)
                    .frame(maxWidth: .infinity)
            }
            .modifier(NeoSecondaryButtonStyleModifier())
            .accessibilityIdentifier("ReplyConfirmationView.CancelButton")
        }
    }

    @ViewBuilder
    func submissionSending() -> some View {
        VStack(spacing: 16) {
            ProgressView()
                .controlSize(.large)
                .accessibilityIdentifier("ReplyConfirmationView.SendingIndicator")

            Text(L10n.Screen.Message.Reply.Confirm.sending)
                .textStyle(.bodyDefaultRegular)
                .accessibilityIdentifier("ReplyConfirmationView.SendingLabel")
        }
        .padding(16)
    }

    @ViewBuilder
    func submissionSuccess() -> some View {
        VStack(alignment: .leading) {
            Text(L10n.Screen.Message.Reply.Success.title)
                .textStyle(.bodyDefaultSemiBold)
                .padding(.horizontal, 16)
                .accessibilityIdentifier("ReplyConfirmationView.SuccessTitle")

            Text(L10n.Screen.Message.Reply.Success.details)
                .textStyle(.bodyDefaultRegular)
                .padding(.horizontal, 16)
                .padding(.bottom, 16)
                .accessibilityIdentifier("ReplyConfirmationView.SuccessDetails")

            Button {
                store.send(.sendSuccess)
            } label: {
                Text(L10n.Screen.Message.Reply.Success.button)
                    .frame(maxWidth: .infinity)
            }
            .modifier(NeoPrimaryButtonStyleModifier())
            .accessibilityIdentifier("ReplyConfirmationView.DoneButton")
        }
    }

    @ViewBuilder
    func confirmationDialog() -> some View {
        VStack {
            if store.submitStatus == .unsent {
                dialogHeader()
                replyPreview()
                    .padding(.bottom, 24)
                optionButtons()
            } else if store.submitStatus == .sending {
                submissionSending()
            } else if store.submitStatus == .sent {
                submissionSuccess()
            }
        }
        .padding(14)
        .glassedEffect(in: RoundedRectangle(cornerRadius: 32))
    }

    var body: some View {
        OverlayDialogView(onBackgroundTap: {
            if store.submitStatus == .unsent {
                store.send(.cancelSend)
            }
        }, content: {
            confirmationDialog()
        })
    }
}

struct ReplyConfirmationView_Previews: PreviewProvider {
    private static let inquiry: NeoContent = .inquiryMessage(title: nil, label: nil)

    private static let poll: NeoContent = .inquiryPoll(
        poll: NeoPollContent(multiSelect: false,
                             question: NeoText(plain: "How tall was the dragon?", html: nil),
                             answers: [
                                 NeoPollAnswer(id: "2",
                                               text: NeoText(plain: "Between 1 and 2 meters", html: nil))
                             ])
    )

    private static func message(_ text: String) -> [NeoContent] {
        [.message(text: NeoText(plain: text, html: nil), replyTo: nil)]
    }

    private static func store(inquiryContent: NeoContent,
                              previewContent: [NeoContent],
                              submitStatus: SubmitReplyStatus) -> StoreOf<ReplyConfirmationFeature> {
        Store(
            initialState: ReplyConfirmationFeature.State(inquiryContent: inquiryContent,
                                                         previewContent: previewContent,
                                                         submitStatus: submitStatus)
        ) {
            ReplyConfirmationFeature()
        }
    }

    static var previews: some View {
        let shortReplyContent = "Yes"

        let longReplyContent = """
        Sehr geehrte Damen und Herren, hiermit bestätige ich den Eingang Ihres Schreibens vom ca.
        12 ten dieses Monats bezüglich unserer Zusammenarbeit. Mit freundlichen Grüßen, Hans Bauer
        Sehr geehrte Damen und Herren, wir freuen uns, Ihnen mitteilen zu dürfen, dass der Auftrag\n
        erfolgreich abgeschlossen wurde und alle Beteiligten zufrieden sind. Hochachtungsvoll, Team
        Sehr geehrte Damen und Herren, anbei übersenden wir Ihnen die gewünschten Unterlagen für
        Ihre Prüfung und bitten um eine kurze Rückmeldung. Mit besten Grüßen, Sabine Hoffmann\n
        Bitte nehmen Sie zur Kenntnis, dass die Lieferung voraussichtlich in drei Werktagen erfolgt.
        Sehr geehrte Damen und Herren, für Rückfragen stehen wir Ihnen jederzeit gerne zur Verfügung.
        Wir danken Ihnen für Ihr Vertrauen und freuen uns auf eine weitere gute Zusammenarbeit heute.
        """

        return Group {
            Color.clear
                .overlay(alignment: .bottom) {
                    ReplyConfirmationView(store: store(inquiryContent: inquiry,
                                                       previewContent: message(shortReplyContent),
                                                       submitStatus: .unsent))
                }
                .previewDisplayName("Unsent Short")

            Color.clear
                .overlay(alignment: .bottom) {
                    ReplyConfirmationView(store: store(inquiryContent: inquiry,
                                                       previewContent: message(longReplyContent),
                                                       submitStatus: .unsent))
                }
                .previewDisplayName("Unsent Long")

            Color.clear
                .overlay(alignment: .bottom) {
                    ReplyConfirmationView(store: store(inquiryContent: poll,
                                                       previewContent: [.poll(selections: ["2"],
                                                                              custom: nil,
                                                                              replyTo: nil)],
                                                       submitStatus: .unsent))
                }
                .previewDisplayName("Unsent Poll")

            Color.clear
                .overlay(alignment: .bottom) {
                    ReplyConfirmationView(store: store(inquiryContent: inquiry,
                                                       previewContent: message(shortReplyContent),
                                                       submitStatus: .sending))
                }
                .previewDisplayName("Sending")

            Color.clear
                .overlay(alignment: .bottom) {
                    ReplyConfirmationView(store: store(inquiryContent: inquiry,
                                                       previewContent: message(shortReplyContent),
                                                       submitStatus: .sent))
                }
                .previewDisplayName("Sent")
        }
    }
}
