//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

struct AttachmentType: Hashable, Sendable {
    let url: String
    let filename: String
    let filesize: Int
    let mimetype: String?

    init(url: String, filename: String, filesize: Int, mimetype: String? = nil) {
        self.url = url
        self.filename = filename
        self.filesize = filesize
        self.mimetype = mimetype
    }
}
