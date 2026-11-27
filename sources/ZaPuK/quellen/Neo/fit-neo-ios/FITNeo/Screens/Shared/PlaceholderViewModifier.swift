//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct PlaceholderViewModifier: ViewModifier {
    let placeholder: String
    let text: String

    func body(content: Content) -> some View {
        content.overlay(alignment: .topLeading) {
            if text.isEmpty {
                Text(placeholder)
                    .textStyle(.bodyDefaultRegular)
                    .foregroundStyle(Asset.Theme.textMuted.swiftUIColor)
                    .padding(.top, 8)
                    .padding(.leading, 8)
                    .allowsHitTesting(false)
            }
        }
    }
}

extension View {
    func placeholder(_ text: String, for value: String) -> some View {
        modifier(PlaceholderViewModifier(placeholder: text, text: value))
    }
}
