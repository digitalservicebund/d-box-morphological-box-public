// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdAdd, MdDeleteOutline } from 'react-icons/md';
import { Textarea } from '../../ui/Textarea';
import { TextField } from '../../ui/TextField';
import { Button } from '../../ui/Button';
import { Checkbox } from '../../ui/Checkbox';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const PollBlockEditor: FC<EditorProps> = ({ block, update, disabled }) => (
  <div className={styles.fields}>
    <Textarea label="Question" value={block.text} onChange={(text) => update({ text })} disabled={disabled} rows={2} />
    {block.answers.map((answer, index) => (
      <div className={styles.option} key={answer.id}>
        <TextField
          label={`Option ${index + 1}`}
          value={answer.text}
          disabled={disabled}
          onChange={(text) =>
            update({ answers: block.answers.map((item) => (item.id === answer.id ? { ...item, text } : item)) })
          }
        />
        <Button
          icon={MdDeleteOutline}
          danger
          variant="text"
          label={`Remove option ${index + 1}`}
          disabled={disabled}
          onClick={() => update({ answers: block.answers.filter((item) => item.id !== answer.id) })}
        />
      </div>
    ))}
    <div>
      <Button
        icon={MdAdd}
        variant="text"
        disabled={disabled}
        onClick={() => update({ answers: [...block.answers, { id: crypto.randomUUID(), text: '' }] })}
      >
        Add option
      </Button>
    </div>
    <Checkbox
      label="Allow multiple selections"
      checked={block.multiple}
      onChange={(multiple) => update({ multiple })}
      disabled={disabled}
    />
    <TextField
      label="Free-text response placeholder (optional)"
      value={block.label}
      onChange={(label) => update({ label })}
      disabled={disabled}
    />
    <small>The required “Other answer” option is included automatically.</small>
  </div>
);
