//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct ProcessesEmptyView: View {
    @Bindable var store: StoreOf<ProcessesViewFeature>

    var body: some View {
        VStack(alignment: .leading) {
            HStack {
                Text(L10n.Screen.ProcessesView.subHeader)
                    .textStyle(.titleDefault)
            }
            HStack {
                Text(L10n.Screen.ProcessesView.paragraph1)
                    .textStyle(.bodyDefaultRegular)
                    .padding(.bottom, 8)
            }
            BenefitsView(text: L10n.Screen.ProcessesView.benefit1,
                         image: Asset.Assets.Images.emptyCases1)
            BenefitsView(text: L10n.Screen.ProcessesView.benefit2,
                         image: Asset.Assets.Images.emptyCases2)
            BenefitsView(text: L10n.Screen.ProcessesView.benefit3,
                         image: Asset.Assets.Images.emptyCases3)
            HStack {
                Text(L10n.Screen.ProcessesView.paragraph2)
                    .textStyle(.bodyDefaultRegular)
                    .padding(.bottom, 16)
                    .padding(.top, 8)
            }
            HStack {
                Spacer()
                Button(L10n.Screen.ProcessesView.federalPortal) {
                    store.send(.openBrowser(.openWebsite(AppSettings.federalPortalURL)))
                }
                .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
                .accessibilityIdentifier("ProcessesEmptyView.OpenFederalPortal")
            }
        }
        .alert(L10n.Alert.errorOccurred,
               isPresented: Binding(get: { store.openBrowser.browserNotSupported },
                                    set: { _ in store.send(.openBrowser(.dismissBrowserAlert)) }),
               actions: {
                   Button(L10n.Alert.ok) {
                       store.send(.openBrowser(.dismissBrowserAlert))
                   }
                   .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
               }, message: {
                   Text(L10n.Screen.Help.browserNotSupported)
               })
        .frame(maxWidth: .infinity)
        .padding(.leading)
        .padding(.trailing)
        .padding(.top, 24)
    }
}

struct BenefitsView: View {
    var image: ImageAsset?
    var text: String

    init(text: String, image: ImageAsset? = nil) {
        self.image = image
        self.text = text
    }

    var body: some View {
        Group {
            HStack {
                ZStack {
                    Rectangle()
                        .clipShape(RoundedRectangle(cornerRadius: 16, style: .circular))
                        .foregroundColor(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
                        .frame(width: 80, height: 80)
                        .fixedSize()
                    if let img = image {
                        Image(asset: img)
                            .frame(width: 80, height: 80)
                            .fixedSize()
                    }
                }
                Text(text)
                    .textStyle(.bodyDefaultRegular)
                    .padding(.leading, 10)
            }
            .padding(.vertical, 4)
            .frame(minHeight: 80)
        }
    }
}

struct ProcessesEmptyView_Previews: PreviewProvider {
    static var previews: some View {
        ProcessesEmptyView(
            store: Store(
                initialState: ProcessesViewFeature.State()
            ) {
                ProcessesViewFeature()
            }
        )
        .background(Asset.Theme.layoutBackgroundHued.swiftUIColor)
    }
}
