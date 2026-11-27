//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI
import UIKit

enum NeoTextStyle {
    case bodyDefaultRegular
    case bodyDefaultSemiBold
    case bodySmallRegular
    case bodySmallSemiBold
    case headlineXLarge
    case labelDefault
    case labelSmall
    case titleDefault
    case sublineDefault
}

struct NeoTextStyleModifier: ViewModifier {
    let style: NeoTextStyle

    let bodyDefaultSize = 18.0
    let bodySmallSize = 16.0
    let headlineXLargeSize = 32.0
    let labelSize = 18.0
    let labelSmallSize = 16.0
    let titleSize = 21.0

    let defaultMinHeight = 24.0
    let largeMinHeight = 44.0

    func body(content: Content) -> some View {
        switch style {
        case .bodyDefaultRegular:
            content
                .font(
                    .custom(FontFamily.FiraSans.regular,
                            size: bodyDefaultSize,
                            relativeTo: .body)
                )
                .frame(minHeight: defaultMinHeight)

        case .bodyDefaultSemiBold:
            content
                .font(
                    .custom(FontFamily.FiraSans.semiBold,
                            size: bodyDefaultSize,
                            relativeTo: .body)
                )
                .frame(minHeight: defaultMinHeight)

        case .bodySmallRegular:
            content
                .font(
                    .custom(FontFamily.FiraSans.regular,
                            size: bodySmallSize,
                            relativeTo: .body)
                )
                .frame(minHeight: defaultMinHeight)

        case .bodySmallSemiBold:
            content
                .font(
                    .custom(FontFamily.FiraSans.semiBold,
                            size: bodySmallSize,
                            relativeTo: .body)
                )
                .frame(minHeight: defaultMinHeight)

        case .headlineXLarge:
            content
                .font(
                    .custom(FontFamily.FiraSans.medium,
                            size: headlineXLargeSize,
                            relativeTo: .largeTitle)
                )
                .fontWeight(.medium)
                .frame(minHeight: largeMinHeight)

        case .labelDefault:
            content
                .font(
                    .custom(FontFamily.FiraSans.medium,
                            size: labelSize,
                            relativeTo: .caption)
                )
                .frame(minHeight: defaultMinHeight)

        case .labelSmall:
            content
                .font(
                    .custom(FontFamily.FiraSans.medium,
                            size: labelSmallSize,
                            relativeTo: .caption)
                )
                .frame(minHeight: defaultMinHeight)

        case .titleDefault:
            content
                .font(
                    .custom(FontFamily.FiraSans.semiBold,
                            size: titleSize,
                            relativeTo: .title)
                )
                .frame(minHeight: defaultMinHeight)

        // MARK: this is the same as labelDefault

        case .sublineDefault:
            content
                .font(
                    .custom(FontFamily.FiraSans.medium,
                            size: labelSize,
                            relativeTo: .caption)
                )
                .frame(minHeight: defaultMinHeight)
        }
    }
}

extension View {
    func textStyle(_ style: NeoTextStyle) -> some View {
        modifier(NeoTextStyleModifier(style: style))
    }
}
