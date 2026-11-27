//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct NeoButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
            .padding(.horizontal, 20)
            .padding(.vertical, 6)
            .textStyle(.labelDefault)
    }
}

struct NeoButtonStyleModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .glassedEffect(in: Capsule(), interactive: true)
            .frame(minHeight: 44)
    }
}

struct NeoGlassButtonStyle: ButtonStyle {
    let horizontalPadding: CGFloat
    let verticalPadding: CGFloat

    init(horizontalPadding: CGFloat = 8, verticalPadding: CGFloat = 10) {
        self.horizontalPadding = horizontalPadding
        self.verticalPadding = verticalPadding
    }

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
            .textStyle(.labelDefault)
            .padding(.horizontal, horizontalPadding)
            .padding(.vertical, verticalPadding)
            .opacity(configuration.isPressed ? 0.7 : 1.0)
    }
}

struct NeoRoundGlassButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
            .textStyle(.labelDefault)
            .opacity(configuration.isPressed ? 0.7 : 1.0)
    }
}

struct NeoHighlightStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .overlay(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Asset.Theme.actionsDefault.swiftUIColor.opacity(configuration.isPressed ? 0.1 : 0))
            )
    }
}

struct NeoPrimaryButtonStyleModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .textStyle(.labelDefault)
            .foregroundColor(Asset.Theme.actionsOnDefault.swiftUIColor)
            .frame(maxWidth: .infinity)
            .padding(.vertical, 12)
            .background(Asset.Theme.actionsDefault.swiftUIColor)
            .clipShape(Capsule())
            .contentShape(Capsule())
    }
}

struct NeoSecondaryButtonStyleModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .textStyle(.labelDefault)
            .foregroundColor(Asset.Theme.actionsDefault.swiftUIColor)
            .padding(.vertical, 12)
            .background(
                Asset.Theme.layoutBackgroundHued.swiftUIColor
            )
            .glassedEffect(in: Capsule())
            .clipShape(Capsule())
            .contentShape(Capsule())
    }
}

extension Button {
    func neoStyle() -> some View {
        buttonStyle(NeoButtonStyle())
            .modifier(NeoButtonStyleModifier())
    }

    func neoGlassStyle(horizontalPadding: CGFloat = 8, verticalPadding: CGFloat = 10) -> some View {
        buttonStyle(NeoGlassButtonStyle(horizontalPadding: horizontalPadding,
                                        verticalPadding: verticalPadding))
            .modifier(NeoButtonStyleModifier())
    }

    func neoRoundGlassStyle() -> some View {
        buttonStyle(NeoRoundGlassButtonStyle())
            .modifier(NeoButtonStyleModifier())
    }
}
