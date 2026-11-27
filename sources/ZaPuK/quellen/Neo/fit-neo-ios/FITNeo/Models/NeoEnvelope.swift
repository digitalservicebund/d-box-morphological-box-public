//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import MatrixRustSDK

extension NeoEnvelope {
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

    func attachments() -> [AttachmentType] {
        blocks.compactMap { block in
            switch block.content {
            case .file(let file, _, _),
                 .image(let file, _, _, _, _, _),
                 .video(let file, _, _, _, _),
                 .audio(let file, _, _, _):
                AttachmentType(url: file.uri, filename: file.name,
                               filesize: Int(file.size ?? 0), mimetype: file.mimetype)
            default:
                nil
            }
        }
    }
}
