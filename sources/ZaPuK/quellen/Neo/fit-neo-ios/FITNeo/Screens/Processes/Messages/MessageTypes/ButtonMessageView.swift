//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct ButtonMessageView: View {
    let hyperlink: NeoHyperlink

    @State private var openBrowser = Store(initialState: OpenBrowserFeature.State()) {
        OpenBrowserFeature()
    }

    var body: some View {
        Button {
            openBrowser.send(.openWebsite(hyperlink.url))
        } label: {
            Text(hyperlink.label.plain ?? hyperlink.url)
        }
        .modifier(NeoPrimaryButtonStyleModifier())
        .padding(.top, 16)
        .padding(.bottom, 16)
        .accessibilityIdentifier("ButtonMessageView")
        .alert(L10n.Alert.errorOccurred,
               isPresented: Binding(get: { openBrowser.browserNotSupported },
                                    set: { _ in openBrowser.send(.dismissBrowserAlert) }),
               actions: {
                   Button(L10n.Alert.ok) {
                       openBrowser.send(.dismissBrowserAlert)
                   }
                   .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
               }, message: {
                   Text(L10n.Screen.Help.browserNotSupported)
               })
    }
}
