// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState } from 'react';
import type { Profile } from '../../matrix/types';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { profileInputSchema } from './profile-validation';

export const useProfileEditor = (profile: Profile, onClose: () => void) => {
  const [displayName, setDisplayName] = useState(profile.displayName ?? '');
  const [avatar, setAvatar] = useState<File>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const saving = useRef(false);
  const { service, refreshProfile, connected } = useAuth();
  const notify = useNotify();
  const save = async () => {
    if (!service || saving.current || !connected) return;
    const result = profileInputSchema.safeParse({ displayName, avatar });
    if (!result.success) {
      setError(result.error.issues.map((issue) => issue.message).join('\n'));
      return;
    }
    saving.current = true;
    setBusy(true);
    setError('');
    let attempted = false;
    try {
      if (result.data.avatar && profile.canSetAvatar) {
        const image = await createImageBitmap(result.data.avatar).catch(() => {
          throw new Error('The selected file is not a readable photo.');
        });
        image.close();
      }
      if (profile.canSetDisplayName && result.data.displayName !== (profile.displayName ?? '')) {
        attempted = true;
        await service.setDisplayName(result.data.displayName);
      }
      if (result.data.avatar && profile.canSetAvatar) {
        attempted = true;
        await service.setAvatar(new Uint8Array(await result.data.avatar.arrayBuffer()), result.data.avatar.type);
      }
      await refreshProfile();
      notify('Profile updated.', 'success');
      onClose();
    } catch (cause) {
      if (attempted) await refreshProfile().catch(() => undefined);
      const message = errorMessage(cause);
      setError(message);
      notify(`Could not finish updating the profile: ${message}`, 'error');
    } finally {
      saving.current = false;
      setBusy(false);
    }
  };
  return { displayName, setDisplayName, avatar, setAvatar, busy, error, save, connected };
};
