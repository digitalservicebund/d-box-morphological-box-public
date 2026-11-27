//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SFSafeSymbols
import SwiftUI

/// Overlays a fading gradient and a chevron at the bottom of a scrollable view
/// whenever there is more content to scroll towards. Tracks its own scroll state,
/// so it can be applied to any ScrollView without extra wiring.
struct ScrollMoreIndicatorModifier: ViewModifier {
    var background: Color = Asset.Theme.inputBackground.swiftUIColor
    var tint: Color = Asset.Theme.actionsDefault.swiftUIColor

    @State private var hasMore = false

    func body(content: Content) -> some View {
        content
            .onScrollGeometryChange(for: Bool.self) { geometry in
                let maxOffset = geometry.contentSize.height
                    - geometry.containerSize.height
                    - geometry.contentInsets.top
                    - geometry.contentInsets.bottom
                return maxOffset > 1 && geometry.contentOffset.y < maxOffset - 1
            } action: { _, value in
                hasMore = value
            }
            .overlay(alignment: .bottom) {
                ZStack(alignment: .bottom) {
                    LinearGradient(colors: [background.opacity(0), background.opacity(0.95)],
                                   startPoint: .top,
                                   endPoint: .bottom)
                        .frame(height: 32)
                    Image(systemSymbol: .chevronCompactDown)
                        .foregroundStyle(tint)
                        .padding(.bottom, 4)
                }
                .allowsHitTesting(false)
                .accessibilityHidden(true)
                .opacity(hasMore ? 1 : 0)
                .animation(.easeInOut(duration: 0.2), value: hasMore)
            }
    }
}

extension View {
    func scrollMoreIndicator(background: Color = Asset.Theme.inputBackground.swiftUIColor,
                             tint: Color = Asset.Theme.actionsDefault.swiftUIColor) -> some View {
        modifier(ScrollMoreIndicatorModifier(background: background, tint: tint))
    }
}
