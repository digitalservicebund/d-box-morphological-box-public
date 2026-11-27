// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, type FC } from 'react';
import { MdUploadFile } from 'react-icons/md';
import { TextField } from '../../ui/TextField';
import { Button } from '../../ui/Button';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const MediaBlockEditor: FC<EditorProps> = ({ block, update, disabled }) => {
  const input = useRef<HTMLInputElement>(null);
  return (
    <div className={styles.fields}>
      <input
        ref={input}
        type="file"
        hidden
        accept={block.kind === 'file' ? undefined : `${block.kind}/*`}
        onChange={(event) => update({ file: event.target.files?.[0] })}
        disabled={disabled}
        aria-label="Attachment"
      />
      <div className={styles.file}>
        <Button icon={MdUploadFile} variant="outlined" onClick={() => input.current?.click()} disabled={disabled}>
          Choose file
        </Button>
        <span>{block.file?.name ?? 'No file selected'}</span>
      </div>
      <TextField
        label="Title (optional)"
        value={block.title}
        onChange={(title) => update({ title })}
        disabled={disabled}
      />
      {block.kind === 'image' && (
        <TextField
          label="Alternative text"
          value={block.text}
          onChange={(text) => update({ text })}
          disabled={disabled}
        />
      )}
      <small>A neutral filename will be sent. Encryption follows the case setting.</small>
    </div>
  );
};
