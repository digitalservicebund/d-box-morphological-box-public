//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Foundation
import SwiftUI

@Reducer
struct OpenBrowserFeature {
    @ObservableState
    struct State: Equatable {
        var browserNotSupported = false
    }

    enum Action {
        case openWebsite(String)
        case browserOpenFailed
        case dismissBrowserAlert
    }

    @Dependency(\.openURL) var openURL

    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .openWebsite(let urlString):
                guard let url = URL(string: urlString) else {
                    return .none
                }
                return .run { send in
                    let accepted = await openURL(url)
                    if !accepted {
                        await send(.browserOpenFailed)
                    }
                }

            case .browserOpenFailed:
                state.browserNotSupported = true
                return .none

            case .dismissBrowserAlert:
                state.browserNotSupported = false
                return .none
            }
        }
    }
}

/// Adds OpenBrowserFeature as a child reducer to any parent feature.
extension Reducer {
    func openBrowser(state: WritableKeyPath<State, OpenBrowserFeature.State>,
                     action: CaseKeyPath<Action, OpenBrowserFeature.Action>) -> some Reducer<State, Action> {
        Scope(state: state, action: action) {
            OpenBrowserFeature()
        }
    }
}
