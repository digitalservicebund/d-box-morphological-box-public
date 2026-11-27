//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import SwiftUI

struct NeoAttachmentView: View {
    var tapHandler: (() -> Void)?

    let filename: String
    let filesize: Int

    init(_ attachment: AttachmentType, onAttachmentTapped: (() -> Void)? = nil) {
        filename = attachment.filename
        filesize = attachment.filesize
        tapHandler = onAttachmentTapped
    }

    @ViewBuilder
    func attachmentView() -> some View {
        HStack(alignment: .center) {
            ZStack {
                Rectangle()
                    .clipShape(RoundedRectangle(cornerRadius: 8, style: .circular))
                    .foregroundColor(Asset.Theme.feedbackLoaderBackground.swiftUIColor)
                    .frame(height: 48)
                    .frame(width: 48)

                Image(systemSymbol: .paperclip)
                    .scaleEffect(1.2)
            }
            .padding(.leading, 16)
            .padding(.top, 16)
            .padding(.bottom, 16)

            VStack(alignment: .leading) {
                Text(filename)
                    .textStyle(.bodyDefaultSemiBold)
                    .padding(.top, 8)
                Text(
                    filesize.formatted(.byteCount(style: .file))
                )
                .textStyle(.bodySmallRegular)
                .padding(.top, -8)
                .padding(.bottom, 8)
            }
            .padding(.leading, 8)
            .padding(.trailing, 16)

            Spacer()
        }
        .contentShape(Rectangle())
        .overlay {
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .stroke(Asset.Theme.divider.swiftUIColor)
        }
    }

    var body: some View {
        Button {
            tapHandler?()
        } label: {
            attachmentView()
        }
        .buttonStyle(.plain)
    }
}

struct NeoAttachmentView_Previews: PreviewProvider {
    static var previews: some View {
        let attach = AttachmentType(url: "https://test.url", filename: "file1.pdf", filesize: 1_200_000)
        NeoAttachmentView(attach)
    }
}
