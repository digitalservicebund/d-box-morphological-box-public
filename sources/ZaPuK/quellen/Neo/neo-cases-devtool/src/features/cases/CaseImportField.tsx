// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, type FC } from 'react';
import { MdUploadFile, MdClose } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import styles from './CaseImportField.module.css';

export interface CaseImportFieldProps {
  name: string;
  error: string;
  loading: boolean;
  count?: number;
  disabled: boolean;
  onFile: (file?: File) => Promise<void>;
}

export const CaseImportField: FC<CaseImportFieldProps> = ({ name, error, loading, count, disabled, onFile }) => {
  const ref = useRef<HTMLInputElement>(null);
  return (
    <div className={styles.field}>
      <span className={styles.label}>Import agency dialogs (optional)</span>
      <input
        ref={ref}
        type="file"
        hidden
        accept=".json,application/json"
        aria-label="Case JSON import"
        disabled={disabled || loading}
        onChange={(event) => void onFile(event.currentTarget.files?.[0])}
      />
      <div className={styles.actions}>
        <Button
          icon={MdUploadFile}
          variant="outlined"
          loading={loading}
          disabled={disabled || loading}
          onClick={() => ref.current?.click()}
        >
          Choose JSON file
        </Button>
        {name && (
          <Button
            icon={MdClose}
            variant="text"
            label="Remove import file"
            danger
            disabled={disabled || loading}
            onClick={() => {
              if (ref.current) ref.current.value = '';
              void onFile();
            }}
          />
        )}
      </div>
      {name && <span>{name}</span>}
      {count !== undefined && (
        <small>
          {count} agency {count === 1 ? 'dialog' : 'dialogs'} ready to import. You can change the case details below.
        </small>
      )}
      {error && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}
    </div>
  );
};
