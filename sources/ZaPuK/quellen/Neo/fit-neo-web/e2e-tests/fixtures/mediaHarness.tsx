// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { createElement, type ReactElement } from 'react';
import { createRoot } from 'react-dom/client';
import AttachmentsSection from '../../src/components/AttachmentsSection/AttachmentsSection';
import Infobox from '../../src/components/Infobox/Infobox';
import { sanitizeMatrixHtml } from '../../src/lib/sanitizeMatrixHtml';

export async function getClientMediaContent(uri: string) {
  const store = new ClientStore({
    matrixHomeserver: 'https://matrix.example.com',
  });
  let sourceUrl: string | undefined;
  Object.assign(store, {
    client: {
      getMediaContent: async (source: { url: () => string }) => {
        sourceUrl = source.url();
        return new Uint8Array([1, 2, 3]).buffer;
      },
    },
  });
  const bytes = await store.getMediaContent(uri);
  return { sourceUrl, bytes: Array.from(new Uint8Array(bytes)) };
}

function mount(element: ReactElement) {
  const container = document.createElement('div');
  document.body.append(container);
  createRoot(container).render(element);
}

export function renderAttachment() {
  mount(
    createElement(AttachmentsSection, {
      attachments: [
        { id: 'file', name: 'report.txt', uri: 'mxc://server/file' },
      ],
      loadMediaContent: async () =>
        new TextEncoder().encode('file contents').buffer,
    }),
  );
}

export function renderFormattedImage() {
  mount(
    createElement(Infobox, {
      title: 'Image',
      body: '',
      bodyHtml: sanitizeMatrixHtml(
        '<img src="mxc://server/image" alt="Example">',
      ),
      loadMediaContent: async (uri: string) => {
        document.documentElement.dataset.mediaUri = uri;
        return new TextEncoder().encode(
          '<svg xmlns="http://www.w3.org/2000/svg"/>',
        ).buffer;
      },
    }),
  );
}
