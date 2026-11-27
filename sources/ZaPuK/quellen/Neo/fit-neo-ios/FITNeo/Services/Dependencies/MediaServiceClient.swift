//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import MatrixRustSDK

struct MediaServiceClient {
    var download: @Sendable (AttachmentType) async throws -> URL
}

extension MediaServiceClient: DependencyKey {
    static let liveValue = MediaServiceClient { attachment in
        let client: ClientProtocol = try await MainActor.run {
            try AppState.shared.getMatrixClient().matrixClient()
        }
        let mediaSource = try MediaSource.fromUrl(url: attachment.url)
        let handle = try await client.getMediaFile(mediaSource: mediaSource,
                                                   filename: attachment.filename,
                                                   mimeType: attachment.mimetype ?? "application/octet-stream",
                                                   useCache: true,
                                                   tempDir: nil)
        // Copy to caches before the handle goes out of scope and deletes the temp file.
        let tempURL = try URL(fileURLWithPath: handle.path())
        let destURL = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask)[0]
            .appendingPathComponent(attachment.filename)
        try? FileManager.default.removeItem(at: destURL)
        try FileManager.default.copyItem(at: tempURL, to: destURL)
        return destURL
    }

    static let testValue = MediaServiceClient { _ in
        throw AppStateError.matrixClientNotSet
    }
}

extension DependencyValues {
    var mediaServiceClient: MediaServiceClient {
        get { self[MediaServiceClient.self] }
        set { self[MediaServiceClient.self] = newValue }
    }
}
