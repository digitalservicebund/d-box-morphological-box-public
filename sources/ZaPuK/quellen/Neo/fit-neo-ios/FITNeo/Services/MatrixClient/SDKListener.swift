//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
// SPDX-FileCopyrightText: 2025 New Vector Ltd.
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK

/// A helper class that can be passed as this listener for SDK callbacks.
///
/// To use this you'll need to add a conformance to the required listener
/// protocol with a specialisation for the type it listens for.
final class SDKListener<T: Sendable> {
    private let onUpdateClosure: @Sendable (T) -> Void

    /// Creates a new listener.
    /// - Parameter onUpdateClosure: A closure that will be called whenever a new value is available.
    init(_ onUpdateClosure: @escaping @Sendable (T) -> Void) {
        self.onUpdateClosure = onUpdateClosure
    }
}

extension SDKListener: SyncServiceStateObserver where T == SyncServiceState {
    func onUpdate(state: SyncServiceState) { onUpdateClosure(state) }
}

extension SDKListener: RoomListServiceStateListener where T == RoomListServiceState {
    func onUpdate(state: RoomListServiceState) { onUpdateClosure(state) }
}

extension SDKListener: RoomListServiceSyncIndicatorListener where T == RoomListServiceSyncIndicator {
    func onUpdate(syncIndicator: RoomListServiceSyncIndicator) { onUpdateClosure(syncIndicator) }
}

extension SDKListener: RoomInfoListener where T == RoomInfo {
    func call(roomInfo: RoomInfo) {
        onUpdateClosure(roomInfo)
    }
}

extension SDKListener: RoomListEntriesListener where T == [RoomListEntriesUpdate] {
    func onUpdate(roomEntriesUpdate: [RoomListEntriesUpdate]) {
        onUpdateClosure(roomEntriesUpdate)
    }
}

extension SDKListener: TimelineListener where T == [TimelineDiff] {
    func onUpdate(diff: [TimelineDiff]) {
        onUpdateClosure(diff)
    }
}
