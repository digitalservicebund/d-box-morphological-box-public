//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK

enum NeoDialogFixtures {
    static func ts(_ date: Date = .now) -> UInt64 {
        UInt64(date.timeIntervalSince1970 * 1000)
    }

    static func pollBlock(id: String,
                          sender: String,
                          timestamp: UInt64,
                          multiSelect: Bool,
                          question: String,
                          answers: KeyValuePairs<String, String>) -> NeoBuildingBlock {
        let pollContent = NeoPollContent(multiSelect: multiSelect,
                                         question: NeoText(plain: question, html: nil),
                                         answers: answers.map { id, text in
                                             NeoPollAnswer(id: id,
                                                           text: NeoText(plain: text, html: nil))
                                         })
        return NeoBuildingBlock(id: id,
                                sender: sender,
                                timestamp: timestamp,
                                content: .inquiryPoll(poll: pollContent))
    }

    static func pollResponseBlock(id: String,
                                  sender: String,
                                  timestamp: UInt64,
                                  answerId: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content: .poll(selections: [answerId], custom: nil, replyTo: nil))
    }

    static func pollCustomResponseBlock(id: String,
                                        sender: String,
                                        timestamp: UInt64,
                                        answer: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content: .poll(selections: [], custom: answer, replyTo: nil))
    }

    static func messageBlock(id: String,
                             sender: String,
                             timestamp: UInt64,
                             text: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content: .message(text: NeoText(plain: text, html: nil),
                                           replyTo: nil))
    }

    static func infoboxBlock(id: String,
                             sender: String,
                             timestamp: UInt64,
                             title: String,
                             text: String,
                             linkLabel: String,
                             linkUrl: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content:
                         .infobox(title: NeoText(plain: title, html: nil),
                                  infobox: NeoText(plain: text, html: nil),
                                  hyperlink: NeoHyperlink(label: NeoText(plain: linkLabel, html: nil),
                                                          url: linkUrl)))
    }

    static func fileBlock(id: String,
                          sender: String,
                          timestamp: UInt64,
                          name: String,
                          mimetype: String,
                          size: UInt64) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content: .file(file: NeoFileInfo(uri: "https://test.url",
                                                          name: name,
                                                          mimetype: mimetype,
                                                          size: size),
                                        title: NeoText(plain: name, html: nil),
                                        replyTo: nil))
    }

    static func inquiryBlock(id: String,
                             sender: String,
                             timestamp: UInt64,
                             title: String,
                             label: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content: .inquiryMessage(title: NeoText(plain: title, html: nil),
                                                  label: NeoText(plain: label, html: nil)))
    }

    static func buttonBlock(id: String,
                            sender: String,
                            timestamp: UInt64,
                            label: String,
                            url: String) -> NeoBuildingBlock {
        NeoBuildingBlock(id: id,
                         sender: sender,
                         timestamp: timestamp,
                         content:
                         .button(
                             hyperlink: NeoHyperlink(label: NeoText(plain: label, html: nil), url: url)
                         ))
    }

    static func dialog(id: String,
                       envId: String? = nil,
                       timestamp: UInt64,
                       blocks: [NeoBuildingBlock],
                       answers: [NeoEnvelope] = []) -> NeoDialogProxy {
        NeoDialogProxy(
            NeoDialog(id: id,
                      message: NeoEnvelope(id: envId ?? "\(id)-env",
                                           timestamp: timestamp,
                                           blocks: blocks),
                      answers: answers)
        )
    }

    static func envelope(id: String,
                         timestamp: UInt64,
                         blocks: [NeoBuildingBlock]) -> NeoEnvelope {
        NeoEnvelope(id: id, timestamp: timestamp, blocks: blocks)
    }
}
