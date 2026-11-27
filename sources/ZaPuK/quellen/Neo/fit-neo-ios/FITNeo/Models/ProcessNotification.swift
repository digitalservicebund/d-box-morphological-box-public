//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation

enum NotificationStatus {
    case read
    case unread
}

struct ProcessNotification: Identifiable, Equatable {
    let id: String
    let readStatus: NotificationStatus
    let title: String
    let detail: String
    let dueDate: Date?

    final class Builder {
        private var id: String?
        private var readStatus: NotificationStatus = .unread
        private var title: String?
        private var detail: String?
        private var dueDate: Date?

        @discardableResult
        func id(_ value: String) -> Builder {
            id = value
            return self
        }

        @discardableResult
        func readStatus(_ value: NotificationStatus) -> Builder {
            readStatus = value
            return self
        }

        @discardableResult
        func title(_ value: String) -> Builder {
            title = value
            return self
        }

        @discardableResult
        func detail(_ value: String) -> Builder {
            detail = value
            return self
        }

        @discardableResult
        func dueDate(_ value: Date?) -> Builder {
            dueDate = value
            return self
        }

        func build() throws -> ProcessNotification {
            guard let id else {
                throw BuilderError.missingField("processId")
            }
            guard let title else {
                throw BuilderError.missingField("title")
            }
            guard let detail else {
                throw BuilderError.missingField("detail")
            }

            return ProcessNotification(id: id,
                                       readStatus: readStatus,
                                       title: title,
                                       detail: detail,
                                       dueDate: dueDate)
        }

        enum BuilderError: Error {
            case missingField(String)
        }
    }
}
