// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useState } from 'react';

export type LoadMediaContent = (
  uri: string,
  signal?: AbortSignal,
) => Promise<ArrayBuffer>;

export function useMediaObjectUrl(
  uri: string | undefined,
  mimeType: string | undefined,
  loadMediaContent: LoadMediaContent | undefined,
): string | undefined {
  const [result, setResult] = useState<{
    uri: string;
    mimeType: string | undefined;
    loader: LoadMediaContent;
    url: string;
  }>();

  useEffect(() => {
    if (!uri || !loadMediaContent) return;

    const controller = new AbortController();
    let objectUrl: string | undefined;
    let active = true;

    void loadMediaContent(uri, controller.signal)
      .then((bytes) => {
        if (!active) return;
        objectUrl = URL.createObjectURL(new Blob([bytes], { type: mimeType }));
        setResult({ uri, mimeType, loader: loadMediaContent, url: objectUrl });
      })
      .catch((error: unknown) => {
        if (!controller.signal.aborted)
          console.error('Media load failed', error);
      });

    return () => {
      active = false;
      controller.abort();
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [uri, mimeType, loadMediaContent]);

  return result &&
    result.uri === uri &&
    result.mimeType === mimeType &&
    result.loader === loadMediaContent
    ? result.url
    : undefined;
}
