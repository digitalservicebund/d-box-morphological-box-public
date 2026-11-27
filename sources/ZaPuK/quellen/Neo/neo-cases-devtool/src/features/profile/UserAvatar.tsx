// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useState, type FC } from 'react';
import { MdPersonOutline } from 'react-icons/md';
import type { IconType } from 'react-icons';
import { useAuth } from '../auth/auth-context';
import styles from './UserAvatar.module.css';
import { acquireAvatar } from './avatar-source';

export interface UserAvatarProps {
  avatarUrl: string | null;
  file?: File;
  large?: boolean;
  fallback?: IconType;
}

export const UserAvatar: FC<UserAvatarProps> = ({ avatarUrl, file, large, fallback: Fallback = MdPersonOutline }) => {
  const [source, setSource] = useState('');
  const { service } = useAuth();
  useEffect(() => {
    let cancelled = false;
    setSource('');
    if (file) {
      const url = URL.createObjectURL(file);
      setSource(url);
      return () => URL.revokeObjectURL(url);
    }
    if (!service || !avatarUrl) return;
    const avatar = acquireAvatar(service, avatarUrl);
    void avatar.source
      .then((url) => {
        if (!cancelled) setSource(url);
      })
      .catch(() => undefined);
    return () => {
      cancelled = true;
      avatar.release();
    };
  }, [avatarUrl, file, service]);
  return (
    <span className={large ? styles.large : styles.avatar}>
      {source ? <img src={source} alt="" onError={() => setSource('')} /> : <Fallback aria-hidden="true" />}
    </span>
  );
};
