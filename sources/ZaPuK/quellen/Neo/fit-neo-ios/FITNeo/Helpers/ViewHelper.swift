//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import SwiftUI

extension View {
    @ViewBuilder
    func glassedEffect(in shape: some Shape = Capsule(), interactive: Bool = false, clear: Bool = false) -> some View {
        if #available(iOS 26.0, *) {
            let glass: Glass = clear ? .clear : .regular
            self.glassEffect(interactive ? glass.interactive() : glass, in: shape)
        } else {
            background(
                shape
                    .fill(.ultraThinMaterial)
                    .overlay(
                        LinearGradient(colors: [.white.opacity(0.3), .clear],
                                       startPoint: .topLeading,
                                       endPoint: .bottomTrailing)
                    )
                    .overlay(shape.stroke(.white.opacity(0.2), lineWidth: 1))
            )
        }
    }
}

enum ViewHelper {
    static func initialsFrom(displayName name: String) -> String {
        let trimmedName = name.trimmingCharacters(in: .whitespacesAndNewlines)

        guard !trimmedName.isEmpty else {
            return ""
        }

        let allowed = CharacterSet.alphanumerics.union(.whitespaces)
        let sanitized = String(trimmedName.unicodeScalars.map { allowed.contains($0) ? Character($0) : " " })

        let words = sanitized.components(separatedBy: .whitespaces).filter { !$0.isEmpty }

        guard !words.isEmpty else {
            return ""
        }

        if words.count == 1 {
            // check for MXID format
            if trimmedName.contains("@") {
                // extract the localpart and strip non-alphanumerics
                let localpart = trimmedName.split(separator: "@").first ?? ""
                let cleaned = localpart.unicodeScalars.filter { allowed.contains($0) }
                return String(String(String.UnicodeScalarView(cleaned)).prefix(2)).uppercased()
            }

            return String(words[0].prefix(1)).uppercased()
        } else {
            if let first = words.first, let last = words.last {
                let firstInitial = first.prefix(1)
                let lastInitial = last.prefix(1)
                return (firstInitial + lastInitial).uppercased()
            } else {
                return ""
            }
        }
    }

    static func dismissKeyboard() {
        UIApplication.shared.sendAction(#selector(UIResponder.resignFirstResponder),
                                        to: nil, from: nil, for: nil)
    }
}
