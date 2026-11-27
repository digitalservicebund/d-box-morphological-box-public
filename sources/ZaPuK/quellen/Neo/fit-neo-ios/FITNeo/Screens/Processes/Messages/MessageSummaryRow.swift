//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import SwiftUI

struct DialogPreviewContent {
    let title: String?
    let label: String?
}

struct MessageSummaryRow: View {
    var dialog: NeoDialogProxy
    var monthSeparator: String?
    var onAttachmentsTapped: (() -> Void)?
    var onReplyNowTapped: (() -> Void)?

    private let status = MessageStatusType(status: .received)

    private var sender: String {
        dialog.blocks.first?.sender ?? ""
    }

    private var summaryContent: DialogPreviewContent {
        switch dialog.blocks.first?.content {
        case .message(let text, _):
            DialogPreviewContent(title: nil,
                                 label: text.plain)

        case .inquiryMessage(let title, let label):
            DialogPreviewContent(title: title?.plain,
                                 label: label?.plain)

        case .infobox(let title, let infobox, _):
            DialogPreviewContent(title: title.plain,
                                 label: infobox.plain)

        case .button(let hyperlink):
            DialogPreviewContent(title: hyperlink.label.plain,
                                 label: hyperlink.url)

        case .inquiryFile(let title, let label):
            DialogPreviewContent(title: title?.plain,
                                 label: label?.plain)

        case .inquiryPoll(let poll):
            DialogPreviewContent(title: L10n.Screen.MessageDetailsView.caseInquiry,
                                 label: poll.question.plain)

        case .poll(let selections, _, _):
            DialogPreviewContent(title: nil,
                                 label: "with \(selections.count) options")

        case .file(_, let title, let caption):
            DialogPreviewContent(title: title?.plain,
                                 label: caption)

        case .image(_, let title, _, _, let caption, _):
            DialogPreviewContent(title: title?.plain,
                                 label: caption?.plain)

        case .video(_, let title, _, _, let caption):
            DialogPreviewContent(title: title?.plain,
                                 label: caption)

        case .audio(_, let title, _, let caption):
            DialogPreviewContent(title: title?.plain,
                                 label: caption)
        case .none:
            DialogPreviewContent(title: nil,
                                 label: nil)
        }
    }

    private var title: String? {
        summaryContent.title
    }

    private var detail: String? {
        summaryContent.label
    }

    private var lastAnswerSender: String { dialog.lastAnswer?.blocks.first?.sender ?? "" }

    init(dialog: NeoDialogProxy,
         monthSeparator: String?,
         onAttachmentsTapped: (() -> Void)? = nil,
         onReplyNowTapped: (() -> Void)? = nil) {
        self.dialog = dialog
        self.monthSeparator = monthSeparator
        self.onAttachmentsTapped = onAttachmentsTapped
        self.onReplyNowTapped = onReplyNowTapped

        if dialog.status == .requiresAnswer,
           dialog.lastAnswer != nil {
            self.dialog.status = .answered
        }
    }

    @ViewBuilder
    func statusBadgeView() -> some View {
        let status = MessageStatusType(status: dialog.status)

        ZStack {
            Circle()
                .frame(width: 40, height: 40)
                .foregroundColor(status.backgroundColor)
                .overlay(
                    RoundedRectangle(cornerRadius: 32, style: .continuous)
                        .stroke(status.color)
                )
            Image(systemSymbol: status.symbol)
                .font(.system(size: 20))
                .foregroundStyle(status.color)
                .padding(0)
                .accessibilityHidden(true)
        }
        .overlay(alignment: .topTrailing) {
            if dialog.unread {
                Circle()
                    .frame(width: 8, height: 8)
                    .foregroundColor(Asset.Theme.feedbackInfo.swiftUIColor)
            }
        }
    }

    @ViewBuilder
    func summaryContentView() -> some View {
        HStack {
            Text(dialog.formattedTimestamp())
                .textStyle(.bodySmallRegular)
                .lineLimit(1)
                .layoutPriority(1)
            Spacer(minLength: 16)
            Text(sender)
                .textStyle(.bodySmallRegular)
                .lineLimit(1)
        }
        .padding(.bottom, 8)

        if let title {
            Text(title)
                .textStyle(.titleDefault)
                .lineLimit(2)
                .padding(.bottom, 4)
        }

        if let detail {
            Text(detail)
                .textStyle(.bodyDefaultRegular)
                .lineLimit(title != nil ? 2 : 4)
        }
    }

    @ViewBuilder
    func attachmentsView() -> some View {
        let attachments = dialog.message.attachments()
        if let firstAttachment = attachments.first {
            HStack {
                let attachmentInfo = NeoGenericInfo(title: firstAttachment.filename,
                                                    type: .attachment,
                                                    withSymbol: true)
                NeoInfoView(info: attachmentInfo)
                if attachments.count > 1 {
                    let countString = "+\(attachments.count - 1)"
                    let attachmentCountInfo = NeoGenericInfo(title: countString, type: .attachment, withSymbol: false)
                    NeoInfoView(info: attachmentCountInfo)
                }
            }
            .padding(.top, 8)
        }
    }

    @ViewBuilder
    func lastAnswerView() -> some View {
        if let timestamp = dialog.lastAnswer?.formattedTimestamp() {
            HStack {
                Text(timestamp)
                    .textStyle(.bodySmallRegular)
                    .lineLimit(1)
                    .layoutPriority(1)
                Spacer(minLength: 16)
                Text(lastAnswerSender)
                    .textStyle(.bodySmallRegular)
                    .lineLimit(1)
            }
            .padding(.bottom, 8)

            Text(L10n.Screen.Messages.yourReply)
                .textStyle(.bodyDefaultSemiBold)
                .lineLimit(2)
                .padding(.bottom, 4)

            Text(dialog.lastAnswerSummary())
                .textStyle(.bodyDefaultRegular)
                .lineLimit(2)
        }
    }

    var body: some View {
        let readStatusColor = dialog.unread ?
            Asset.Theme.feedbackInfoBackground.swiftUIColor :
            Asset.Theme.layoutBackgroundDefault.swiftUIColor

        VStack(alignment: .leading) {
            if let monthSeparator {
                Text(monthSeparator)
                    .textStyle(.sublineDefault)
                    .padding(.vertical, 16)
            }

            HStack(alignment: .center, spacing: 0) {
                statusBadgeView()
                    .padding(.leading, -56)

                VStack(alignment: .leading, spacing: 0) {
                    HStack {
                        VStack(alignment: .leading, spacing: 0) {
                            summaryContentView()
                                .accessibilityIdentifier("MessageSummaryRow.Content")

                            if !dialog.message.attachments().isEmpty {
                                Button {
                                    onAttachmentsTapped?()
                                } label: {
                                    attachmentsView()
                                }
                                .buttonStyle(.plain)
                                .accessibilityIdentifier("MessageSummaryRow.Attachments")
                            }
                        }
                        Spacer()
                        Image(systemSymbol: .chevronRight)
                            .scaleEffect(1.5)
                            .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
                            .padding(.leading, 16)
                            .padding(.trailing, 4)
                    }
                    .padding(.bottom,
                             dialog.hasAnswers || dialog.status == .requiresAnswer
                                 ? 16
                                 : 0)

                    if dialog.status == .requiresAnswer,
                       !dialog.hasAnswers {
                        if let date = dialog.dueDate {
                            NeoDueDateLabel(date: date)
                                .padding(.bottom, 16)
                        }

                        Button {
                            onReplyNowTapped?()
                        } label: {
                            Text(L10n.Screen.Action.replyNow)
                                .frame(maxWidth: .infinity)
                        }
                        .modifier(NeoPrimaryButtonStyleModifier())
                        .accessibilityIdentifier("MessageSummaryRow.ReplyNowButton")
                    }

                    if dialog.hasAnswers {
                        VStack(alignment: .leading, spacing: 0) {
                            lastAnswerView()
                        }
                        .padding(16)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8, style: .continuous)
                                .stroke(Asset.Theme.divider.swiftUIColor)
                        )
                    }
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .fill(readStatusColor)
                )
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityIdentifier(dialog.unread ? "MessageSummaryRow.Unread" : "MessageSummaryRow.Read")
    }
}

struct MessageSummaryRow_Previews: PreviewProvider {
    private typealias F = NeoDialogFixtures

    static var previews: some View {
        let now = F.ts()

        let replyEnvelope = F.envelope(id: "reply-env", timestamp: now, blocks: [
            F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: now,
                           text: """
                           Sehr geehrte Damen und Herren, bei dem entsprechenden
                           Drachen handelt es sich um einen ca. 65 cm großen,
                           grünen Walddrachen handeln. Mit freundlichen Grüße, Erika Mustermann
                           """)
        ])

        let dialog = F.dialog(id: "1", timestamp: now, blocks: [
            F.inquiryBlock(id: "1-block",
                           sender: "Zentrale Zulassungsstelle für Drachenhaltung",
                           timestamp: now,
                           title: "Bestätigung Vorgangserstellung",
                           label: """
                           Sehr geehrte Frau Mustermann, hiermit möchte ich mich \
                           bei Ihnen für das freundliche Gespräch und die \
                           aufschlussreichen Informationen bedanken. \
                           Besonders habe ich mich über die angenehme \
                           Atmosphäre gefreut, in der ich einen guten Einblick \
                           in Ihr Unternehmen sowie die ausgeschriebene Position \
                           gewinnen konnte.
                           """),
            F.fileBlock(id: "1-file1", sender: "Zentrale Zulassungsstelle für Drachenhaltung",
                        timestamp: now, name: "file1.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "1-file2", sender: "Zentrale Zulassungsstelle für Drachenhaltung",
                        timestamp: now, name: "file2.jpg", mimetype: "image/jpeg", size: 1_024_000),
            F.fileBlock(id: "1-file3", sender: "Zentrale Zulassungsstelle für Drachenhaltung",
                        timestamp: now, name: "file3.jpg", mimetype: "image/jpeg", size: 10_024_000)
        ], answers: [replyEnvelope])

        MessageSummaryRow(dialog: dialog, monthSeparator: "September")
    }
}
