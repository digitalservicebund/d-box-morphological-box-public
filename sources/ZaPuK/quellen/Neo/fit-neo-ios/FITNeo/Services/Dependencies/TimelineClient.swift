//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK

private actor TimelineStore {
    private var timelines: [String: Timeline] = [:]

    func getOrCreate(roomId: String, room: MatrixRustSDK.Room) async throws -> Timeline {
        if let existing = timelines[roomId] { return existing }
        let configuration = TimelineConfiguration(focus: .live(hideThreadedEvents: false),
                                                  filter: .all,
                                                  internalIdPrefix: nil,
                                                  dateDividerMode: .monthly,
                                                  trackReadReceipts: .allEvents,
                                                  reportUtds: false)
        let timeline = try await room.timelineWithConfiguration(configuration: configuration)
        timelines[roomId] = timeline
        return timeline
    }
}

final class TimelineClientListener: TimelineListener {
    private let continuation: AsyncStream<[TimelineDiff]>.Continuation

    init(continuation: AsyncStream<[TimelineDiff]>.Continuation) {
        self.continuation = continuation
    }

    func onUpdate(diff: [TimelineDiff]) {
        continuation.yield(diff)
    }
}

struct TimelineClient {
    var observe: (ProcessRoom) -> AsyncStream<[TimelineDiff]>
}

extension TimelineClient: DependencyKey {
    static let liveValue: TimelineClient = {
        let store = TimelineStore()

        return TimelineClient { process in
            AsyncStream<[TimelineDiff]> { continuation in
                guard let room = process.room else {
                    continuation.finish()
                    return
                }

                let listener = TimelineClientListener(continuation: continuation)
                let task = Task {
                    do {
                        let timeline = try await store.getOrCreate(roomId: process.id, room: room)
                        let handle = await timeline.addListener(listener: listener)
                        defer { withExtendedLifetime(handle) { } }
                        try? await Task.sleep(nanoseconds: UInt64.max)
                    } catch {
                        continuation.finish()
                    }
                }

                continuation.onTermination = { _ in task.cancel() }
            }
        }
    }()

    static let testValue = TimelineClient { _ in AsyncStream { $0.finish() } }
}

extension DependencyValues {
    var timelineClient: TimelineClient {
        get { self[TimelineClient.self] }
        set { self[TimelineClient.self] = newValue }
    }
}
