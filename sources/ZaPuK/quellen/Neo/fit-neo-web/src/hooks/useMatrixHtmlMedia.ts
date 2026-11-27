// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef } from 'react';
import type { LoadMediaContent } from './useMediaObjectUrl';

export function useMatrixHtmlMedia(
  html: string | undefined,
  loadMediaContent: LoadMediaContent | undefined,
) {
  const ref = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    const root = ref.current;
    if (!root || !html || !loadMediaContent) return;

    const controller = new AbortController();
    const objectUrls: string[] = [];

    for (const image of root.querySelectorAll<HTMLImageElement>('img')) {
      const uri =
        image.getAttribute('data-mxc-uri') ?? image.getAttribute('src');
      if (!uri?.startsWith('mxc://')) continue;
      image.setAttribute('data-mxc-uri', uri);
      image.removeAttribute('src');
      void loadMediaContent(uri, controller.signal)
        .then((bytes) => {
          if (controller.signal.aborted) return;
          const url = URL.createObjectURL(new Blob([bytes]));
          objectUrls.push(url);
          image.src = url;
        })
        .catch((error: unknown) => {
          if (!controller.signal.aborted) {
            console.error('Media load failed', error);
          }
        });
    }

    return () => {
      controller.abort();
      for (const url of objectUrls) URL.revokeObjectURL(url);
    };
  }, [html, loadMediaContent]);

  return ref;
}
