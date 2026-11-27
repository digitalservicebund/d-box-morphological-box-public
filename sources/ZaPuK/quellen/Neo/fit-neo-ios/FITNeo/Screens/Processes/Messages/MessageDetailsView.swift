//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import MatrixRustSDK
import QuickLook
import SFSafeSymbols
import SwiftUI

struct MessageDetailsView: View {
    @Bindable var store: StoreOf<MessageDetailsFeature>

    @Environment(\.dismiss) private var dismiss
    @State private var selectedDetent: SheetDetent

    init(store: StoreOf<MessageDetailsFeature>) {
        self.store = store
        let startExpanded = store.replyFeature.isExpanded || store.dialog.hasAnswers
        _selectedDetent = State(initialValue: startExpanded
            ? ReplyView.expandedDetent
            : ReplyView.collapsedDetent)
    }

    @ViewBuilder
    func avatarView() -> some View {
        ZStack {
            Circle()
                .fill(Asset.Theme.feedbackLoaderBackground.swiftUIColor)
                .frame(width: 48, height: 48)

            Text(ViewHelper.initialsFrom(displayName: store.authority))
                .textStyle(.bodyDefaultSemiBold)
        }
    }

    @ViewBuilder
    func messageHeaderPreview() -> some View {
        HStack(alignment: .top) {
            Text(store.authority)
                .textStyle(.bodySmallSemiBold)
                .lineLimit(store.state.expandedHeader ? nil : 2)
                .fixedSize(horizontal: false, vertical: store.state.expandedHeader)

            Spacer()

            Button {
                store.send(.toggleHeader)
            } label: {
                Image(systemSymbol: .chevronDown)
                    .rotationEffect(.degrees(store.state.expandedHeader ? -180 : 0))
                    .scaleEffect(1.5)
                    .frame(minWidth: 48, minHeight: 48)
                    .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
                    .glassedEffect(in: Circle(), interactive: true)
            }
            .padding(.trailing, 8)
            .accessibilityIdentifier("MessageDetailsView.ToggleHeader")
            .accessibilityLabel(
                store.state.expandedHeader
                    ? L10n.Screen.MessageDetailsView.collapseHeader
                    : L10n.Screen.MessageDetailsView.expandHeader
            )
        }
    }

    @ViewBuilder
    func messageHeaderDetails() -> some View {
        // MARK: placeholder data

        Text("Öffnungszeiten: 08:00 - 17:00 (heute)")
            .textStyle(.bodySmallRegular)
            .padding(.bottom, 8)
            .transition(.opacity.combined(with: .move(edge: .top)))

        Button {
            store.send(.openBrowserAction(.openWebsite("https://fitko.de")))
        } label: {
            Image(systemSymbol: .arrowUpRightSquare)

            // MARK: placeholder data

            Text("Zur Website der Behörde")
                .textStyle(.labelDefault)
        }
        .alert(L10n.Alert.errorOccurred,
               isPresented: Binding(get: { store.openBrowserFeature.browserNotSupported },
                                    set: { _ in store.send(.openBrowserAction(.dismissBrowserAlert)) }),
               actions: {
                   Button(L10n.Alert.ok) {
                       store.send(.openBrowserAction(.dismissBrowserAlert))
                   }
                   .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
               }, message: {
                   Text(L10n.Screen.Help.browserNotSupported)
               })
        .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
        .buttonStyle(.plain)
        .transition(.opacity.combined(with: .move(edge: .top)))
    }

    @ViewBuilder
    func messageHeaderView() -> some View {
        VStack(spacing: 0) {
            HStack(alignment: .top, spacing: 16) {
                avatarView()

                VStack(alignment: .leading, spacing: 0) {
                    messageHeaderPreview()

                    if store.state.expandedHeader {
                        messageHeaderDetails()
                    }
                }
            }
            .padding(.horizontal, 16)
            .padding(.bottom, 16)

            NeoHorizontalDivider()
        }
        .animation(.spring(response: 0.3, dampingFraction: 0.8), value: store.state.expandedHeader)
    }

    @ViewBuilder
    private func messageContent() -> some View {
        GeometryReader { geometry in
            ScrollView {
                messageHeaderView()
                    .padding(.top, 16)

                VStack(alignment: .leading, spacing: 0) {
                    Text(store.dialog.formattedTimestamp())
                        .textStyle(.bodySmallRegular)
                        .padding(.top, 16)

                    let blocks = store.dialog.message.blocks
                    let firstFileIndex = blocks.firstIndex { $0.content.isFileAttachment }

                    ForEach(Array(blocks.enumerated()), id: \.offset) { index, block in
                        let sectionHeader: String? = index == firstFileIndex
                            ? L10n.Screen.attachments
                            : nil
                        BuildingBlockView(block: block,
                                          sectionHeader: sectionHeader) { attachment in
                            store.send(.previewAttachment(attachment))
                        }
                    }
                }
                .padding(.horizontal, 16)
            }
            .scrollDismissesKeyboard(.interactively)
            .contentMargins(.bottom, 100 + geometry.safeAreaInsets.bottom)
            .accessibilityIdentifier("MessageDetailsView.ScrollView")
            .onTapGesture {
                UIApplication.shared.sendAction(#selector(UIResponder.resignFirstResponder),
                                                to: nil, from: nil, for: nil)
            }
            .background(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
            .quickLookPreview(Binding(get: { store.selectedAttachmentURL },
                                      set: { _ in store.send(.dismissAttachmentPreview) }),
                              in: store.attachmentURLs)
            .toolbarVisibility(.hidden, for: .tabBar)
            .onAppear {
                store.send(.onAppear)
            }
            .ignoresSafeArea(.container, edges: .bottom)
        }
    }

    var body: some View {
        ZStack {
            messageContent()
                .zIndex(0)

            if store.dialog.shouldShowReply {
                ReplyView(store: store.scope(state: \.replyFeature,
                                             action: \.replyAction),
                          selectedDetent: $selectedDetent)
                    .zIndex(1)
            }

            if store.replyFeature.isConfirmingReply {
                ReplyConfirmationView(store: store.scope(state: \.replyConfirmationFeature,
                                                         action: \.replyConfirmationAction))
                    .zIndex(2)
            }
        }
        .animation(.easeInOut(duration: 0.25), value: store.replyFeature.isConfirmingReply)
        .navigationBarBackButtonHidden(true)
        .toolbarVisibility(selectedDetent == .large ? .hidden : .visible, for: .navigationBar)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                Button {
                    dismiss()
                } label: {
                    Image(systemSymbol: .chevronBackward)
                        .frame(width: 44, height: 44)
                        .contentShape(Rectangle())
                }
                .tint(Asset.Theme.actionsDefault.swiftUIColor)
                .disabled(store.replyFeature.isConfirmingReply)
                .accessibilityIdentifier("MessageDetailsView.BackButton")
            }
        }
    }
}

struct MessageDetailsView_Previews: PreviewProvider {
    private typealias F = NeoDialogFixtures

    static var previews: some View {
        let timestamp = F.ts()
        var dialog = F.dialog(id: "1", timestamp: timestamp, blocks: [
            F.inquiryBlock(id: "1-block",
                           sender: "Peter Muller",
                           timestamp: timestamp,
                           title: "Bestätigung Vorgangserstellung",
                           label: """
                           Sehr geehrte Frau Mustermann, hiermit möchte ich mich \
                           bei Ihnen für das freundliche Gespräch und die \
                           aufschlussreichen Informationen bedanken. \
                           Besonders habe ich mich über die angenehme \
                           Atmosphäre gefreut, in der ich einen guten Einblick \
                           in Ihr Unternehmen sowie die ausgeschriebene Position \
                           gewinnen konnte.
                           """),
            F.infoboxBlock(id: "1-infobox", sender: "Peter Muller", timestamp: timestamp,
                           title: "Infobox", text: "Hier gibt es einen Link mit dazugehörigem Erklärungstext.",
                           linkLabel: "Link", linkUrl: "https://fitko.de"),
            F.buttonBlock(id: "1-button", sender: "Peter Muller", timestamp: timestamp,
                          label: "Fortsetzung des Vorgangs", url: "https://fitko.de"),
            F.fileBlock(id: "1-file1", sender: "Peter Muller", timestamp: timestamp,
                        name: "file1.pdf", mimetype: "application/pdf", size: 256_000),
            F.fileBlock(id: "1-file2", sender: "Peter Muller", timestamp: timestamp,
                        name: "file2.jpg", mimetype: "image/jpeg", size: 1_024_000)
        ])
        dialog.dueDate = Date.now

        let answeredDialog = F.dialog(id: "2", timestamp: timestamp, blocks: [
            F.inquiryBlock(id: "2-block",
                           sender: "Peter Muller",
                           timestamp: timestamp,
                           title: "Bestätigung Vorgangserstellung",
                           label: """
                           Sehr geehrte Frau Mustermann, hiermit möchte ich mich \
                           bei Ihnen für das freundliche Gespräch und die \
                           aufschlussreichen Informationen bedanken. \
                           Besonders habe ich mich über die angenehme \
                           Atmosphäre gefreut, in der ich einen guten Einblick \
                           in Ihr Unternehmen sowie die ausgeschriebene Position \
                           gewinnen konnte.
                           """)
        ], answers: [
            F.envelope(id: "reply-envelope", timestamp: timestamp, blocks: [
                F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: timestamp,
                               text: """
                               Sehr geehrte Damen und Herren, bei dem entsprechenden
                               Drachen handelt es sich um einen ca. 65 cm großen.
                               grünen Walddrachen handeln.\n
                               Mit freundlichen Grüße, Erika Mustermann
                               """)
            ])
        ])

        return Group {
            NavigationStack {
                MessageDetailsView(
                    store: Store(
                        initialState:
                        MessageDetailsFeature.State(caseId: "!preview-room:example.org",
                                                    authority: """
                                                    Peter Muller | Zentrale Zulassungsstelle fur Drachenhaltung
                                                    """,
                                                    dialog: dialog)
                    ) {
                        MessageDetailsFeature()
                    }
                )
                .navigationBarBackButtonHidden(false)
                .toolbar {
                    ToolbarItem(placement: .navigationBarLeading) {
                        Button { } label: {
                            Image(systemName: "chevron.left")
                        }
                    }
                }
            }
            .previewDisplayName("Needs answer")

            NavigationStack {
                MessageDetailsView(
                    store: Store(
                        initialState:
                        MessageDetailsFeature.State(caseId: "!preview-room:example.org",
                                                    authority: """
                                                    Peter Muller | Zentrale Zulassungsstelle fur Drachenhaltung
                                                    """,
                                                    dialog: answeredDialog)
                    ) {
                        MessageDetailsFeature()
                    }
                )
                .navigationBarBackButtonHidden(false)
                .toolbar {
                    ToolbarItem(placement: .navigationBarLeading) {
                        Button { } label: {
                            Image(systemName: "chevron.left")
                        }
                    }
                }
            }
            .previewDisplayName("Answered")
        }
    }
}
