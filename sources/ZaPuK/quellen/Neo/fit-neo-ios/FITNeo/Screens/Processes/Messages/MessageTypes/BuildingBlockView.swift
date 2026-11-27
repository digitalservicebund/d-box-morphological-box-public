//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct BuildingBlockView: View {
    let block: NeoBuildingBlock
    let sectionHeader: String?
    var onAttachmentTapped: ((AttachmentType) -> Void)?

    var body: some View {
        if let sectionHeader {
            Text(sectionHeader)
                .textStyle(.bodyDefaultSemiBold)
                .padding(.top, 16)
                .padding(.bottom, 16)
        }

        switch block.content {
        case .message:
            MessageView(content: block.content)

        case .infobox:
            InfoboxView(content: block.content)

        case .button(let hyperlink):
            ButtonMessageView(hyperlink: hyperlink)

        case .inquiryMessage:
            InquiryMessageView(content: block.content)

        case .inquiryFile(let title, let label):
            if let title = title?.plain {
                Text(title)
                    .textStyle(.titleDefault)
            }
            if let label = label?.plain {
                Text(label)
                    .textStyle(.bodyDefaultRegular)
                    .frame(maxWidth: .infinity, alignment: .leading)
            }

        case .inquiryPoll:
            InquiryPollView(content: block.content)

        case .poll:
            EmptyView()

        case .file(let file, _, _),
             .image(let file, _, _, _, _, _),
             .video(let file, _, _, _, _),
             .audio(let file, _, _, _):
            let attachment = AttachmentType(url: file.uri, filename: file.name,
                                            filesize: Int(file.size ?? 0), mimetype: file.mimetype)
            NeoAttachmentView(attachment) { onAttachmentTapped?(attachment) }
                .padding(.bottom, 8)
        }
    }
}

struct BuildingBlockView_Previews: PreviewProvider {
    private typealias F = NeoDialogFixtures

    static var previews: some View {
        let now = F.ts()

        let poll = F.pollBlock(id: "1-poll-block",
                               sender: "Peter Muller",
                               timestamp: F.ts(),
                               multiSelect: false,
                               question: "This is a poll question",
                               answers: [
                                   "1": "Kleiner als 1 Meter",
                                   "2": "Zwischen 1 und 2 Meter",
                                   "3": "Größer als 2 Meter"
                               ])

        let infobox = F.infoboxBlock(id: "1-infobox", sender: "Peter Muller", timestamp: now,
                                     title: "Infobox",
                                     text: """
                                     Hier gibt es einen Link mit dazugehörigem Erklärungstext.
                                     """,
                                     linkLabel: "Link", linkUrl: "https://fitko.de")

        let button = F.buttonBlock(id: "1-button",
                                   sender: "1-sender",
                                   timestamp: now,
                                   label: "Sample button",
                                   url: "https://fitko.de")

        let inquiry = F.inquiryBlock(id: "1-block",
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
                                     """)

        let file = F.fileBlock(id: "1-file1", sender: "Zentrale Zulassungsstelle für Drachenhaltung",
                               timestamp: now, name: "file1.pdf", mimetype: "application/pdf", size: 256_000)

        let message = F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: now,
                                     text: """
                                     Sehr geehrte Damen und Herren, bei dem entsprechenden
                                     Drachen handelt es sich um einen ca. 65 cm großen,
                                     grünen Walddrachen handeln. Mit freundlichen Grüße, Erika Mustermann
                                     """)

        ScrollView {
            VStack(alignment: .leading) {
                BuildingBlockView(block: inquiry, sectionHeader: nil)
                BuildingBlockView(block: infobox, sectionHeader: nil)
                BuildingBlockView(block: message, sectionHeader: nil)
                BuildingBlockView(block: button, sectionHeader: nil)
                BuildingBlockView(block: file, sectionHeader: "Attachments")
                BuildingBlockView(block: poll, sectionHeader: nil)
            }
            .padding(16)
        }
    }
}
