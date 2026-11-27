//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct NeoHorizontalDivider: View {
    var body: some View {
        Rectangle()
            .fill(Asset.Theme.divider.swiftUIColor)
            .frame(height: 1)
            .frame(maxWidth: .infinity)
    }
}

struct NeoVerticalDivider: View {
    var body: some View {
        Rectangle()
            .fill(Asset.Theme.divider.swiftUIColor)
            .frame(width: 1)
            .frame(maxHeight: .infinity)
    }
}
