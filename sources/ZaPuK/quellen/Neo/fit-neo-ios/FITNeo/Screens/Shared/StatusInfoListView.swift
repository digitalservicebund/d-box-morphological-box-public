//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct StatusInfoListView: View {
    private let store: StoreOf<StatusInfoFeature>
    private let statusInfo: ProcessStatusInfo

    init(store: StoreOf<StatusInfoFeature>) {
        self.store = store
        statusInfo = ProcessStatusInfo(status: store.activeStatus)
    }

    @ViewBuilder
    func statusDetailView(_ status: ProcessStatus) -> some View {
        let expanded = store.expandedStatus.contains(status)
        let statusInfo = ProcessStatusInfo(status: status)
        let isActive = status == store.activeStatus
        let symbol = (isActive ? statusInfo.symbol : .circleFill) ?? .circleFill
        let color = statusInfo.color
        let bgColor = (isActive ? statusInfo.backgroundColor : .clear)

        HStack {
            VStack(spacing: 4) {
                NeoVerticalDivider()
                ZStack {
                    Circle()
                        .frame(width: isActive ? 40 : 0, height: isActive ? 40 : 0)
                        .foregroundColor(bgColor)
                        .overlay(
                            RoundedRectangle(cornerRadius: 32, style: .continuous)
                                .stroke(color)
                        )
                        .accessibilityHidden(true)
                    Image(systemSymbol: symbol)
                        .font(.system(size: isActive ? 16 : 8))
                        .foregroundStyle(color)
                        .padding(0)
                        .accessibilityHidden(true)
                }
                NeoVerticalDivider()
            }
            .frame(width: 36)
            .padding(.leading, 18)
            .padding(.trailing, -4)

            VStack {
                HStack {
                    NeoInfoView(info: statusInfo)
                        .accessibilityLabel(statusInfo.title)
                    Spacer()
                    Button {
                        withAnimation {
                            if expanded {
                                store.send(.collapse(status))
                            } else {
                                store.send(.expand(status))
                            }
                        }
                    } label: {
                        Image(systemSymbol: .chevronDown)
                            .rotationEffect(.degrees(expanded ? -180 : 0))
                            .scaleEffect(1.5)
                            .frame(width: 44, height: 44)
                            .padding(.trailing, 8)
                            .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
                            .accessibilityLabel(
                                expanded ? L10n.Screen.hideDetails : L10n.Screen.expandDetails
                            )
                    }
                    .accessibilityIdentifier("StatusInfoListView.Toggle.\(statusInfo.title)")
                }
                .padding(.vertical, 16)
                .padding(.leading, 16)

                if expanded {
                    Text(status.detailInfo)
                        .textStyle(.bodyDefaultRegular)
                        .padding(.top, 8)
                        .padding(.trailing, 32)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .transition(.opacity)
                        .padding(.leading, 16)
                        .accessibilityIdentifier("StatusInfoListView.Text.\(statusInfo.title)")
                        .padding(.bottom, expanded ? 16 : 0)
                        .opacity(expanded ? 1 : 0)
                }
            }
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            )
            .background(Color.clear)
            .padding(.vertical, 8)
            .padding(.horizontal, 16)
            .animation(.easeInOut, value: expanded)
        }
    }

    var body: some View {
        ScrollViewReader { _ in
            ScrollView {
                VStack(spacing: 0) {
                    statusDetailView(.processSent)
                    statusDetailView(.processReceived)
                    statusDetailView(.processUnderReview)
                    statusDetailView(.processActivityRequired)
                    statusDetailView(.processCompleted)
                }
                .padding(.top, 24)
            }
            .background(Asset.Theme.layoutBackgroundHued.swiftUIColor)
        }
        .navigationTitle(L10n.Screen.StatusInfoListView.navigationTitle)
        .navigationBarTitleDisplayMode(.large)
        .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
        .onAppear {
            store.send(.onAppear)
        }
    }
}

struct StatusInfoListView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationStack {
            StatusInfoListView(
                store: Store(
                    initialState: StatusInfoFeature
                        .State(activeStatus: .processSent)
                ) {
                    StatusInfoFeature()
                }
            )
        }
        .previewDisplayName("Status List")
    }
}
