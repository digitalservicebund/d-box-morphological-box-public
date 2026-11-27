//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import QuickLook
import SFSafeSymbols
import SwiftUI

struct MessagesOverviewView: View {
    private enum Destination: Hashable {
        case processDetails
        case messageDetails
    }

    let store: StoreOf<MessagesOverviewFeature>
    let onOpenHelpTapped: () -> Void

    init(store: StoreOf<MessagesOverviewFeature>, onOpenHelpTapped: @escaping () -> Void = { }) {
        self.store = store
        self.onOpenHelpTapped = onOpenHelpTapped
    }

    var encrypted: Bool {
        store.process.isEncrypted
    }

    private var destination: Binding<Destination?> {
        Binding(get: {
            if store.messageDetails != nil {
                .messageDetails
            } else if store.showProcessDetails {
                .processDetails
            } else {
                nil
            }
        }, set: { newValue in
            guard newValue == nil else { return }
            if store.messageDetails != nil {
                store.send(.messageDetails(.dismiss))
            }
            if store.showProcessDetails {
                store.send(.toggleProcessDetails)
            }
        })
    }

    @ViewBuilder
    func verticalDividerView() -> some View {
        VStack(spacing: 0) {
            NeoVerticalDivider()
        }
        .padding(.leading, 36)
        .padding(.trailing, 35)
    }

    @ViewBuilder
    func processStatusView() -> some View {
        if let info = store.process.statusInfo {
            HStack {
                NavigationLink {
                    let active = store.process.status ?? .processSent
                    StatusInfoListView(
                        store: Store(
                            initialState: StatusInfoFeature.State(activeStatus: active)
                        ) {
                            StatusInfoFeature()
                        }
                    )
                } label: {
                    if let dueDate = store.effectiveDueDate {
                        NeoDueDateStatusView(date: dueDate)
                    } else {
                        NeoInfoView(info: info, showInfo: true)
                        Spacer()
                    }
                }
                .buttonStyle(PlainButtonStyle())
            }
            .padding(16)
        }
    }

    @ViewBuilder
    func emptyMessagesView() -> some View {
        Group {
            Image(asset: Asset.Assets.Images.emptyMessages)
                .frame(width: 104, height: 104)
                .fixedSize()

            HStack {
                Text(L10n.Screen.MessagesOverviewView.noMessages)
                    .textStyle(.titleDefault)
                    .padding(.top, 16)
            }
            HStack {
                Text(L10n.Screen.MessagesOverviewView.noMessagesDetail)
                    .textStyle(.bodyDefaultRegular)
                    .multilineTextAlignment(.center)
                    .padding(.top, 8)
                    .padding(.horizontal, 16)
            }
        }
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier("MessagesOverviewView.EmptyState")
    }

    @ViewBuilder
    func contentLoadingView() -> some View {
        HStack {
            Spacer()
            ProgressView()
                .tint(Asset.Theme.actionsDefault.swiftUIColor)
                .padding(.vertical, 20)
                .accessibilityHidden(true)
            Text(L10n.Screen.loading)
                .textStyle(.labelSmall)
                .padding(8)
            Spacer()
        }
        .accessibilityIdentifier("MessagesOverviewView.Loading")
    }

    @ViewBuilder
    func messagesView() -> some View {
        let entries = store.state.entries
        if !entries.isEmpty {
            // MARK: temporary handling of months for showcasing

            ForEach(Array(entries.enumerated()),
                    id: \.element.id) { index, entry in
                // Month separators will be provided by the timeline date divider items
                let monthSeparator: String? = index == 0 ||
                    entry.monthName() != entries[index - 1].monthName()
                    ? entry.monthName()
                    : nil
                HStack(alignment: .center, spacing: 0) {
                    verticalDividerView()
                    Button {
                        store.send(.selectMessage(entry))
                    } label: {
                        MessageSummaryRow(dialog: entry,
                                          monthSeparator: monthSeparator,
                                          onAttachmentsTapped: {
                                              store.send(.previewAttachments(entry.message.attachments()))
                                          }, onReplyNowTapped: {
                                              store.send(.replyNowTapped(entry))
                                          })
                    }
                    .buttonStyle(.plain)
                    .padding(.bottom, 16)
                    .padding(.trailing, 16)
                }
            }
        } else if store.isContentLoading {
            contentLoadingView()
        } else {
            emptyMessagesView()
        }

        ProcessesHelpView(onOpenHelpTapped: onOpenHelpTapped)
            .padding(.top, 32)
    }

    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                HStack {
                    Text(store.process.processName)
                        .textStyle(.headlineXLarge)
                        .padding(.horizontal, 16)
                        .padding(.top, 16)
                        .accessibilityIdentifier("MessagesOverviewView.ProcessName")
                    Spacer()
                }

                processStatusView()
            }

            LazyVStack(spacing: 0) {
                messagesView()
            }
            .frame(maxWidth: .infinity, alignment: .bottom)
            .padding(.horizontal, 0)
            .padding(.vertical, 8)
            .accessibilityIdentifier("MessagesOverviewView.MessagesList")
        }
        .background(Asset.Theme.layoutBackgroundHued.swiftUIColor)
        .quickLookPreview(Binding(get: { store.selectedAttachmentURL },
                                  set: { _ in store.send(.dismissAttachmentPreview) }),
                          in: store.attachmentURLs)
        .accessibilityIdentifier("MessagesOverviewView.ScrollView")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button(L10n.Screen.ProcessDetailsView.title) {
                    store.send(.toggleProcessDetails)
                }
                .textStyle(.labelDefault)
                .tint(Asset.Theme.actionsDefault.swiftUIColor)
                .accessibilityIdentifier("MessagesOverviewView.ProcessDetailsButton")
            }
        }
        .navigationDestination(item: destination) { destination in
            switch destination {
            case .processDetails:
                ProcessDetailsView(
                    store: Store(initialState: ProcessDetailsFeature.State()) {
                        ProcessDetailsFeature()
                    }
                )
            case .messageDetails:
                if let detailStore = store.scope(state: \.messageDetails, action: \.messageDetails.presented) {
                    MessageDetailsView(store: detailStore)
                }
            }
        }
        .onAppear {
            store.send(.onAppear)
        }
    }
}

struct MessagesOverviewView_Previews: PreviewProvider {
    private typealias F = NeoDialogFixtures

    static var previews: some View {
        // Empty State
        let process1 = ProcessRoom(id: "room-id",
                                   name: "Gewerbliche Drachenhaltung - Erlaubnis beantragen",
                                   authority: "Zentrale Zulassungsstelle für Drachenhaltung",
                                   status: ProcessStatus.processSent)
        let emptyStore = Store(
            initialState: MessagesOverviewFeature.State(process: process1,
                                                        isContentLoading: false,
                                                        _previewEntries: [])
        ) {
            MessagesOverviewFeature()
        }

        // Loading State
        let loadingStore = Store(
            initialState: MessagesOverviewFeature
                .State(process: process1, _previewEntries: [])
        ) {
            MessagesOverviewFeature()
        }

        // Single Message
        let process2 = ProcessRoom(id: "room-id",
                                   name: "Gewerbliche Drachenhaltung - Erlaubnis beantragen",
                                   authority: "Zentrale Zulassungsstelle für Drachenhaltung",
                                   status: ProcessStatus.processUnderReview,
                                   notice: "Rückmeldung der Behörde bis zum 29.09.2025 (Schätzung der App)")
        let now = F.ts()
        let singleDialog = F.dialog(id: "1", timestamp: now, blocks: [
            F.inquiryBlock(id: "1-block", sender: "Peter Muller", timestamp: now,
                           title: "Bestätigung Vorgangserstellung",
                           label: """
                           Sehr geehrte Frau Mustermann, hiermit möchte ich mich \
                           bei Ihnen für das freundliche Gespräch und die \
                           aufschlussreichen Informationen bedanken.
                           """),
            F.fileBlock(id: "1-file1", sender: "Peter Muller", timestamp: now,
                        name: "file1.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "1-file2", sender: "Peter Muller", timestamp: now,
                        name: "file2.jpg", mimetype: "image/jpeg", size: 1_024_000),
            F.fileBlock(id: "1-file3", sender: "Peter Muller", timestamp: now,
                        name: "file3.jpg", mimetype: "image/jpeg", size: 10_024_000)
        ])

        let singleStore = Store(
            initialState: MessagesOverviewFeature.State(process: process2,
                                                        _previewEntries: [singleDialog])
        ) {
            MessagesOverviewFeature()
        }

        // Multiple Messages
        let process3 = ProcessRoom(id: "room-id",
                                   name: "Gewerbliche Drachenhaltung - Erlaubnis beantragen",
                                   authority: "Zentrale Zulassungsstelle für Drachenhaltung",
                                   status: ProcessStatus.processActivityRequired)

        let replyEnvelope = F.envelope(id: "reply-env", timestamp: now, blocks: [
            F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: now,
                           text: """
                           Sehr geehrte Damen und Herren, bei dem entsprechenden
                           Drachen handelt es sich um einen ca. 65 cm großen,
                           grünen Walddrachen handeln. Mit freundlichen Grüße, Erika Mustermann
                           """)
        ])

        let title = "Bestätigung Vorgangserstellung"
        let msgBody = """
        Sehr geehrte Frau Mustermann, hiermit möchte ich mich \
        bei Ihnen für das freundliche Gespräch und die \
        aufschlussreichen Informationen bedanken. \
        Besonders habe ich mich über die angenehme \
        Atmosphäre gefreut, in der ich einen guten Einblick \
        in Ihr Unternehmen sowie die ausgeschriebene Position \
        gewinnen konnte.
        """

        var dialog1 = F.dialog(id: "msg-1", timestamp: now, blocks: [
            F.inquiryBlock(id: "msg-1-block", sender: "Peter Muller", timestamp: now, title: title, label: msgBody)
        ], answers: [])
        dialog1.dueDate = Date.now.addingTimeInterval(60000)

        let dialog2 = F.dialog(id: "msg-2", timestamp: now, blocks: [
            F.messageBlock(id: "msg-2-block", sender: "Peter Muller", timestamp: now, text: msgBody),
            F.fileBlock(id: "msg-2-file1", sender: "Peter Muller", timestamp: now,
                        name: "file1.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "msg-2-file2", sender: "Peter Muller", timestamp: now,
                        name: "file2.jpg", mimetype: "image/jpeg", size: 1_024_000),
            F.fileBlock(id: "msg-2-file3", sender: "Peter Muller", timestamp: now,
                        name: "file3.jpg", mimetype: "image/jpeg", size: 10_024_000)
        ], answers: [replyEnvelope])

        let monthAgoDate = Calendar.current.date(byAdding: .month, value: -1, to: .now) ?? .now
        let monthAgo = F.ts(monthAgoDate)
        let dialog3 = F.dialog(id: "msg-3", timestamp: monthAgo, blocks: [
            F.inquiryBlock(id: "msg-3-block", sender: "Peter Muller",
                           timestamp: monthAgo, title: title, label: msgBody),
            F.fileBlock(id: "msg-3-file1", sender: "Peter Muller", timestamp: monthAgo,
                        name: "file1.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "msg-3-file2", sender: "Peter Muller", timestamp: monthAgo,
                        name: "file2.jpg", mimetype: "image/jpeg", size: 1_024_000),
            F.fileBlock(id: "msg-3-file3", sender: "Peter Muller", timestamp: monthAgo,
                        name: "file3.jpg", mimetype: "image/jpeg", size: 10_024_000)
        ])

        let multiStore = Store(
            initialState: MessagesOverviewFeature.State(process: process3,
                                                        _previewEntries: [dialog1, dialog2, dialog3])
        ) {
            MessagesOverviewFeature()
        }

        return Group {
            NavigationStack {
                MessagesOverviewView(store: multiStore)
            }
            .previewDisplayName("Multi-message")

            NavigationStack {
                MessagesOverviewView(store: singleStore)
            }
            .previewDisplayName("Single message")

            NavigationStack {
                MessagesOverviewView(store: emptyStore)
            }
            .previewDisplayName("No Messages")

            NavigationStack {
                MessagesOverviewView(store: loadingStore)
            }
            .previewDisplayName("Loading")

            NavigationStack {
                MessagesOverviewView(store: loadingStore)
            }
            .previewDisplayName("Loading")
        }
    }
}
