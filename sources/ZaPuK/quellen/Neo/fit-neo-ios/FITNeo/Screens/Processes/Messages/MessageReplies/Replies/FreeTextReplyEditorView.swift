//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct FreeTextReplyEditorView: ReplyEditorView {
    typealias Feature = FreeTextReplyFeature

    @Bindable var store: StoreOf<FreeTextReplyFeature>
    @FocusState private var isFocused: Bool

    init(store: StoreOf<FreeTextReplyFeature>) {
        self.store = store
    }

    var body: some View {
        HStack {
            ZStack(alignment: .topTrailing) {
                TextEditor(text: $store.textContent.sending(\.contentChanged))
                    .focused($isFocused)
                    .placeholder(L10n.Screen.Message.Reply.placeholder,
                                 for: store.textContent)
                    .textStyle(.bodyDefaultRegular)
                    .foregroundStyle(Asset.Theme.textDefault.swiftUIColor)
                    .overlay(
                        RoundedRectangle(cornerRadius: 8, style: .continuous)
                            .stroke(Asset.Theme.divider.swiftUIColor)
                    )
                    .scrollContentBackground(.hidden)
                    .background(Asset.Theme.inputBackground.swiftUIColor)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .accessibilityIdentifier("FreeTextReplyView.TextEditor")
                    .accessibilityLabel(L10n.Screen.Message.Reply.placeholder)

                if store.textContent.isEmpty {
                    Button {
                        store.send(.addReplyTemplate)
                    } label: {
                        Image(systemSymbol: .textBadgePlus)
                            .frame(width: 44, height: 44)
                    }
                    .foregroundColor(Asset.Theme.actionsDefault.swiftUIColor)
                    .accessibilityIdentifier("FreeTextReplyView.TemplateButton")
                    .accessibilityLabel(L10n.Screen.Message.Reply.addTemplate)
                } else {
                    Button {
                        store.send(.contentCleared)
                    } label: {
                        Image(systemSymbol: .xmarkCircle)
                            .frame(width: 44, height: 44)
                    }
                    .foregroundColor(Asset.Theme.feedbackDanger.swiftUIColor)
                    .accessibilityIdentifier("FreeTextReplyView.ClearButton")
                    .accessibilityLabel(L10n.Screen.Message.Reply.clearReply)
                }
            }
        }
        .onAppear {
            Task { @MainActor in
                if let name = try? AppState.shared.userDisplayName() {
                    store.send(.setTemplate(name))
                }
            }
        }
        .padding(.top, 16)
    }

    static func answerView(_ envelope: NeoEnvelope, inquiry: NeoContent?) -> some View {
        Group {
            Text(envelope.formattedTimestamp())
                .textStyle(.bodySmallRegular)
                .padding(.top, 16)

            ForEach(envelope.blocks, id: \.id) { block in
                messageBlockView(block)
            }
        }
    }

    static func confirmationView(_ contents: [NeoContent], inquiry: NeoContent?) -> some View {
        ForEach(contents) { content in
            messageContentView(content)
        }
    }

    @ViewBuilder
    private static func messageBlockView(_ block: NeoBuildingBlock) -> some View {
        messageContentView(block.content)
    }

    @ViewBuilder
    private static func messageContentView(_ content: NeoContent) -> some View {
        switch content {
        case .message(let text, _):
            messageText(text)
        default:
            Text(L10n.Screen.Message.Reply.unsupportedContentType)
                .textStyle(.bodyDefaultRegular)
        }
    }

    @ViewBuilder
    private static func messageText(_ text: NeoText) -> some View {
        if let plain = text.plain {
            Text(plain)
                .textStyle(.bodyDefaultRegular)
                .padding(.bottom, 8)
        } else {
            Text(L10n.Screen.Message.Reply.noContent)
                .textStyle(.bodyDefaultRegular)
        }
    }
}

struct FreeTextReplyEditorView_Previews: PreviewProvider {
    private typealias F = NeoDialogFixtures

    private struct PreviewHost: View {
        let store: StoreOf<ReplyFeature>

        @State private var detent: SheetDetent = ReplyView.expandedDetent

        var body: some View {
            ZStack {
                Asset.Theme.layoutBackgroundDefault.swiftUIColor
                    .ignoresSafeArea()
                Text("Message details")

                ReplyView(store: store, selectedDetent: $detent)
            }
        }
    }

    static var previews: some View {
        let inquiry = F.inquiryBlock(id: "1-block",
                                     sender: "Peter Muller",
                                     timestamp: F.ts(),
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

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: inquiry.content)) {
            ReplyFeature()
        })
        .previewDisplayName("Free Text Editor")

        let replyEnvelope = F.envelope(id: "reply-env", timestamp: F.ts(), blocks: [
            F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: F.ts(),
                           text: """
                           Sehr geehrte Damen und Herren, bei dem entsprechenden
                           Drachen handelt es sich um einen ca. 65 cm großen.
                           grünen Walddrachen handeln.\n
                           Mit freundlichen Grüße, Erika Mustermann
                           """)
        ])

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: inquiry.content,
                                                                  answers: [replyEnvelope])) {
                ReplyFeature()
            })
            .previewDisplayName("Poll Answer")
    }
}
