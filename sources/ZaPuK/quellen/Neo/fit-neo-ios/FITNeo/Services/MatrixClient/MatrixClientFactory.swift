//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK

struct MatrixClientFactory {
    func makeClientBuilder(_ homeserverAddress: String,
                           _ documentsPath: String,
                           _ cachesPath: String) -> ClientBuilder {
        var builder = ClientBuilder()
            .userAgent(
                userAgent: UserAgentHelper.userAgentString()
            )
            .requestConfig(config: .init(retryLimit: 3,
                                         timeout: 30000,
                                         maxConcurrentRequests: nil,
                                         maxRetryTime: nil))
            .serverNameOrHomeserverUrl(serverNameOrUrl: homeserverAddress)
            .sessionPaths(dataPath: documentsPath,
                          cachePath: cachesPath)
            .slidingSyncVersionBuilder(
                versionBuilder: SlidingSyncVersionBuilder.native
            )

        // MARK: setup encryption

        builder = builder.autoEnableBackups(autoEnableBackups: true)
            .autoEnableCrossSigning(autoEnableCrossSigning: true)
            .backupDownloadStrategy(backupDownloadStrategy: .afterDecryptionFailure)
            .enableShareHistoryOnInvite(enableShareHistoryOnInvite: true)

        #if DEBUG
        builder = builder.disableSslVerification()
        #endif

        return builder
    }

    func makeClient(_ homeserverAddress: String,
                    _ documentsPath: String,
                    _ cachesPath: String) async throws -> ClientProtocol {
        let builder = makeClientBuilder(homeserverAddress,
                                        documentsPath,
                                        cachesPath)

        return try await builder.build()
    }
}
