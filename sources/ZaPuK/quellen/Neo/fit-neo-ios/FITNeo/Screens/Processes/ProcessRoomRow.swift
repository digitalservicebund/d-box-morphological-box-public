//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct ProcessRoomRow: View {
    let process: ProcessRoom

    var body: some View {
        HStack(alignment: .center) {
            VStack(alignment: .leading) {
                if let status = process.status {
                    let statusInfo = ProcessStatusInfo(status: status)
                    NeoInfoView(info: statusInfo)
                        .padding(.leading, 16)
                        .padding(.top, 16)
                        .accessibilityIdentifier("ProcessRow.\(process.id).Status")
                }
                HStack(alignment: .center) {
                    VStack(alignment: .leading) {
                        Text(process.processName)
                            .textStyle(.titleDefault)
                            .lineLimit(3)
                            .accessibilityIdentifier("ProcessRow.\(process.id).Name")

                        Text(process.authorityName)
                            .textStyle(.bodyDefaultRegular)
                            .lineLimit(3)
                            .accessibilityIdentifier("ProcessRow.\(process.id).Authority")
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
                .padding(EdgeInsets(top: process.status != nil ? 8 : 16, leading: 16, bottom: 16, trailing: 16))
            }
            Spacer()
            Image(systemSymbol: .chevronRight)
                .scaleEffect(1.5)
                .foregroundStyle(Asset.Theme.actionsDefault.swiftUIColor)
                .padding(24)
        }
        .accessibilityIdentifier("ProcessRow.\(process.id)")
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Asset.Theme.layoutBackgroundDefault.swiftUIColor)
        )
    }
}

struct ProcessRoomRow_Previews: PreviewProvider {
    static var previews: some View {
        let process1 = ProcessRoom(id: "room-id",
                                   name: "Fishing license request",
                                   authority: "Ministry of fishing",
                                   status: ProcessStatus.processSent)
        ProcessRoomRow(process: process1)

        let process2 = ProcessRoom(id: "room-id",
                                   name: "Fishing license request",
                                   authority: "Ministry of fishing")
        ProcessRoomRow(process: process2)
    }
}
