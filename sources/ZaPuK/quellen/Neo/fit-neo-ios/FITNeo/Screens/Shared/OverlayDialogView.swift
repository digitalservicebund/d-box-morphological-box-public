//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct OverlayDialogView<Content: View>: View {
    var onBackgroundTap: () -> Void = { }
    @ViewBuilder let content: () -> Content

    var body: some View {
        Color.black.opacity(0.2)
            .ignoresSafeArea()
            .contentShape(Rectangle())
            .onTapGesture { onBackgroundTap() }
            .overlay {
                content()
                    .padding(.horizontal, 46)
                    .contentShape(Rectangle())
                    .onTapGesture { }
            }
            .transition(.opacity)
    }
}
