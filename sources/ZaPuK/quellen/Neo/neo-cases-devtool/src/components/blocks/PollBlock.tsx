// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdRadioButtonUnchecked, MdCheckCircleOutline } from 'react-icons/md';
import { keys, types } from '../../domain/neo/identifiers';
import { object } from '../../domain/neo/relations';
import type { MatrixEvent } from '../../domain/neo/types';
import { RichText } from './RichText';
import styles from './Blocks.module.css';

export interface PollBlockProps {
  event: MatrixEvent;
  inquiry?: MatrixEvent;
}

export const PollBlock: FC<PollBlockProps> = ({ event, inquiry }) => {
  const response = event.type === types.pollResponse;
  const poll = object((inquiry ?? event).content[keys.poll]);
  const options = Array.isArray(poll.answers) ? poll.answers.map(object) : [];
  const selections = Array.isArray(event.content[keys.selections]) ? (event.content[keys.selections] as string[]) : [];
  return (
    <div className={styles.inquiry}>
      <span className={styles.label}>{response ? 'POLL RESPONSE' : 'POLL INQUIRY'}</span>
      {!response && <RichText value={poll.question} />}
      <ul className={styles.options}>
        {options
          .filter((answer) => !response || selections.includes(String(answer['m.id'])))
          .map((answer) => (
            <li key={String(answer['m.id'])}>
              {response ? <MdCheckCircleOutline aria-hidden="true" /> : <MdRadioButtonUnchecked aria-hidden="true" />}
              <RichText value={answer} plainOnly />
            </li>
          ))}
      </ul>
      {response && !selections.length && <p>No option selected.</p>}
      {!response && (
        <small>
          {Number(poll.max_selections ?? 1) === 1 ? 'Single selection' : 'Multiple selections'} · Free-text answer
          available
        </small>
      )}
    </div>
  );
};
