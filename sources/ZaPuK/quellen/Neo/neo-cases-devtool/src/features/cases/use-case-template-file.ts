// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, useState } from 'react';
import { parseCaseTemplate, type CaseTemplate } from '../../domain/neo/case-template';
import { errorMessage } from '../../utils/errors';
import { useNotify } from '../notifications/notification-context';

export const useCaseTemplateFile = (onMetadata: (metadata: CaseTemplate['case']) => void) => {
  const [template, setTemplate] = useState<CaseTemplate>();
  const [name, setName] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const generation = useRef(0);
  const notify = useNotify();
  useEffect(
    () => () => {
      ++generation.current;
    },
    [],
  );
  const load = async (file?: File) => {
    const epoch = ++generation.current;
    setTemplate(undefined);
    setError('');
    setName(file?.name ?? '');
    if (!file) {
      setLoading(false);
      return;
    }
    setLoading(true);
    try {
      if (file.size > 10 * 1024 * 1024) throw new Error('Select a JSON export smaller than 10 MB.');
      const parsed = parseCaseTemplate(JSON.parse(await file.text()));
      if (generation.current !== epoch) return;
      setTemplate(parsed);
      onMetadata(parsed.case);
    } catch (cause) {
      if (generation.current !== epoch) return;
      const message = errorMessage(cause);
      setError(message);
      notify(`Invalid case import: ${message}`, 'error');
    } finally {
      if (generation.current === epoch) setLoading(false);
    }
  };
  return { template, name, error, loading, load };
};
