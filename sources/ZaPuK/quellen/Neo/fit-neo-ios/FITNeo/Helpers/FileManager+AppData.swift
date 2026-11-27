//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import OSLog

extension FileManager {
    func clearAppData() {
        let directories: [SearchPathDirectory] = [.documentDirectory, .cachesDirectory]
        for directory in directories {
            guard let dirURL = urls(for: directory, in: .userDomainMask).first else { continue }
            do {
                let contents = try contentsOfDirectory(at: dirURL,
                                                       includingPropertiesForKeys: nil,
                                                       options: [])
                for fileURL in contents {
                    try removeItem(at: fileURL)
                }
            } catch {
                Logger.logger(for: FileManager.self)
                    .error("Failed to clear \(directory.rawValue): \(error)")
            }
        }
    }

    func excludeFromBackup(_ url: URL) {
        guard fileExists(atPath: url.path) else { return }

        var url = url
        var values = URLResourceValues()
        values.isExcludedFromBackup = true
        do {
            try url.setResourceValues(values)
        } catch {
            Logger.logger(for: FileManager.self)
                .error("Failed to exclude \(url.lastPathComponent) from backup: \(error)")
        }
    }
}
