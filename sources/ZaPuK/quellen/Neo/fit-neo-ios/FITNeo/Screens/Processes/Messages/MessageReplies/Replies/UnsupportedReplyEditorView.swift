//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import MatrixRustSDK
import SwiftUI

struct UnsupportedReplyEditorView: ReplyEditorView {
    typealias Feature = UnsupportedReplyFeature

    let store: StoreOf<UnsupportedReplyFeature>

    init(store: StoreOf<UnsupportedReplyFeature>) {
        self.store = store
    }

    var body: some View {
        ScrollView {
            VStack {
                NeoEmptyView()
            }
        }
    }

    static func answerView(_ envelope: NeoEnvelope, inquiry: NeoContent?) -> some View {
        ScrollView {
            VStack {
                NeoEmptyView()
            }
        }
    }

    static func confirmationView(_ contents: [NeoContent], inquiry: NeoContent?) -> some View {
        ScrollView {
            VStack {
                NeoEmptyView()
            }
        }
    }
}
