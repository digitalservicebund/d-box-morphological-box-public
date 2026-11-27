//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK
import OSLog

final class ProcessRoom: Identifiable, Equatable, Hashable {
    private let logger = Logger.logger(for: ProcessRoom.self)

    let room: MatrixRustSDK.Room?
    var roomInfo: RoomInfo?
    private var roomInfoUpdatesHandle: TaskHandle?

    let id: String
    var processName: String
    var authorityName: String
    var status: ProcessStatus?
    var statusInfo: ProcessStatusInfo?
    var encrypted = false

    init(room: MatrixRustSDK.Room) {
        self.room = room
        id = room.id()

        processName = room.displayName() ?? room.canonicalAlias() ?? room.id()
        authorityName = room.heroes().first?.displayName ?? room.topic() ?? L10n.Processes.unknownAuthority
        checkForMembership()
        checkForEncryption()
        startRoomInfoUpdatesListener()
        updateStatus()
    }

    init(id: String, name: String, authority: String, status: ProcessStatus? = nil, notice: String? = nil) {
        room = nil
        self.id = id
        if let status {
            self.status = status
            if let notice {
                statusInfo = ProcessStatusInfo(status: status, withNotice: notice)
            } else {
                statusInfo = ProcessStatusInfo(status: status)
            }
        }
        processName = name
        authorityName = authority
    }

    deinit {
        roomInfoUpdatesHandle = nil
    }

    static func == (lhs: ProcessRoom, rhs: ProcessRoom) -> Bool {
        lhs.id == rhs.id &&
            lhs.processName == rhs.processName &&
            lhs.authorityName == rhs.authorityName &&
            lhs.status == rhs.status &&
            lhs.encrypted == rhs.encrypted
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
    }

    private func checkForMembership() {
        guard let room else {
            return
        }

        let roomId = id

        if room.membership() == .invited {
            Task {
                do {
                    try await room.join()
                    logger.info("Auto-accepting invite to join room: \(roomId)")
                } catch {
                    logger.error("Failed to join room \(roomId): \(error)")
                }
            }
        } else if room.membership() == .left || room.membership() == .banned {
            Task {
                do {
                    try await room.leave()
                    try await room.forget()
                    logger.info("Leaving room: \(roomId)")
                } catch {
                    logger.error("Failed to leave+forget room \(roomId): \(error)")
                }
            }
        }
    }

    private func checkForEncryption() {
        guard room != nil else {
            return
        }

        Task { [weak self] in
            guard let self else { return }

            let isEncrypted = await room?.isEncrypted()

            if isEncrypted != nil, isEncrypted != false {
                encrypted = true
            }
        }
    }

    private func startRoomInfoUpdatesListener() {
        guard room != nil, roomInfoUpdatesHandle == nil else {
            return
        }

        Task { @MainActor in
            do {
                self.roomInfo = try await room?.roomInfo()
            } catch {
                logger.error("Failed to load room info: \(error)")
            }

            let listener = SDKListener<RoomInfo> { [weak self] roomInfo in
                Task { @MainActor in
                    self?.roomInfo = roomInfo

                    self?.processName = roomInfo.displayName ?? roomInfo.canonicalAlias ?? roomInfo.id
                    self?.authorityName = roomInfo.topic ?? L10n.Processes.unknownAuthority

                    self?.updateStatus()
                    self?.checkForMembership()
                    self?.checkForEncryption()
                }
            }
            self.roomInfoUpdatesHandle = room?.subscribeToRoomInfoUpdates(listener: listener)
        }
    }

    private func updateStatus() {
        // MARK: temporary, as we have not decided where status will be derived from

        if authorityName.lowercased().contains("erforderlich") {
            status = ProcessStatus.processActivityRequired
        } else if authorityName.lowercased().contains("abgeschlossen") {
            status = ProcessStatus.processCompleted
        } else if authorityName.lowercased().contains("neu") {
            status = ProcessStatus.processReceived
        }
        if let status {
            statusInfo = ProcessStatusInfo(status: status)
        }
    }

    var isCompleted: Bool {
        status == .processCompleted
    }

    var isEncrypted: Bool {
        encrypted
    }
}
