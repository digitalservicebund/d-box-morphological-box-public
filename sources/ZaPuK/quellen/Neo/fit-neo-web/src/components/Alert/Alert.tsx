// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { type PropsWithChildren, useEffect, useId, useRef } from 'react';
import type { ButtonProps } from '../Button/Button';
import Button from '../Button/Button';
import './Alert.css';

export interface AlertProps {
  title: string;
  text?: string;
  primaryAction?: PropsWithChildren<Omit<ButtonProps, 'type' | 'icon'>>;
  secondaryAction?: PropsWithChildren<Omit<ButtonProps, 'type' | 'icon'>>;
  open?: boolean;
  onClose?: () => void;
}

export default function Alert({
  title,
  text,
  primaryAction,
  secondaryAction,
  children,
  open,
  onClose,
}: PropsWithChildren<AlertProps>) {
  const dialogRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    if (open) {
      if (!dialog.open) dialog.showModal();
    } else {
      if (dialog.open) dialog.close();
    }
  }, [open]);

  const onCloseRef = useRef(onClose);
  useEffect(() => {
    onCloseRef.current = onClose;
  });

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    const handler = () => onCloseRef.current?.();
    dialog.addEventListener('close', handler);
    return () => dialog.removeEventListener('close', handler);
  }, []);

  const titleId = useId();
  const textId = useId();

  return (
    <dialog
      ref={dialogRef}
      className="alert"
      aria-labelledby={titleId}
      aria-describedby={text ? textId : undefined}
      onClick={(e) => {
        if (e.target === dialogRef.current) dialogRef.current?.close();
      }}
    >
      <div className="dialog-header">
        <h2 id={titleId}>{title}</h2>
        <button
          className="close-button"
          aria-label="Schließen"
          onClick={() => dialogRef.current?.close()}
        >
          <svg
            width="14"
            height="14"
            viewBox="0 0 14 14"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fillRule="evenodd"
              clipRule="evenodd"
              d="M1.675 12.875L6.575 7.975L11.475 12.875C11.6583 13.0583 11.8917 13.15 12.175 13.15C12.4583 13.15 12.6917 13.0583 12.875 12.875C13.0583 12.6917 13.15 12.4583 13.15 12.175C13.15 11.8917 13.0583 11.6583 12.875 11.475L7.975 6.575L12.875 1.675C13.0583 1.49167 13.15 1.25833 13.15 0.975C13.15 0.691667 13.0583 0.458333 12.875 0.275C12.6917 0.0916667 12.4583 0 12.175 0C11.8917 0 11.6583 0.0916667 11.475 0.275L6.575 5.175L1.675 0.275C1.49167 0.0916667 1.25833 0 0.975 0C0.691667 0 0.458333 0.0916667 0.275 0.275C0.0916667 0.458333 0 0.691667 0 0.975C0 1.25833 0.0916667 1.49167 0.275 1.675L5.175 6.575L0.275 11.475C0.0916667 11.6583 0 11.8917 0 12.175C0 12.4583 0.0916667 12.6917 0.275 12.875C0.458333 13.0583 0.691667 13.15 0.975 13.15C1.25833 13.15 1.49167 13.0583 1.675 12.875Z"
              fill="currentColor"
            />
          </svg>
        </button>
      </div>
      <div className="dialog-body">
        {text && <p id={textId}>{text}</p>}
        {children}
      </div>
      {(primaryAction ?? secondaryAction) && (
        <div className="dialog-footer">
          {secondaryAction && <Button {...secondaryAction} type="danger" />}
          {primaryAction && <Button {...primaryAction} type="primary" />}
        </div>
      )}
    </dialog>
  );
}
