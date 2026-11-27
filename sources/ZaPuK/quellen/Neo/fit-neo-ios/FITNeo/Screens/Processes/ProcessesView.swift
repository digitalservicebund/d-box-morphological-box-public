//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import SFSafeSymbols
import SwiftUI

struct ProcessesView: View {
    @Bindable var store: StoreOf<ProcessesViewFeature>

    @State private var areReportsPresented = false
    @State private var isCompletedSectionExpanded = false

    var completedProcesses: String {
        "(\(store.completed.count))"
    }

    @ViewBuilder
    private func processesList() -> some View {
        ScrollView {
            LazyVStack(spacing: 0) {
                processesSection(store.open)
            }

            if !store.completed.isEmpty {
                Button {
                    withAnimation {
                        isCompletedSectionExpanded.toggle()
                    }
                } label: {
                    HStack {
                        Text(L10n.Screen.ProcessesView.completedProcesses)
                            .textStyle(.sublineDefault)
                        Text(completedProcesses)
                            .textStyle(.sublineDefault)
                        Spacer()
                        Image(systemSymbol: .chevronDown)
                            .rotationEffect(
                                .degrees(
                                    isCompletedSectionExpanded ? -180 : 0
                                )
                            )
                            .scaleEffect(1.5)
                            .frame(minWidth: 44, minHeight: 44)
                            .glassedEffect(in: Circle(), interactive: true)
                            .accessibilityLabel(isCompletedSectionExpanded
                                ? L10n.Screen.ProcessesView.hideCompleted
                                : L10n.Screen.ProcessesView.showCompleted
                            )
                    }
                    .padding(EdgeInsets(top: 32, leading: 16, bottom: 8, trailing: 24))
                }
                .tint(Asset.Theme.actionsDefault.swiftUIColor)
                .accessibilityIdentifier("ProcessesView.CompletedSection.Header")

                if isCompletedSectionExpanded {
                    VStack(spacing: 0) {
                        processesSection(store.completed)
                    }
                }
            }

            ProcessesHelpView {
                store.send(.openHelpButtonTapped)
            }
            .padding(EdgeInsets(top: 16, leading: 0, bottom: 0, trailing: 0))
        }
        .background(Asset.Theme.layoutBackgroundHued.swiftUIColor)
        .accessibilityIdentifier("ProcessesView.List")
    }

    @ViewBuilder
    private func processesSection(_ processes: [ProcessRoom]) -> some View {
        ForEach(processes) { process in
            Button {
                store.send(.processSelected(process))
            } label: {
                ProcessRoomRow(process: process)
            }
            .buttonStyle(NeoHighlightStyle())
            .padding(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
        }
    }

    var body: some View {
        NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
            Group {
                if store.loadState == .loaded {
                    if store.numberOfProcesses > 0 {
                        processesList()
                    } else {
                        ScrollView {
                            ProcessesEmptyView(store: store)
                                .padding(.bottom, 32)
                            ProcessesHelpView {
                                store.send(.openHelpButtonTapped)
                            }
                        }
                        .background(Asset.Theme.layoutBackgroundHued.swiftUIColor)
                        .accessibilityIdentifier("ProcessesView.EmptyState")
                    }
                } else if store.loadState == .failed {
                    Text(L10n.Screen.loadingFailed)
                        .accessibilityIdentifier("ProcessesView.Failed")
                } else {
                    HStack {
                        Spacer()
                        ProgressView()
                            .tint(Asset.Theme.actionsDefault.swiftUIColor)
                            .padding(.vertical, 20)
                        Text(L10n.Screen.loading)
                            .textStyle(.bodyDefaultRegular)
                            .accessibilityIdentifier("ProcessesView.Loading")
                            .padding(8)
                        Spacer()
                    }
                }
            }
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button(L10n.Screen.ReportsView.title, systemSymbol: .bell) {
                        store.send(.notificationsButtonTapped)
                        areReportsPresented = true
                    }
                    .tint(Asset.Theme.actionsDefault.swiftUIColor)
                    .sheet(isPresented: $areReportsPresented) {
                        ReportsView(store: mockedReportsStore())
                    }
                }
            }
            .navigationTitle(L10n.Navigation.Tab.processes)
            .safeAreaInset(edge: .top) {
                Color.clear.frame(height: store.numberOfProcesses == 0 ? 0 : 16)
            }
        } destination: { messagesStore in
            MessagesOverviewView(store: messagesStore) {
                store.send(.openHelpButtonTapped)
            }
        }
        .onAppear {
            store.send(.onAppear)
        }
    }

    func mockedReportsStore() -> StoreOf<ReportsFeature> {
        let emptyState = ReportsFeature.State()

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
        let fullState = ReportsFeature.State(notifications: notifications)

        let state = store.showEmptyReports ? emptyState : fullState

        let store = Store(initialState: state) {
            ReportsFeature()
        }
        return store
    }
}

struct ProcessesView_Previews: PreviewProvider {
    static var previews: some View {
        let proc001Name = "Gewerbliche Drachenhaltung - Erlaubnis beantragen"
        let proc001Authority = "Zentrale Zulassungsstelle für Drachenhaltung"
        let proc002Name = "Zentrale Zulassungsstelle für Drachenhaltung"
        let proc002Authority = "Nachrichten mit dieser Behörde"
        let proc003Name = "Gewerbliche Drachenhaltung - Erlaubnis beantragen"
        let proc003Authority = "Zentrale Zulassungsstelle für Drachenhaltung"

        let mockProcesses = [
            ProcessRoom(id: "proc-001",
                        name: proc001Name,
                        authority: proc001Authority,
                        status: .processReceived),
            ProcessRoom(id: "proc-002",
                        name: proc002Name,
                        authority: proc002Authority),
            ProcessRoom(id: "proc-003",
                        name: proc003Name,
                        authority: proc003Authority,
                        status: .processCompleted)
        ]
        let mockState: ProcessesViewFeature.State = .init(loadState: .loaded,
                                                          allProcesses: mockProcesses)

        let emptyState: ProcessesViewFeature.State = .init(loadState: .loaded,
                                                           allProcesses: [])

        let loadingState: ProcessesViewFeature.State = .init(loadState: .loading,
                                                             allProcesses: [])

        ProcessesView(
            store: Store(
                initialState: mockState
            ) {
                ProcessesViewFeature()
            })
            .previewDisplayName("Process List")

        ProcessesView(
            store: Store(
                initialState: emptyState
            ) {
                ProcessesViewFeature()
            })
            .previewDisplayName("Empty List")

        ProcessesView(
            store: Store(
                initialState: loadingState
            ) {
                ProcessesViewFeature()
            })
            .previewDisplayName("Loading")
    }
}
