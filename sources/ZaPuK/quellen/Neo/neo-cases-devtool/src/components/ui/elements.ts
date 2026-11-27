// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import '@banegasn/m3-button';
import '@banegasn/m3-card';
import '@banegasn/m3-text-field';
import '@banegasn/m3-checkbox';
import '@banegasn/m3-menu';
import '@banegasn/m3-dialog';
import '@banegasn/m3-list';
import '@banegasn/m3-top-app-bar';
import '@banegasn/m3-snackbar';
import '@banegasn/m3-search-bar';
import type { DetailedHTMLProps, HTMLAttributes } from 'react';

type ElementProps<Tag extends keyof HTMLElementTagNameMap> = DetailedHTMLProps<
  HTMLAttributes<HTMLElementTagNameMap[Tag]>,
  HTMLElementTagNameMap[Tag]
>;

declare module 'react' {
  namespace JSX {
    interface IntrinsicElements {
      'm3-search-bar': ElementProps<'m3-search-bar'> & {
        value?: string;
        placeholder?: string;
        disabled?: boolean;
        name?: string;
      };
      'm3-button': ElementProps<'m3-button'> & {
        variant?: string;
        size?: string;
        disabled?: boolean;
        loading?: boolean;
        'full-width'?: boolean;
        'icon-only'?: boolean;
        type?: string;
        form?: string;
      };
      'm3-text-field': ElementProps<'m3-text-field'> & {
        label?: string;
        value?: string;
        type?: string;
        disabled?: boolean;
        required?: boolean;
        name?: string;
        placeholder?: string;
        autocomplete?: string;
        error?: boolean;
        'error-text'?: string;
        'helper-text'?: string;
        variant?: string;
      };
      'm3-checkbox': ElementProps<'m3-checkbox'> & {
        checked?: boolean;
        disabled?: boolean;
        label?: string;
        name?: string;
      };
      'm3-card': ElementProps<'m3-card'> & { variant?: string; width?: 'auto' | 'full' | 'fixed' };
      'm3-list': ElementProps<'m3-list'>;
      'm3-list-item': ElementProps<'m3-list-item'> & {
        selected?: boolean;
        disabled?: boolean;
        lines?: string;
        shape?: string;
      };
      'm3-top-app-bar': ElementProps<'m3-top-app-bar'> & { variant?: string };
      'm3-dialog': ElementProps<'m3-dialog'> & { open?: boolean; headline?: string };
      'm3-menu': ElementProps<'m3-menu'> & { open?: boolean; placement?: string };
      'm3-menu-item': ElementProps<'m3-menu-item'> & { value?: string; disabled?: boolean };
      'm3-snackbar': ElementProps<'m3-snackbar'> & {
        open?: boolean;
        message?: string;
        duration?: number;
        lines?: string;
        live?: 'polite' | 'assertive';
      };
    }
  }
}
