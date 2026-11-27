// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct LoginView: View {
    let store: StoreOf<LoginFeature>

    var body: some View {
        GeometryReader { geometry in
            ZStack {
                Color(Asset.Theme.layoutBackgroundHued.swiftUIColor)
                    .ignoresSafeArea()

                VStack {
                    Spacer()
                        .frame(height: geometry.size.height * 0.10)

                    Image(asset: Asset.Assets.Images.loginLogo)
                        .resizable()
                        .scaledToFit()
                        .frame(width: 96, height: 96)
                        .foregroundColor(Color.accentColor)
                        .padding(.bottom, 24)
                        .accessibilityHidden(true)

                    Text(L10n.App.displayName)
                        .textStyle(.titleDefault)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal, 32)
                        .padding(.bottom, 4)

                    Text(L10n.Screen.LoginView.subtitle)
                        .textStyle(.bodyDefaultRegular)
                        .multilineTextAlignment(.center)
                        .foregroundColor(.primary)
                        .padding(.horizontal, 32)
                        .padding(.bottom, 8)

                    Spacer()

                    Button {
                        Task {
                            store.send(.loginButtonTapped)
                        }
                    } label: {
                        Text(L10n.Screen.LoginView.button)
                            .frame(maxWidth: .infinity, minHeight: 44)
                    }
                    .neoGlassStyle()
                    .accessibilityIdentifier(L10n.Screen.LoginView.button)
                    .padding(.horizontal, 32)
                    .padding(.bottom, geometry.safeAreaInsets.bottom + 32)
                    .disabled(store.loginState != .loggedOut)
                }
                .frame(width: geometry.size.width, height: geometry.size.height)
            }
            .alert(L10n.Alert.errorOccurred, isPresented: .constant(store.loginError != nil), actions: { }, message: {
                Text(L10n.Screen.LoginError.authenticationFailed)
            })
            .overlay {
                if store.loginState == .loggingIn {
                    Color.black.opacity(0.2)
                        .ignoresSafeArea()
                        .overlay {
                            ProgressView()
                                .tint(Asset.Theme.textDefault.swiftUIColor)
                                .scaleEffect(2)
                        }
                        .transition(.opacity)
                }
            }
            .animation(.easeInOut(duration: 0.1), value: store.loginState)
        }
        .onAppear {
            store.send(.onAppear)
        }
    }
}

struct LoginView_Previews: PreviewProvider {
    static var previews: some View {
        LoginView(
            store: Store(initialState: LoginFeature.State()) {
                LoginFeature()
            })
    }
}
