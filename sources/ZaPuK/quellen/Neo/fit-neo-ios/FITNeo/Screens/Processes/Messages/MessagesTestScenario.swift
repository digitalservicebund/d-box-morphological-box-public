//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK

private typealias F = NeoDialogFixtures

enum MessagesTestScenario: String, CaseIterable {
    case empty
    case singleMessage
    case singlePollMessage
    case singlePollMessageAnswered
    case multipleMessages
    case messagesWithDueDate

    var mockDialogs: [NeoDialogProxy] {
        switch self {
        case .empty:
            []

        case .singleMessage:
            [
                F.dialog(id: "msg-1",
                         envId: "msg-1-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.inquiryBlock(id: "msg-1-block",
                                            sender: "Central Licensing Office",
                                            timestamp: F.ts(),
                                            title: "Application Confirmation",
                                            label: "Your application has been received and is being processed.")
                         ])
            ]

        case .singlePollMessage:
            [
                F.dialog(id: "msg-poll-1",
                         envId: "msg-poll-1-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.pollBlock(id: "msg-poll-1-block",
                                         sender: "Central Licensing Office",
                                         timestamp: F.ts(),
                                         multiSelect: false,
                                         question: "How tall was the dragon you observed?",
                                         answers: [
                                             "1": "Less than 1 meter",
                                             "2": "Between 1 and 2 meters",
                                             "3": "Greater than 2 meters",
                                             "dev.fitko.neo.poll.custom.v1": "Other (please specify)"
                                         ])
                         ])
            ]

        case .singlePollMessageAnswered:
            [
                F.dialog(id: "msg-poll-1",
                         envId: "msg-poll-1-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.pollBlock(id: "msg-poll-1-block",
                                         sender: "Central Licensing Office",
                                         timestamp: F.ts(),
                                         multiSelect: false,
                                         question: "How tall was the dragon you observed?",
                                         answers: [
                                             "1": "Less than 1 meter",
                                             "2": "Between 1 and 2 meters",
                                             "3": "Greater than 2 meters"
                                         ])
                         ],
                         answers: [
                             F.envelope(id: "msg-poll-1-reply",
                                        timestamp: F.ts(),
                                        blocks: [
                                            F.pollResponseBlock(id: "msg-poll-1-reply-block",
                                                                sender: "Eva Mustermann",
                                                                timestamp: F.ts(),
                                                                answerId: "2")
                                        ])
                         ])
            ]

        case .multipleMessages:
            [
                F.dialog(id: "msg-1",
                         envId: "msg-1-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.inquiryBlock(id: "msg-1-block",
                                            sender: "Central Licensing Office",
                                            timestamp: F.ts(),
                                            title: "Application Received",
                                            label: "Your application has been received. We will review it shortly.")
                         ]),
                F.dialog(id: "msg-2",
                         envId: "msg-2-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.inquiryBlock(id: "msg-2-block",
                                            sender: "Central Licensing Office",
                                            timestamp: F.ts(),
                                            title: "Documents Required",
                                            label: "Please submit these documents to proceed with the application."),
                             F.fileBlock(id: "msg-2-file1",
                                         sender: "Central Licensing Office",
                                         timestamp: F.ts(),
                                         name: "file1.pdf",
                                         mimetype: "application/pdf",
                                         size: 256_000),
                             F.fileBlock(id: "msg-2-file2",
                                         sender: "Central Licensing Office",
                                         timestamp: F.ts(),
                                         name: "file2.jpg",
                                         mimetype: "image/jpeg",
                                         size: 1_024_000)
                         ]),
                F.dialog(id: "msg-3",
                         envId: "msg-3-env",
                         timestamp: F.ts(),
                         blocks: [
                             F.inquiryBlock(id: "msg-3-block",
                                            sender: "Eva Mustermann",
                                            timestamp: F.ts(),
                                            title: "Documents Submitted",
                                            label: "I have attached the requested documents for your review."),
                             F.fileBlock(id: "msg-3-file1",
                                         sender: "Eva Mustermann",
                                         timestamp: F.ts(),
                                         name: "file1.pdf",
                                         mimetype: "application/pdf",
                                         size: 256_000)
                         ],
                         answers: [
                             F.envelope(id: "msg-3-reply",
                                        timestamp: F.ts(),
                                        blocks: [
                                            F.messageBlock(id: "msg-3-reply-block",
                                                           sender: "Central Licensing Office",
                                                           timestamp: F.ts(),
                                                           text: "Thank you, we have received your documents.")
                                        ])
                         ])
            ]

        case .messagesWithDueDate:
            {
                var entry = F.dialog(id: "msg-1",
                                     envId: "msg-1-env",
                                     timestamp: F.ts(),
                                     blocks: [
                                         F.inquiryBlock(id: "msg-1-block",
                                                        sender: "Central Licensing Office",
                                                        timestamp: F.ts(),
                                                        title: "Action Required",
                                                        label: "Please submit the following documents by the deadline.")
                                     ])
                entry.dueDate = Calendar.current.date(byAdding: .day, value: 15, to: .now)
                return [entry]
            }()
        }
    }

    func makeState(process: ProcessRoom) -> MessagesOverviewFeature.State {
        var state = MessagesOverviewFeature.State(process: process,
                                                  _previewEntries: mockDialogs)
        if self == .messagesWithDueDate {
            state.readMessageIds.insert("msg-1")
        }
        state.isContentLoading = false
        return state
    }
}
