//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK
import SwiftUI

struct NeoDialogProxy: Equatable, Hashable {
    let rustValue: NeoDialog
    var status: MessageStatus = .received
    var unread = true
    var dueDate: Date?

    init(_ dialog: NeoDialog) {
        rustValue = dialog
        status = requiresAnswer ? .requiresAnswer : .received
    }

    var id: String {
        rustValue.id
    }

    var message: NeoEnvelope {
        rustValue.message
    }

    var answers: [NeoEnvelope] {
        rustValue.answers
    }

    var lastAnswer: NeoEnvelope? {
        rustValue.answers.last
    }

    var blocks: [NeoBuildingBlock] {
        rustValue.message.blocks
    }

    var timestamp: UInt64 {
        rustValue.message.timestamp
    }

    var hasAnswers: Bool {
        !rustValue.answers.isEmpty
    }

    var requiresAnswer: Bool {
        // MARK: Check if any block is of inquiry type - in the future should also check if answer was given

        blocks.contains { block in
            if case .inquiryMessage = block.content { return true }
            if case .inquiryPoll = block.content { return true }
            if case .inquiryFile = block.content { return true }
            return false
        }
    }

    var shouldShowReply: Bool {
        // MARK: Should also check for an actual existing answer in the future

        requiresAnswer && blocks.contains { block in
            if case .inquiryMessage = block.content { return true }
            if case .inquiryPoll = block.content { return true }
            if case .inquiryFile = block.content { return true }
            return false
        }
    }

    var inquiryContent: NeoContent? {
        /// There may only be at most one Inquiry event per Multi-Event.

        blocks.compactMap { block in
            switch block.content {
            case .inquiryMessage, .inquiryPoll, .inquiryFile:
                block.content
            default:
                nil
            }
        }.first
    }

    func formattedTimestamp() -> String {
        Date(timeIntervalSince1970: TimeInterval(timestamp / 1000)).formatted(
            .dateTime
                .day(.twoDigits)
                .month(.twoDigits)
                .year()
                .hour(.twoDigits(amPM: .omitted))
                .minute(.twoDigits)
        )
    }

    func monthName() -> String {
        Date(timeIntervalSince1970: TimeInterval(timestamp / 1000)).formatted(
            .dateTime
                .day(.omitted)
                .month(.wide)
                .year(.omitted)
                .hour(.omitted)
                .minute(.omitted)
        )
    }

    /// not sure if all reply summaries will be text-based but good enough for now
    func lastAnswerSummary() -> String {
        for block in lastAnswer?.blocks ?? [] {
            return contentTextSummary(block.content) ?? L10n.Screen.Message.Reply.unsupportedContentType
        }
        return L10n.Alert.errorOccurred
    }

    func contentTextSummary(_ content: NeoContent) -> String? {
        switch content {
        case .message(let text, _):
            return text.plain

        case .poll(let selections, let custom, _):
            if let custom, !custom.isEmpty {
                return custom
            }

            guard case .inquiryPoll(let poll) = inquiryContent else { return nil }

            let texts = selections.compactMap { id in
                poll.answers.first { $0.id == id }?.text.plain
            }
            return texts.isEmpty ? nil : texts.joined(separator: ", ")

        default:
            return L10n.Screen.Message.Reply.unsupportedContentType
        }
    }
}
