// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, useState } from 'react';
import type { MatrixEvent } from '../../domain/neo/types';
import { keys, types } from '../../domain/neo/identifiers';
import { object } from '../../domain/neo/relations';
import { useAuth } from '../../features/auth/auth-context';
import { useNotify } from '../../features/notifications/notification-context';
import { errorMessage } from '../../utils/errors';
import { textRepresentations } from '../../utils/rich-text';

export const useMedia = (event: MatrixEvent) => {
  const [loading, setLoading] = useState(false);
  const [url, setUrl] = useState('');
  const currentUrl = useRef('');
  const mounted = useRef(true);
  const { service } = useAuth();
  const notify = useNotify();
  const file = object(event.content[types.file]);
  const filename = typeof file.name === 'string' ? file.name : 'attachment';
  const title = textRepresentations(event.content[keys.title]).plain || filename;
  const mime = typeof file.mimetype === 'string' ? file.mimetype : 'application/octet-stream';
  const safeName = filename.replace(/[^\p{L}\p{N}._ -]/gu, '_');
  useEffect(
    () => () => {
      mounted.current = false;
      if (currentUrl.current) URL.revokeObjectURL(currentUrl.current);
    },
    [],
  );
  const download = async () => {
    if (!service || loading) return;
    setLoading(true);
    try {
      const data = await service.download({ url: file.uri, ...object(file[keys.encryptedFile]) });
      if (!mounted.current) return;
      const source = URL.createObjectURL(new Blob([new Uint8Array(data)], { type: mime }));
      if (currentUrl.current) URL.revokeObjectURL(currentUrl.current);
      currentUrl.current = source;
      setUrl(source);
      if (event.type === types.file) {
        const anchor = document.createElement('a');
        anchor.href = source;
        anchor.download = safeName;
        anchor.click();
      }
    } catch (cause) {
      notify(`Attachment unavailable: ${errorMessage(cause)}`, 'error');
    } finally {
      if (mounted.current) setLoading(false);
    }
  };
  return { title, mime, file, url, loading, download };
};
