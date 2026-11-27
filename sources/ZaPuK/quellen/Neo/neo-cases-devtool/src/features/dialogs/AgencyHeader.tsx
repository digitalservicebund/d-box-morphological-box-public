// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdOutlineAccountBalance } from 'react-icons/md';
import { UserAvatar } from '../profile/UserAvatar';
import { useSenderProfile } from '../profile/use-sender-profile';
import styles from './DialogCard.module.css';

export interface AgencyHeaderProps {
  sender: string;
  timestamp: number;
}

export const AgencyHeader: FC<AgencyHeaderProps> = ({ sender, timestamp }) => {
  const profile = useSenderProfile(sender);
  return (
    <div slot="header" className={styles.header}>
      <UserAvatar avatarUrl={profile.avatarUrl} fallback={MdOutlineAccountBalance} />
      <div>
        <div className={styles.name}>
          <strong>Agency: </strong>
          {profile.displayName || sender}
        </div>
        <span className={styles.sender}>{sender}</span>
      </div>
      <time dateTime={new Date(timestamp).toISOString()}>{new Date(timestamp).toLocaleString()}</time>
    </div>
  );
};
