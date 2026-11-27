// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdDownload, MdAttachFile } from 'react-icons/md';
import { types } from '../../domain/neo/identifiers';
import type { MatrixEvent } from '../../domain/neo/types';
import { textRepresentations } from '../../utils/rich-text';
import { Button } from '../ui/Button';
import styles from './Blocks.module.css';
import { useMedia } from './use-media';

export interface MediaBlockProps {
  event: MatrixEvent;
}

export const MediaBlock: FC<MediaBlockProps> = ({ event }) => {
  const { title, mime, file, url, loading, download } = useMedia(event);
  return (
    <div className={styles.media}>
      <div className={styles.mediaHeader}>
        <MdAttachFile aria-hidden="true" />
        <div>
          <strong>{title}</strong>
          <small>
            {mime}
            {typeof file.size === 'number' ? ` · ${new Intl.NumberFormat().format(file.size)} bytes` : ''}
          </small>
        </div>
        <Button icon={MdDownload} variant="text" loading={loading} onClick={() => void download()}>
          {event.type === types.file ? 'Download' : 'Load media'}
        </Button>
      </div>
      {url && event.type === types.image && (
        <img src={url} alt={textRepresentations(event.content['m.alt_text']).plain || title} />
      )}
      {url && event.type === types.video && <video src={url} controls aria-label={title} />}
      {url && event.type === types.audio && <audio src={url} controls aria-label={title} />}
    </div>
  );
};
