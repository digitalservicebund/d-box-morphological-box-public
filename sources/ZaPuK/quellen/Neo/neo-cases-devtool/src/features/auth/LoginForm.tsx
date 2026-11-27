// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useState, type FC, type FormEvent } from 'react';
import { MdLogin, MdOutlineAccountBalance } from 'react-icons/md';
import { config } from '../../app/config';
import { Button } from '../../components/ui/Button';
import { TextField } from '../../components/ui/TextField';
import { errorMessage } from '../../utils/errors';
import { useNotify } from '../notifications/notification-context';
import { useAuth } from './auth-context';
import { ServerSelect } from './ServerSelect';
import { readServerPreference, writeServerPreference } from './server-preference';
import styles from './LoginForm.module.css';

export const LoginForm: FC = () => {
  const [homeserver, setHomeserver] = useState(() => readServerPreference(config.homeservers));
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const { login, phase } = useAuth();
  const notify = useNotify();
  const busy = phase === 'loggingIn' || phase === 'restoring';
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy || !username.trim() || !password) return;
    setError('');
    try {
      await login(username, password, homeserver);
    } catch (cause) {
      const message = errorMessage(cause);
      setError(message);
      notify(`Login failed: ${message}`, 'error');
    } finally {
      setPassword('');
    }
  };
  return (
    <div className={styles.center}>
      <div className={styles.intro}>
        <MdOutlineAccountBalance size={36} aria-hidden="true" />
        <h1>Neo Cases Devtool</h1>
        <p>Sign in to create cases and simulate agency dialogs.</p>
      </div>
      <form
        className={styles.form}
        onSubmit={(event) => void submit(event)}
        aria-label="Agency login"
        onKeyDown={(event) => {
          if (event.key === 'Enter') {
            event.preventDefault();
            event.currentTarget.requestSubmit();
          }
        }}
      >
        <ServerSelect
          value={homeserver}
          onChange={(value) => {
            setHomeserver(value);
            writeServerPreference(value);
            setError('');
          }}
          disabled={busy}
        />
        <TextField
          label="Username or MXID"
          name="username"
          value={username}
          onChange={setUsername}
          autoComplete="username"
          required
          disabled={busy}
        />
        <TextField
          label="Password"
          name="password"
          value={password}
          onChange={setPassword}
          type="password"
          autoComplete="current-password"
          required
          disabled={busy}
        />
        <Button icon={MdLogin} type="submit" fullWidth loading={busy} disabled={!username.trim() || !password || busy}>
          {phase === 'restoring' ? 'Restoring session…' : 'Log in'}
        </Button>
        {error && (
          <p className={styles.error} role="alert">
            {error}
          </p>
        )}
      </form>
    </div>
  );
};
