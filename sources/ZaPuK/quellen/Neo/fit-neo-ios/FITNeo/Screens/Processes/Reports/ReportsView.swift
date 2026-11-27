//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct ReportsView: View {
    let store: StoreOf<ReportsFeature>

    @Environment(\.dismiss) var dismiss

    @ViewBuilder
    func emptyReportsView() -> some View {
        VStack {
            Image(asset: Asset.Assets.Images.emptyReports)
                .frame(width: 104, height: 104)
                .fixedSize()

            HStack {
                Text(L10n.Screen.ReportsView.Empty.title)
                    .textStyle(.titleDefault)
                    .padding(.top, 16)
            }
            HStack {
                Text(L10n.Screen.ReportsView.Empty.detail)
                    .textStyle(.bodyDefaultRegular)
                    .multilineTextAlignment(.center)
                    .padding(.top, 8)
                    .padding(.leading, 16)
                    .padding(.trailing, 16)
            }
        }
        .accessibilityIdentifier("MessagesOverviewView.EmptyState")
    }

    @ViewBuilder
    func notificationRow(_ notification: ProcessNotification) -> some View {
        let unread = notification.readStatus == .unread
        let unreadColor = Asset.Theme.feedbackInfoBackground.swiftUIColor
        let readColor = Asset.Theme.layoutBackgroundDefault.swiftUIColor

        HStack {
            if unread {
                Circle()
                    .frame(width: 8, height: 8)
                    .foregroundColor(Asset.Theme.feedbackInfo.swiftUIColor)
                    .accessibilityHidden(true)
                    .padding(.trailing, 8)
                    .padding(.leading, 16)
            }
            VStack(alignment: .leading) {
                Text(notification.title)
                    .textStyle(.bodyDefaultSemiBold)
                    .lineLimit(2)
                Text(notification.detail)
                    .textStyle(.bodySmallRegular)
                    .lineLimit(1)
                if let dueDate = notification.dueDate {
                    let date = dueDate.formatted(date: .numeric, time: .omitted)
                    let dateAlert = NeoGenericInfo(title: L10n.Screen.Warning.messageDueDate(date),
                                                   type: .warning,
                                                   withSymbol: true)

                    NeoInfoView(info: dateAlert)
                }
            }
            .padding(.leading, unread ? 0 : 16)

            Spacer()

            Image(systemSymbol: .chevronRight)
                .scaleEffect(1.5)
                .frame(width: 44, height: 44)
                .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
        }
        .padding(.top, 16)
        .padding(.bottom, 16)
        .alignmentGuide(.listRowSeparatorLeading) { _ in 0 }
        .alignmentGuide(.listRowSeparatorTrailing) { dist in dist.width }
        .listRowSeparatorTint(Asset.Theme.divider.swiftUIColor)
        .listRowInsets(EdgeInsets(top: 0, leading: 0, bottom: 0, trailing: 0))
        .listRowBackground(unread ? unreadColor : readColor)
    }

    @ViewBuilder
    func reportsView() -> some View {
        List(store.state.notifications) { notification in
            notificationRow(notification)
        }
        .listStyle(.plain)
    }

    var body: some View {
        NavigationStack {
            Group {
                if store.state.notifications.isEmpty {
                    emptyReportsView()
                } else {
                    reportsView()
                }
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button {
                        dismiss()
                    } label: {
                        Image(systemSymbol: .xmark)
                    }
                    .tint(Asset.Theme.actionsDefault.swiftUIColor)
                }
                ToolbarItem(placement: .principal) {
                    HStack {
                        Text(L10n.Screen.ReportsView.title)
                            .textStyle(.sublineDefault)
                    }
                }
            }
        }
        .accessibilityIdentifier("Sheets.ReportsView")
        .presentationDetents([.medium, .large])
        .presentationCompactAdaptation(.none)
    }
}

struct ReportsView_Previews: PreviewProvider {
    static var previews: some View {
        let notif1 = try! ProcessNotification.Builder()
            .id("proc-123")
            .title("Sie haben eine neue Nachricht zu Ihrem Vorgang.")
            .detail("Gewerbliche Drachenhaltung - Erlaubnis beantragen")
            .readStatus(.unread)
            .build()

        let notif2 = try! ProcessNotification.Builder()
            .id("proc-321")
            .title("Bitte beantworten Sie eine Rückfrage zu Ihrem Vorgang.")
            .detail("Gewerbliche Drachenhaltung - Erlaubnis beantragen")
            .readStatus(.unread)
            .dueDate(Date().addingTimeInterval(60000))
            .build()

        let notif3 = try! ProcessNotification.Builder()
            .id("proc-111")
            .title("Sie haben eine neue Nachricht zu Ihrem Vorgang.")
            .detail("Gewerbliche Drachenhaltung - Erlaubnis beantragen")
            .readStatus(.read)
            .build()

        let notifications = [notif1, notif2, notif3]

        let state1 = ReportsFeature.State()
        let store1 = Store(initialState: state1) {
            ReportsFeature()
        }

        let state2 = ReportsFeature.State(notifications: notifications)
        let store2 = Store(initialState: state2) {
            ReportsFeature()
        }

        ReportsView(store: store1)
            .previewDisplayName("Empty View")

        ReportsView(store: store2)
            .previewDisplayName("List View")
    }
}
