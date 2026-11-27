//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Dependencies
import Foundation

enum ProcessesTestScenario: String, CaseIterable {
    case empty
    case openOnly
    case completedOnly
    case mixed
    case allStatuses
    case manyProcesses

    var mockProcesses: [ProcessRoom] {
        switch self {
        case .empty:
            return []

        case .openOnly:
            return [
                ProcessRoom(id: "open-1",
                            name: "License Application",
                            authority: "City Hall",
                            status: .processSent),
                ProcessRoom(id: "open-2",
                            name: "Permit Request",
                            authority: "Planning Department",
                            status: .processReceived)
            ]

        case .completedOnly:
            return [
                ProcessRoom(id: "done-1",
                            name: "Completed License",
                            authority: "License Office",
                            status: .processCompleted),
                ProcessRoom(id: "done-2",
                            name: "Finished Permit",
                            authority: "Permit Bureau",
                            status: .processCompleted)
            ]

        case .mixed:
            return [
                ProcessRoom(id: "open-1",
                            name: "Open Application",
                            authority: "Application Center",
                            status: .processSent),
                ProcessRoom(id: "open-2",
                            name: "Pending Request",
                            authority: "Request Office",
                            status: .processReceived),
                ProcessRoom(id: "done-1",
                            name: "Finished Task",
                            authority: "Task Bureau",
                            status: .processCompleted),
                ProcessRoom(id: "done-2",
                            name: "Archived Process",
                            authority: "Archive Office",
                            status: .processCompleted)
            ]

        case .allStatuses:
            return [
                ProcessRoom(id: "p1",
                            name: "Sent Process",
                            authority: "Office A",
                            status: .processSent),
                ProcessRoom(id: "p2",
                            name: "Received Process",
                            authority: "Office B",
                            status: .processReceived),
                ProcessRoom(id: "p3",
                            name: "Review Process",
                            authority: "Office C",
                            status: .processUnderReview),
                ProcessRoom(id: "p4",
                            name: "Activity Process",
                            authority: "Office D",
                            status: .processActivityRequired),
                ProcessRoom(id: "p5",
                            name: "Completed Process",
                            authority: "Office E",
                            status: .processCompleted)
            ]

        case .manyProcesses:
            var processes: [ProcessRoom] = []
            for idx in 1...7 {
                processes.append(
                    ProcessRoom(id: "open-\(idx)",
                                name: "Open Process \(idx)",
                                authority: "Authority \(idx)",
                                status: .processSent)
                )
            }
            for idx in 1...3 {
                processes.append(
                    ProcessRoom(id: "done-\(idx)",
                                name: "Done Process \(idx)",
                                authority: "Authority \(idx)",
                                status: .processCompleted)
                )
            }
            return processes
        }
    }

    func makeState() -> ProcessesViewFeature.State {
        ProcessesViewFeature.State(loadState: .loaded,
                                   pendingActivities: 0,
                                   allProcesses: mockProcesses)
    }
}
