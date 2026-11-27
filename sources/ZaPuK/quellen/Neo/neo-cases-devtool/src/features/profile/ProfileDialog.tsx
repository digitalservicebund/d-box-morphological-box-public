// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, type FC } from 'react';
import type { M3Dialog } from '@banegasn/m3-dialog';
import { MdAddAPhoto, MdClose, MdSave } from 'react-icons/md';
import type { Profile } from '../../matrix/types';
import { Button } from '../../components/ui/Button';
import { TextField } from '../../components/ui/TextField';
import { UserAvatar } from './UserAvatar';
import { useProfileEditor } from './use-profile-editor';
import { avatarTypes } from './profile-validation';
import styles from './ProfileDialog.module.css';
import { ProfileSessionActions } from './ProfileSessionActions';

export interface ProfileDialogProps {
  profile: Profile;
  onClose: () => void;
}

export const ProfileDialog: FC<ProfileDialogProps> = ({ profile, onClose }) => {
  const editor = useProfileEditor(profile, onClose);
  const ref = useRef<M3Dialog>(null);
  const file = useRef<HTMLInputElement>(null);
  useEffect(() => {
    const dialog = ref.current;
    const prevent = (event: Event) => {
      if (event.target === dialog && editor.busy) event.preventDefault();
    };
    const close = (event: Event) => {
      if (event.target === dialog) onClose();
    };
    dialog?.addEventListener('dialog-request-close', prevent);
    dialog?.addEventListener('dialog-close', close);
    return () => {
      dialog?.removeEventListener('dialog-request-close', prevent);
      dialog?.removeEventListener('dialog-close', close);
    };
  }, [editor.busy, onClose]);
  return (
    <m3-dialog ref={ref} open headline="Edit profile">
      <form
        id="profile-form"
        className={styles.form}
        onSubmit={(event) => {
          event.preventDefault();
          void editor.save();
        }}
      >
        <div className={styles.photo}>
          <UserAvatar avatarUrl={profile.avatarUrl} file={editor.avatar} large />
          <div>
            <Button
              icon={MdAddAPhoto}
              variant="outlined"
              onClick={() => file.current?.click()}
              disabled={editor.busy || !profile.canSetAvatar}
            >
              Choose photo
            </Button>
            <p>{editor.avatar?.name ?? 'PNG, JPEG, WebP, or GIF · up to 10 MB'}</p>
          </div>
          <input
            ref={file}
            type="file"
            hidden
            accept={avatarTypes.join(',')}
            aria-label="New avatar photo"
            onChange={(event) => editor.setAvatar(event.currentTarget.files?.[0])}
            disabled={editor.busy || !profile.canSetAvatar}
          />
        </div>
        <TextField
          label="Display name"
          value={editor.displayName}
          onChange={editor.setDisplayName}
          disabled={editor.busy || !profile.canSetDisplayName}
        />
        <p className={styles.mxid}>{profile.mxid}</p>
        {(!profile.canSetAvatar || !profile.canSetDisplayName) && (
          <p className={styles.hint}>
            Your homeserver restricts profile editing. Disabled fields are managed by the server or account provider.
          </p>
        )}
        {editor.error && (
          <p className={styles.error} role="alert">
            {editor.error}
          </p>
        )}
      </form>
      <ProfileSessionActions disabled={editor.busy} />
      <Button slot="actions" icon={MdClose} variant="text" onClick={onClose} disabled={editor.busy}>
        Cancel
      </Button>
      <Button
        slot="actions"
        icon={MdSave}
        type="submit"
        form="profile-form"
        loading={editor.busy}
        disabled={editor.busy || !editor.connected || (!profile.canSetDisplayName && !profile.canSetAvatar)}
      >
        Save profile
      </Button>
    </m3-dialog>
  );
};
