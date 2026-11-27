// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdOpenInNew } from 'react-icons/md';
import { keys, types } from '../../domain/neo/identifiers';
import { canonicalPayload } from '../../domain/neo/legacy-compatibility';
import { object } from '../../domain/neo/relations';
import type { MatrixEvent } from '../../domain/neo/types';
import { textRepresentations } from '../../utils/rich-text';
import { RichText } from './RichText';
import { MediaBlock } from './MediaBlock';
import { PollBlock } from './PollBlock';
import styles from './Blocks.module.css';

export interface BlockRendererProps {
  event: MatrixEvent;
  inquiry?: MatrixEvent;
}

export const BlockRenderer: FC<BlockRendererProps> = ({ event: wireEvent, inquiry }) => {
  const event = { ...wireEvent, ...canonicalPayload(wireEvent) };
  if ([types.file, types.image, types.audio, types.video].some((type) => type === event.type))
    return <MediaBlock event={event} />;
  if (event.type === types.pollInquiry || event.type === types.pollResponse)
    return <PollBlock event={event} inquiry={inquiry} />;
  if (event.type === types.button) {
    const link = object(event.content[keys.hyperlink]);
    return (
      <a className={styles.link} href={String(link.url)} target="_blank" rel="noopener noreferrer">
        {textRepresentations(link[keys.label]).plain}
        <MdOpenInNew aria-hidden="true" />
      </a>
    );
  }
  if (event.type === types.infobox) {
    const link = object(event.content[keys.hyperlink]);
    return (
      <m3-card variant="filled" width="full" className={styles.infobox}>
        {event.content[keys.title] !== undefined && (
          <strong>{textRepresentations(event.content[keys.title]).plain}</strong>
        )}
        <RichText value={event.content[keys.infoboxText]} />
        {typeof link.url === 'string' && link.url !== '' && (
          <a className={styles.link} href={link.url} target="_blank" rel="noopener noreferrer">
            {textRepresentations(link[keys.label]).plain}
            <MdOpenInNew aria-hidden="true" />
          </a>
        )}
      </m3-card>
    );
  }
  if (event.type === types.textInquiry || event.type === types.fileInquiry) {
    const settings = object(event.content[event.type]);
    return (
      <div className={styles.inquiry}>
        <span className={styles.label}>
          {event.type === types.fileInquiry ? 'FILE REQUESTED' : 'TEXT RESPONSE REQUESTED'}
        </span>
        <RichText value={event.content} />
        <strong>{textRepresentations(settings[keys.title]).plain}</strong>
        {settings[keys.label] !== undefined && <small>{textRepresentations(settings[keys.label]).plain}</small>}
      </div>
    );
  }
  return <RichText value={event.content} />;
};
