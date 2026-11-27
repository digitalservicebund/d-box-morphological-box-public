//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct ReplyView: View {
    @Bindable var store: StoreOf<ReplyFeature>
    @Binding var selectedDetent: SheetDetent

    static let collapsedDetent: SheetDetent = .height(120)
    static let expandedDetent: SheetDetent = .fraction(0.5)

    private let detents: [SheetDetent] = [collapsedDetent, expandedDetent, .large]

    init(store: StoreOf<ReplyFeature>, selectedDetent: Binding<SheetDetent>) {
        self.store = store
        _selectedDetent = selectedDetent
    }

    private var isCollapsed: Bool {
        selectedDetent == Self.collapsedDetent
    }

    private func setDetent(_ detent: SheetDetent) {
        withAnimation(.spring(response: 0.35, dampingFraction: 0.85)) {
            selectedDetent = detent
        }
    }

    @ViewBuilder
    private func statusInfo() -> some View {
        if let date = store.dueDate, store.answers.isEmpty {
            NeoDueDateLabelWithSymbol(date: date)
        } else if store.answers.isEmpty {
            NeoInfoView(info: NeoGenericInfo(title: L10n.Screen.Message.Reply.required,
                                             type: .warning,
                                             withSymbol: true))
        } else {
            NeoInfoView(info: NeoGenericInfo(title: L10n.Screen.Message.Reply.answered,
                                             type: .success,
                                             withSymbol: true))
        }
    }

    @ViewBuilder
    private func toggleButton() -> some View {
        Button {
            setDetent(isCollapsed ? Self.expandedDetent : Self.collapsedDetent)
        } label: {
            Image(systemSymbol: .chevronCompactUp)
                .rotationEffect(.degrees(isCollapsed ? 0 : 180))
                .scaleEffect(1.5)
                .frame(minWidth: 48, minHeight: 48)
                .glassedEffect(in: Circle(), interactive: true, clear: true)
                .accessibilityLabel(isCollapsed
                    ? L10n.Screen.Message.Reply.expand
                    : L10n.Screen.Message.Reply.collapse)
        }
        .foregroundColor(isCollapsed
            ? Asset.Theme.actionsOnDefault.swiftUIColor
            : Asset.Theme.actionsDefault.swiftUIColor)
        .background(isCollapsed
            ? Asset.Theme.actionsDefault.swiftUIColor
            : Asset.Theme.inputBackground.swiftUIColor)
        .clipShape(Circle())
        .accessibilityIdentifier("FreeTextReplyView.ToggleButton")
    }

    @ViewBuilder
    private func headerSection() -> some View {
        VStack(spacing: 4) {
            HStack(alignment: .top) {
                statusInfo()
                Spacer()
                toggleButton()
            }

            HStack {
                Text(L10n.Screen.Messages.yourReply)
                    .textStyle(.titleDefault)
                    .accessibilityIdentifier("FreeTextReplyView.Title")
                Spacer()
            }
            .padding(.top, 4)
        }
    }

    @ViewBuilder
    private func answerEditor() -> some View {
        if store.content != nil {
            editorView()
        } else {
            Text(L10n.Screen.Message.Reply.unsupportedContentType)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
        }
    }

    @ViewBuilder
    private func editorView() -> some View {
        switch store.scope(state: \.editor, action: \.editor).case {
        case .freeText(let editorStore):
            FreeTextReplyEditorView(store: editorStore)
        case .singleSelect(let editorStore):
            SingleSelectReplyEditorView(store: editorStore)
        case .unsupported(let editorStore):
            UnsupportedReplyEditorView(store: editorStore)
        }
    }

    @ViewBuilder
    private func answerViewer() -> some View {
        VStack(alignment: .leading) {
            ForEach(store.answers, id: \.id) { answer in
                VStack(alignment: .leading) {
                    answerView(answer)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
            }

            NeoHorizontalDivider()
                .padding(.bottom, 8)

            HStack {
                Spacer()

                Button { } label: {
                    Text(L10n.Screen.Message.Reply.addReply)
                        .padding(.horizontal, 16)
                }
                .modifier(NeoSecondaryButtonStyleModifier())
            }
        }
    }

    @ViewBuilder
    private func answerView(_ envelope: NeoEnvelope) -> some View {
        switch store.editor {
        case .freeText:
            FreeTextReplyEditorView.answerView(envelope, inquiry: store.content)
        case .singleSelect:
            SingleSelectReplyEditorView.answerView(envelope, inquiry: store.content)
        case .unsupported:
            UnsupportedReplyEditorView.answerView(envelope, inquiry: store.content)
        }
    }

    @ViewBuilder
    private func submitReply() -> some View {
        HStack {
            Spacer()

            Button {
                ViewHelper.dismissKeyboard()
                store.send(.submitReply)
            } label: {
                if store.isSendingReply {
                    ProgressView()
                        .tint(Asset.Theme.actionsOnDefault.swiftUIColor)
                } else {
                    Text(L10n.Screen.Message.Reply.Submit.button)
                        .textStyle(.labelDefault)
                }
            }
            .padding(.vertical, 14)
            .padding(.horizontal, 24)
            .foregroundColor(store.isValidReply && !store.isSendingReply
                ? Asset.Theme.actionsOnDefault.swiftUIColor
                : Asset.Theme.textMuted.swiftUIColor.opacity(0.7))
            .background(store.isValidReply && !store.isSendingReply
                ? Asset.Theme.actionsDefault.swiftUIColor
                : Asset.Theme.textMuted.swiftUIColor.opacity(0.3))
            .clipShape(Capsule())
            .disabled(!store.isValidReply || store.isConfirmingReply)
            .glassedEffect(in: Capsule(), interactive: true)
            .accessibilityIdentifier("FreeTextReplyView.SubmitButton")
        }
        .padding(.top, 16)
    }

    @ViewBuilder
    private func answeredContent() -> some View {
        ScrollView {
            answerViewer()
                .accessibilityIdentifier("FreeTextReplyView.AnswersList")
        }
        .scrollDismissesKeyboard(.interactively)
        .scrollMoreIndicator()
    }

    @ViewBuilder
    private func editableContent() -> some View {
        VStack(spacing: 0) {
            answerEditor()
            submitReply()
        }
    }

    @ViewBuilder
    private func expandedContent() -> some View {
        Group {
            if store.answers.isEmpty {
                editableContent()
            } else {
                answeredContent()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    var body: some View {
        DraggableSheetOverlayView(detents: detents,
                                  selection: $selectedDetent,
                                  interactiveThreshold: Self.expandedDetent) {
            headerSection()
                .padding(.horizontal, 16)
                .padding(.top, 24)
        } content: {
            Group {
                if !isCollapsed {
                    expandedContent()
                        .transition(.opacity.combined(with: .move(edge: .bottom)))
                }
            }
            .padding(.horizontal, 16)
            .padding(.bottom, 16)
        }
        .onAppear {
            UITextView.appearance().textContainerInset = UIEdgeInsets(top: 8, left: 4,
                                                                      bottom: 8, right: 44)
        }
        .onChange(of: selectedDetent) { _, detent in
            let expanded = detent != Self.collapsedDetent
            if expanded != store.isExpanded {
                store.send(.toggleExpanded)
            }
            if detent == Self.collapsedDetent {
                ViewHelper.dismissKeyboard()
            }
        }
    }
}

struct ReplyView_Previews: PreviewProvider {
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
                                     label: "Notwendige Angaben zur Bearbeitung Ihres Anliegens")

        let replyEnvelope = F.envelope(id: "reply-env", timestamp: F.ts(), blocks: [
            F.messageBlock(id: "reply-block", sender: "Eva Musterman", timestamp: F.ts(),
                           text: """
                           Sehr geehrte Damen und Herren,
                           mit Befremden habe ich Ihr Schreiben vom vergangenen Monat zur
                           Kenntnis genommen, in welchem Sie mich auffordern, die genauen
                           Maße meines privat genutzten Lenkdrachens innerhalb einer Frist
                           von vierzehn Tagen mitzuteilen. Ich erlaube mir die Anfrage,
                           auf welcher gesetzlichen Grundlage ein derartiges
                           Auskunftsersuchen beruht und welches öffentliche Interesse die
                           Erfassung dieser Daten rechtfertigt. Bis zur Vorlage einer
                           entsprechenden Rechtsgrundlage sehe ich mich außerstande,
                           Ihrem Ersuchen nachzukommen, und bitte um schriftliche Stellungnahme.
                           Mit freundlichen Grüßen
                           """)
        ])

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

        let pollReplyEnvelope = F.envelope(id: "reply-env", timestamp: F.ts(), blocks: [
            F.pollCustomResponseBlock(id: "reply-block", sender: "Eva Musterman",
                                      timestamp: F.ts(), answer: "My custom answer")
        ])

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: inquiry.content)) {
            ReplyFeature()
        })
        .previewDisplayName("Free Text Answer")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: poll.content)) {
            ReplyFeature()
        })
        .previewDisplayName("Poll Answer")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: inquiry.content,
                                                                  dueDate: Date.now)) {
                ReplyFeature()
            })
            .previewDisplayName("Due Date")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: inquiry.content,
                                                                  answers: [replyEnvelope],
                                                                  dueDate: Date.now)) {
                ReplyFeature()
            })
            .previewDisplayName("Free Text Answered")

        PreviewHost(store: Store(initialState: ReplyFeature.State(content: poll.content,
                                                                  answers: [pollReplyEnvelope],
                                                                  dueDate: Date.now)) {
                ReplyFeature()
            })
            .previewDisplayName("Single Select Poll Answered")
    }
}
