//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Foundation
import SFSafeSymbols
import SwiftUI

struct ProcessesHelpView: View {
    let onOpenHelpTapped: () -> Void
    @Environment(\.openURL) private var openURL
    @State private var callsNotSupported = false

    var body: some View {
        VStack(alignment: .leading) {
            Group {
                ZStack(alignment: .top) {
                    Rectangle()
                        .clipShape(RoundedRectangle(cornerRadius: 16, style: .circular))
                        .foregroundColor(Asset.Theme.layoutBackgroundHued.swiftUIColor)
                        .frame(maxWidth: 380)
                        .frame(height: 200)
                        .padding(.top, 16)
                        .padding(.bottom, 8)

                    Image(asset: Asset.Assets.Images.userHelp)
                        .resizable()
                        .scaledToFill()
                        .frame(maxWidth: 380)
                        .frame(height: 184, alignment: .top)
                        .clipped()
                        .padding(.top, 32)
                        .padding(.horizontal, 16)
                }
                .frame(maxWidth: .infinity, alignment: .center)

                HStack {
                    Text(L10n.Screen.Help.title)
                        .textStyle(.titleDefault)
                        .padding(.top, 16)
                }
                HStack {
                    Text(L10n.Screen.Help.paragraph1)
                        .textStyle(.bodyDefaultRegular)
                        .padding(.bottom, 16)
                }
                HStack {
                    Text(L10n.Screen.Help.paragraph2)
                        .textStyle(.bodyDefaultRegular)
                        .padding(.bottom, 16)
                }
                HStack {
                    Button(L10n.Screen.Help.call) {
                        if let helpURL = URL(string: "tel://115") {
                            openURL(helpURL) { accepted in
                                if !accepted {
                                    callsNotSupported = true
                                }
                            }
                        }
                    }
                    .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
                    .accessibilityIdentifier("ProcessesHelpView.CallButton")

                    Spacer()

                    Button(L10n.Screen.Help.openHelp) {
                        onOpenHelpTapped()
                    }
                    .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
                    .accessibilityIdentifier("ProcessesHelpView.OpenHelpButton")
                }
                .padding(.bottom, 16)
            }
            .padding(.leading)
            .padding(.trailing)
        }
        .alert(L10n.Alert.errorOccurred, isPresented: $callsNotSupported, actions: {
            Button(L10n.Alert.ok) {
                callsNotSupported.toggle()
            }
            .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
            .accessibilityIdentifier("ProcessesHelpView.Alert.CallNotSupported.OKButton")
        }, message: {
            Text(L10n.Screen.Help.callsNotSupported)
        })
        .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
        .frame(maxWidth: .infinity)
        .padding(.vertical, 16)
    }
}

struct ProcessesHelpView_Previews: PreviewProvider {
    static var previews: some View {
        ProcessesHelpView { }
    }
}
