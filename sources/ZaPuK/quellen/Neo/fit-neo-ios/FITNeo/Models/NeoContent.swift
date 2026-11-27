//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK

extension NeoContent: @retroactive Identifiable {
    public var id: Int {
        hashValue
    }

    var isFileAttachment: Bool {
        switch self {
        case .file, .image, .video, .audio:
            true
        default:
            false
        }
    }
}
