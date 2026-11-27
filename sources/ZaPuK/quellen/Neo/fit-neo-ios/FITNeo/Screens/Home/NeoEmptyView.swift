//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct NeoEmptyView: View {
    var body: some View {
        VStack {
            NeoInfoView(
                info: NeoGenericInfo(title: L10n.Screen.notImplemented,
                                     type: .info)
            )
        }
        .padding()
    }
}

struct NeoEmptyView_Previews: PreviewProvider {
    static var previews: some View {
        NeoEmptyView()
    }
}
