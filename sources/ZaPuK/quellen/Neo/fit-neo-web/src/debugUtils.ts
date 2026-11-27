// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoDialog } from '@matrix-g2x/neo-web-sdk/web';

// Shows the empty "no messages" state instead of real case data
export const debugMode = false;

// Shows a dummy CaseDetail with an Infobox + attachments for UI testing
export const debugCaseDetail = false;

export const debugOrgName = 'Zentrale Zulassungsstelle für Drachenhaltung';

const debugTimestamp = BigInt(new Date(2025, 8, 15, 13, 20).getTime());

export const debugDialog: NeoDialog = {
  id: '$debug-dialog-id',
  message: {
    id: '$debug-envelope-id',
    timestamp: debugTimestamp,
    blocks: [
      {
        id: '$debug-block-infobox-1',
        sender: '@max.mustermann:example.com',
        timestamp: debugTimestamp,
        content: new NeoContent.Infobox({
          title: { plain: 'Bestätigung Vorgangserstellung', html: undefined },
          infobox: {
            plain:
              'Sehr geehrte Frau Mustermann, hiermit möchten wir Ihnen eine wichtige Information zu Ihrem Vorgang mitteilen',
            html: undefined,
          },
          hyperlink: { url: '', label: { plain: '', html: undefined } },
        }),
      },
      {
        id: '$debug-block-file-1',
        sender: '@max.mustermann:example.com',
        timestamp: debugTimestamp,
        content: new NeoContent.File({
          title: { plain: 'Bestätigung', html: undefined },
          file: {
            uri: 'mxc://example.com/debugfile1',
            name: 'Bestätigung.pdf',
            mimetype: 'application/pdf',
            size: BigInt(482_304),
          },
        }),
      },
      {
        id: '$debug-block-file-2',
        sender: '@max.mustermann:example.com',
        timestamp: debugTimestamp,
        content: new NeoContent.File({
          title: { plain: 'Nachweis', html: undefined },
          file: {
            uri: 'mxc://example.com/debugfile2',
            name: 'Nachweis.pdf',
            mimetype: 'application/pdf',
            size: BigInt(128_954),
          },
        }),
      },
      {
        id: '$debug-block-file-3',
        sender: '@max.mustermann:example.com',
        timestamp: debugTimestamp,
        content: new NeoContent.File({
          title: { plain: 'Bescheid', html: undefined },
          file: {
            uri: 'mxc://example.com/debugfile3',
            name: 'Bescheid.pdf',
            mimetype: 'application/pdf',
            size: BigInt(957_112),
          },
        }),
      },
    ],
  },
  answers: [],
};

export async function debugUploadMedia(
  _data: ArrayBuffer,
  _mimeType: string,
): Promise<string> {
  // Simulate a 1-second upload and return a fake MXC URI
  await new Promise((r) => setTimeout(r, 1000));
  return 'mxc://example.com/debugfakeupload123';
}
