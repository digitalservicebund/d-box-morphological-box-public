// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

enum MessageServiceError: Error, Equatable {
    case unsupportedClientType
    case invalidContent
}

struct MessageServiceClient {
    var loadCase: @Sendable (_ caseId: String) async throws -> NeoCase
    var sendAnswer: @Sendable (_ caseId: String, _ dialogId: String, _ blocks: [NeoContent]) async throws -> String
}

extension MessageServiceClient: DependencyKey {
    static let liveValue = MessageServiceClient(loadCase: { caseId in
                                                    let service = try await createMessageService()
                                                    return try await service.loadCase(caseId: caseId)
                                                },
                                                sendAnswer: { caseId, dialogId, blocks in
                                                    let service = try await createMessageService()
                                                    return try await service.sendAnswer(caseId: caseId,
                                                                                        dialogId: dialogId,
                                                                                        blocks: blocks)
                                                })

    static let testValue = MessageServiceClient(loadCase: { _ in NeoCase(id: "test-case-1", dialogs: []) },
                                                sendAnswer: { _, _, _ in "test-event-id" })

    private static func createMessageService() async throws -> MessageService {
        let clientProtocol: ClientProtocol = try await MainActor.run {
            try AppState.shared.getMatrixClient().matrixClient()
        }

        guard let client = clientProtocol as? Client else {
            throw MessageServiceError.unsupportedClientType
        }

        return messageService(client: client)
    }
}

extension DependencyValues {
    var messageServiceClient: MessageServiceClient {
        get { self[MessageServiceClient.self] }
        set { self[MessageServiceClient.self] = newValue }
    }
}
