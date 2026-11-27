//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct SingleSelectReplyEditorView: ReplyEditorView {
    typealias Feature = SingleSelectReplyFeature

    static let bottomAnchorID = "SingleSelectReplyEditor.BottomAnchor"

    @Bindable var store: StoreOf<SingleSelectReplyFeature>
    @FocusState private var isCommentFocused: Bool

    init(store: StoreOf<SingleSelectReplyFeature>) {
        self.store = store
    }

    private static func displayOptions(_ options: [NeoPollAnswer]) -> [NeoPollAnswer] {
        let noneText = NeoText(plain: L10n.Screen.Message.Reply.Polls.noneOfTheAbove,
                               html: nil)
        return options.map { option in
            option.id == SingleSelectReplyFeature.customOptionID
                ? NeoPollAnswer(id: option.id, text: noneText)
                : option
        }
    }

    private func scrollToBottom(_ proxy: ScrollViewProxy) {
        withAnimation {
            proxy.scrollTo(Self.bottomAnchorID, anchor: .bottom)
        }
    }

    var body: some View {
        ScrollViewReader { proxy in
            ScrollView {
                VStack(alignment: .leading) {
                    ForEach(Self.displayOptions(store.options)) { option in
                        PollOptionRow(id: option.id,
                                      label: option.text.plain ?? option.id,
                                      isSelected: store.selectedOptionID == option.id) {
                            store.send(.optionSelected(option.id))
                        }
                    }
                    NeoHorizontalDivider()

                    if store.isCustomSelected {
                        TextEditor(text: $store.comment.sending(\.commentChanged))
                            .focused($isCommentFocused)
                            .placeholder(L10n.Screen.Message.Reply.Polls.addComment,
                                         for: store.comment)
                            .textStyle(.bodyDefaultRegular)
                            .foregroundStyle(Asset.Theme.textDefault.swiftUIColor)
                            .overlay(
                                RoundedRectangle(cornerRadius: 8, style: .continuous)
                                    .stroke(Asset.Theme.divider.swiftUIColor)
                            )
                            .scrollContentBackground(.hidden)
                            .background(Asset.Theme.inputBackground.swiftUIColor)
                            .frame(maxWidth: .infinity, maxHeight: .infinity)
                            .frame(minHeight: 40)
                            .padding(.vertical, 16)
                            .accessibilityIdentifier("SingleSelectReplyEditor.TextEditor")
                            .accessibilityLabel(L10n.Screen.Message.Reply.Polls.addComment)
                    }

                    Color.clear
                        .frame(height: 1)
                        .id(Self.bottomAnchorID)
                }
                .padding(.top, 16)
                .onChange(of: store.selectedOptionID) { _, _ in
                    if store.isCustomSelected {
                        isCommentFocused = true
                        scrollToBottom(proxy)
                    }
                }
                .onChange(of: store.comment) { _, _ in
                    if store.isCustomSelected {
                        scrollToBottom(proxy)
                    }
                }
                .accessibilityElement(children: .contain)
            }
            .accessibilityIdentifier("SingleSelectReplyEditor")
            .scrollMoreIndicator()
        }
    }

    static func answerView(_ envelope: NeoEnvelope, inquiry: NeoContent?) -> some View {
        let options = displayOptions(pollOptions(in: inquiry))
        let selection = pollSelection(in: envelope.blocks.map(\.content))
        let selectedIDs = selectedIDs(for: selection)

        ScrollView {
            VStack(alignment: .leading) {
                Text(envelope.formattedTimestamp())
                    .textStyle(.bodySmallRegular)
                    .padding(.top, 16)

                ForEach(options) { option in
                    PollSelectionRow(label: option.text.plain ?? option.id,
                                     isSelected: selectedIDs.contains(option.id))
                }

                if let comment = selection?.custom, !comment.isEmpty {
                    Text(L10n.Screen.Message.Reply.Polls.comment)
                        .textStyle(.bodyDefaultSemiBold)

                    commentView(comment)
                        .padding(.bottom, 16)
                }
            }
        }
        .accessibilityIdentifier("SingleSelectReplyEditor.Answer")
    }

    static func confirmationView(_ contents: [NeoContent], inquiry: NeoContent?) -> some View {
        let options = displayOptions(pollOptions(in: inquiry))
        let selection = pollSelection(in: contents)
        let selectedIDs = selectedIDs(for: selection)

        return VStack(alignment: .leading, spacing: 8) {
            ForEach(options.filter { selectedIDs.contains($0.id) }) { option in
                Text(option.text.plain ?? option.id)
                    .textStyle(.bodyDefaultRegular)
            }

            if let comment = selection?.custom, !comment.isEmpty {
                Text(L10n.Screen.Message.Reply.Polls.comment)
                    .textStyle(.bodyDefaultSemiBold)

                commentView(comment)
            }
        }
        .accessibilityIdentifier("SingleSelectReplyEditor.Confirmation")
    }

    private static func pollOptions(in inquiry: NeoContent?) -> [NeoPollAnswer] {
        guard case .inquiryPoll(let poll) = inquiry else { return [] }
        return poll.answers
    }

    private static func pollSelection(
        in contents: [NeoContent]
    ) -> (selections: [String], custom: String?)? {
        for content in contents {
            if case .poll(let selections, let custom, _) = content {
                return (selections, custom)
            }
        }
        return nil
    }

    private static func selectedIDs(
        for selection: (selections: [String], custom: String?)?
    ) -> [String] {
        guard let selection else { return [] }
        if let custom = selection.custom, !custom.isEmpty {
            return [SingleSelectReplyFeature.customOptionID]
        }
        return selection.selections
    }

    @ViewBuilder
    private static func commentView(_ comment: String) -> some View {
        Text(comment)
            .textStyle(.bodyDefaultRegular)
            .foregroundStyle(Asset.Theme.textDefault.swiftUIColor)
            .padding(.top, 8)
            .accessibilityIdentifier("SingleSelectReplyEditor.Comment")
    }
}

struct PollSelectionRow: View {
    let label: String
    let isSelected: Bool

    var body: some View {
        NeoHorizontalDivider()

        HStack {
            Text(label)
                .textStyle(.bodyDefaultRegular)

            Spacer()

            ZStack {
                if isSelected {
                    Image(systemSymbol: .checkmarkCircleFill)
                        .symbolRenderingMode(.palette)
                        .foregroundStyle(Asset.Theme.actionsOnDefault.swiftUIColor,
                                         Asset.Theme.feedbackSuccess.swiftUIColor)
                        .scaleEffect(1.5)
                }
            }
            .frame(width: 33, height: 33)
        }
        .padding(.vertical, 8)
        .padding(.trailing, 2)
        .accessibilityElement(children: .combine)
        .accessibilityLabel(label)
        .accessibilityAddTraits(isSelected ? .isSelected : [])
    }
}

struct PollOptionRow: View {
    let id: String
    let label: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        NeoHorizontalDivider()

        Button(action: action) {
            HStack {
                Text(label)
                    .textStyle(.bodyDefaultRegular)

                Spacer()

                ZStack {
                    Circle()
                        .stroke(Asset.Theme.actionsDefault.swiftUIColor, lineWidth: 2)
                        .frame(width: 22, height: 22)

                    if isSelected {
                        Circle()
                            .fill(Asset.Theme.actionsDefault.swiftUIColor)
                            .frame(width: 22, height: 22)

                        Image(systemSymbol: .checkmarkCircleFill)
                            .symbolRenderingMode(.palette)
                            .foregroundStyle(Asset.Theme.inputBackground.swiftUIColor,
                                             Asset.Theme.actionsDefault.swiftUIColor)
                            .scaleEffect(1.5)
                    }
                }
                .frame(width: 33, height: 33)
            }
            .contentShape(Rectangle())
        }
        .padding(.vertical, 8)
        .padding(.trailing, 2)
        .buttonStyle(.plain)
        .accessibilityElement(children: .combine)
        .accessibilityIdentifier("SingleSelectReplyEditor.Option.\(id)")
        .accessibilityLabel(label)
        .accessibilityAddTraits(isSelected ? [.isButton, .isSelected] : .isButton)
    }
}

struct SingleSelectReplyEditorView_Previews: PreviewProvider {
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
        let poll = F.pollBlock(id: "1-poll-block",
                               sender: "Peter Muller",
                               timestamp: F.ts(),
                               multiSelect: false,
                               question: "Question",
                               answers: [
                                   "1": "Kleiner als 1 Meter",
                                   "2": "Zwischen 1 und 2 Meter",
                                   "dev.fitko.neo.poll.custom.v1": "Custom"
                               ])

        let replyEnvelope = F.envelope(id: "reply-env", timestamp: F.ts(), blocks: [
            F.pollResponseBlock(id: "reply-block", sender: "Eva Musterman",
                                timestamp: F.ts(), answerId: "2")
        ])

        let customReplyEnvelope = F.envelope(id: "reply-env", timestamp: F.ts(), blocks: [
            F.pollCustomResponseBlock(id: "reply-block", sender: "Eva Musterman",
                                      timestamp: F.ts(), answer: "My custom answer")
        ])

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: poll.content)) {
            ReplyFeature()
        })
        .previewDisplayName("Poll Editor")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: poll.content,
                                                                  answers: [replyEnvelope])) {
                ReplyFeature()
            })
            .previewDisplayName("Poll Answer")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: poll.content,
                                                                  answers: [customReplyEnvelope])) {
                ReplyFeature()
            })
            .previewDisplayName("Custom Poll Answer")
    }
}
