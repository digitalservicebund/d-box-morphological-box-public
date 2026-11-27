//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SFSafeSymbols
import SwiftUI

protocol NeoInfoCapsuleProtocol: Equatable, Hashable {
    var title: String { get }
    var subtitle: String? { get }
    var symbol: SFSymbol? { get }
    var color: Color { get }
    var backgroundColor: Color { get }
}

struct ProcessStatusInfo: NeoInfoCapsuleProtocol {
    private let status: ProcessStatus
    private let notice: String?

    init(status stat: ProcessStatus) {
        status = stat
        notice = nil
    }

    init(status stat: ProcessStatus, withNotice notice: String) {
        status = stat
        self.notice = notice
    }

    var title: String {
        switch status {
        case .processSent: String(L10n.Status.processSent)
        case .processReceived: String(L10n.Status.processReceived)
        case .processUnderReview: String(L10n.Status.processUnderReview)
        case .processActivityRequired: String(L10n.Status.processActivityRequired)
        case .processCompleted: String(L10n.Status.processCompleted)
        }
    }

    var subtitle: String? {
        notice
    }

    var symbol: SFSymbol? {
        switch status {
        case .processSent: SFSymbol.paperplaneFill
        case .processReceived: SFSymbol.squareAndArrowDownFill
        case .processUnderReview: SFSymbol.clockFill
        case .processActivityRequired: SFSymbol.exclamationmarkTriangleFill
        case .processCompleted: SFSymbol.checkmarkCircleFill
        }
    }

    var color: Color {
        switch status {
        case .processSent, .processReceived, .processUnderReview:
            Asset.Theme.feedbackLoader.swiftUIColor
        case .processActivityRequired:
            Asset.Theme.feedbackWarning.swiftUIColor
        case .processCompleted:
            Asset.Theme.feedbackSuccess.swiftUIColor
        }
    }

    var backgroundColor: Color {
        switch status {
        case .processSent, .processReceived, .processUnderReview:
            Asset.Theme.layoutBackgroundHued.swiftUIColor
        case .processActivityRequired:
            Asset.Theme.feedbackWarningBackground.swiftUIColor
        case .processCompleted:
            Asset.Theme.feedbackSuccessBackground.swiftUIColor
        }
    }
}

enum NeoInfoType {
    case warning
    case danger
    case success
    case info
    case newMessage
    case attachment
}

struct NeoGenericInfo: NeoInfoCapsuleProtocol {
    let _title: String
    let _subtitle: String?
    let withSymbol: Bool
    private let type: NeoInfoType

    init(title: String, subtitle: String? = nil, type: NeoInfoType) {
        _title = title
        _subtitle = subtitle
        withSymbol = true
        self.type = type
    }

    init(title: String, subtitle: String? = nil, type: NeoInfoType, withSymbol: Bool) {
        _title = title
        _subtitle = subtitle
        self.withSymbol = withSymbol
        self.type = type
    }

    var title: String {
        _title
    }

    var subtitle: String? {
        _subtitle
    }

    var symbol: SFSymbol? {
        if !withSymbol {
            return nil
        }
        return switch type {
        case .warning: SFSymbol.exclamationmarkTriangleFill
        case .danger: SFSymbol.exclamationmarkTriangleFill
        case .success: SFSymbol.checkmarkCircleFill
        case .info: SFSymbol.infoCircleFill
        case .newMessage: SFSymbol.circleFill
        case .attachment: SFSymbol.paperclip
        }
    }

    var color: Color {
        switch type {
        case .warning: Asset.Theme.feedbackWarning.swiftUIColor
        case .danger: Asset.Theme.feedbackDanger.swiftUIColor
        case .success: Asset.Theme.feedbackSuccess.swiftUIColor
        case .info: Asset.Theme.feedbackInfo.swiftUIColor
        case .newMessage: Asset.Theme.feedbackLoader.swiftUIColor
        case .attachment: Asset.Theme.feedbackLoader.swiftUIColor
        }
    }

    var backgroundColor: Color {
        switch type {
        case .warning: Asset.Theme.feedbackWarningBackground.swiftUIColor
        case .danger: Asset.Theme.feedbackDangerBackground.swiftUIColor
        case .success: Asset.Theme.feedbackSuccessBackground.swiftUIColor
        case .info: Asset.Theme.feedbackInfoBackground.swiftUIColor
        case .newMessage: Asset.Theme.layoutBackgroundHued.swiftUIColor
        case .attachment: Asset.Theme.layoutBackgroundHued.swiftUIColor
        }
    }
}

struct NeoInfoView: View {
    private let store: StoreOf<OpenBrowserFeature>

    private var title: String
    private var subtitle: String?
    private var symbol: SFSymbol?
    private var color: Color?
    private var backgroundColor: Color?
    private var showInfo: Bool
    private var hasLink: Bool
    private var linkTo: String?
    private var linkLabel: String?

    init() {
        store = Store(
            initialState: OpenBrowserFeature.State()
        ) {
            OpenBrowserFeature()
        }

        title = ""
        subtitle = nil
        symbol = nil
        color = nil
        backgroundColor = nil
        showInfo = false
        hasLink = false
        linkTo = nil
        linkLabel = nil
    }

    init(info: any NeoInfoCapsuleProtocol) {
        self.init()

        title = info.title
        subtitle = info.subtitle
        symbol = info.symbol
        color = info.color
        backgroundColor = info.backgroundColor
        showInfo = false
        hasLink = false
        linkTo = nil
    }

    init(info: any NeoInfoCapsuleProtocol, showInfo: Bool = false) {
        self.init(info: info)

        self.showInfo = showInfo
    }

    init(info: any NeoInfoCapsuleProtocol, withLink link: String, andLabel label: String) {
        self.init(info: info)

        hasLink = true
        linkTo = link
        linkLabel = label
    }

    init(text: NeoText, infobox: NeoText, hyperlink: NeoHyperlink) {
        let info = NeoGenericInfo(title: text.plain ?? L10n.Screen.information,
                                  subtitle: infobox.plain,
                                  type: .info)
        self.init(info: info, withLink: hyperlink.url, andLabel: hyperlink.label.plain ?? hyperlink.url)
    }

    @ViewBuilder
    func borderContent(withColor color: Color, hasSubtitle subtitle: Bool, isCircle circle: Bool) -> some View {
        if circle {
            Circle().stroke(color)
        } else {
            RoundedRectangle(cornerRadius: subtitle ? 16 : 32, style: .continuous)
                .stroke(color)
        }
    }

    var body: some View {
        let _color = color ?? Asset.Theme.feedbackLoader.swiftUIColor
        let _backgroundColor = backgroundColor ?? Asset.Theme.layoutBackgroundHued.swiftUIColor
        let hasSymbol = symbol != nil
        let hasSubtitle = subtitle != nil
        let isCircle = title.count <= 2 ? true : false

        VStack {
            HStack(alignment: .top) {
                if let symbol {
                    Image(systemSymbol: symbol)
                        .foregroundStyle(_color)
                        .frame(width: 24, height: 24)
                        .padding(.leading, hasSubtitle ? 16 : 4)
                        .padding(.trailing, 0)
                        .padding(.top, hasSubtitle ? 8 : 0)
                        .accessibilityHidden(true)
                }
                VStack(alignment: .leading) {
                    Text(title)
                        .textStyle(
                            hasSubtitle ? .bodyDefaultSemiBold : .labelSmall
                        )
                        .padding(.leading, hasSymbol ? (hasSubtitle ? 4 : -4) : 8)
                        .padding(.trailing, 4)
                        .padding(.top, hasSubtitle ? 8 : 0)
                    if let subtitle {
                        Text(subtitle)
                            .textStyle(.bodyDefaultRegular)
                            .padding(.leading, 4)
                            .padding(.top, -8)
                            .padding(.trailing, showInfo ? 0 : 16)
                            .padding(.bottom, 8)
                    }
                }
                if showInfo {
                    Image(systemSymbol: SFSymbol.questionmarkCircle)
                        .scaleEffect(1.2)
                        .foregroundColor(Asset.Theme.actionsDefault.swiftUIColor)
                        .padding(.top, hasSubtitle ? 10 : 2)
                        .padding(.leading, 0)
                        .padding(.trailing, hasSubtitle ? 8 : 4)
                        .accessibilityLabel(L10n.Screen.showMoreInfo)
                }
                if hasLink {
                    Spacer()
                }
            }
            if hasLink, let linkLabel {
                HStack {
                    Spacer()
                    Button {
                        if let linkTo {
                            store.send(.openWebsite(linkTo))
                        }
                    } label: {
                        Image(systemSymbol: .arrowUpRightSquare)
                        Text(linkLabel)
                            .textStyle(.labelDefault)
                    }
                    .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
                    .buttonStyle(.plain)
                }
                .padding(.trailing, 8)
                .padding(.bottom, 8)
            }
        }
        .padding(.leading, 4)
        .padding(.trailing, 8)
        .padding(.vertical, 8)
        .background(
            Rectangle()
                .fill(_backgroundColor)
                .clipShape(
                    RoundedRectangle(cornerRadius: hasSubtitle ? 16 : 32, style: .continuous)
                )
        )
        .overlay(
            borderContent(withColor: _color, hasSubtitle: hasSubtitle, isCircle: isCircle)
        )
        .alert(L10n.Alert.errorOccurred,
               isPresented: Binding(get: { store.browserNotSupported },
                                    set: { _ in store.send(.dismissBrowserAlert) }),
               actions: {
                   Button(L10n.Alert.ok) {
                       store.send(.dismissBrowserAlert)
                   }
                   .neoGlassStyle(horizontalPadding: 20, verticalPadding: 8)
               }, message: {
                   Text(L10n.Screen.Help.browserNotSupported)
               })
        .accessibilityIdentifier("NeoInfoView")
    }
}

struct NewMessageInfoView: View {
    var body: some View {
        NeoInfoView(info: NeoGenericInfo(title: L10n.Status.newMessage, type: .newMessage))
    }
}

struct NeoInfoView_Previews: PreviewProvider {
    static var previews: some View {
        // Infobox
        let info = NeoGenericInfo(title: "Infobox",
                                  subtitle: "Hier gibt es einen Link mit dazugehörigem Erklärungstext.",
                                  type: .info)
        // Process Statuses
        let sent = ProcessStatusInfo(status: .processSent)
        let received = ProcessStatusInfo(status: .processReceived)
        let review = ProcessStatusInfo(status: .processUnderReview)
        let activityRequired = ProcessStatusInfo(status: .processActivityRequired)
        let completed = ProcessStatusInfo(status: .processCompleted)
        // Generic Info
        let dateAlert = NeoGenericInfo(title: "Antwort bis zum 28.10.2025", type: .warning, withSymbol: false)
        let helpInfo = NeoGenericInfo(title: "Verfügbarkeit für Frankfurt:",
                                      subtitle: "Voller 115 Service verfügbar",
                                      type: .success)
        let deadline = NeoGenericInfo(title: "Antwort bis zum 28.10.2025",
                                      subtitle: "Sie müssen mindestens eine Nachricht bis zum 28.10.2025 beantworten.",
                                      type: .danger)
        let attachmentInfo = NeoGenericInfo(title: "bestaetigung.pdf",
                                            type: .attachment,
                                            withSymbol: true)
        let attachmentCountInfo = NeoGenericInfo(title: "+2",
                                                 type: .attachment,
                                                 withSymbol: false)

        ScrollView {
            VStack(alignment: .leading) {
                // Infobox
                NeoInfoView(info: info, withLink: "https://fitko.de", andLabel: "Link")
                // Statuses
                NeoInfoView(info: sent)
                NeoInfoView(info: received)
                NeoInfoView(info: review)
                NeoInfoView(info: review, showInfo: true)
                NeoInfoView(info: activityRequired)
                NeoInfoView(info: completed)
                NewMessageInfoView()
                // Generic Info Boxes
                NeoInfoView(info: dateAlert)
                NeoInfoView(info: helpInfo)
                NeoInfoView(info: deadline, showInfo: true)
                    .frame(maxWidth: .infinity, alignment: .leading)
                // Attachments
                HStack {
                    NeoInfoView(info: attachmentInfo)
                    NeoInfoView(info: attachmentCountInfo)
                }
            }
            .frame(maxWidth: .infinity,
                   maxHeight: .infinity,
                   alignment: .topLeading)
            .padding()
        }
    }
}
