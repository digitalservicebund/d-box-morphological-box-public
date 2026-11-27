// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef } from 'react';
import type { M3Menu } from '@banegasn/m3-menu';

/** Keeps M3 menus above scrolling containers and forwards each selection only once. */
export const useAnchoredMenu = (onSelect: (value: string) => void) => {
  const menu = useRef<M3Menu>(null);
  const popover = useRef<HTMLDivElement>(null);
  const anchor = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const element = menu.current;
    const choose = (event: Event) => {
      if (event.target !== element) return;
      onSelect((event as CustomEvent<{ value: string }>).detail.value);
    };
    const dismiss = () => popover.current?.hidePopover();
    element?.addEventListener('menu-item-select', choose);
    element?.addEventListener('menu-dismiss', dismiss);
    return () => {
      element?.removeEventListener('menu-item-select', choose);
      element?.removeEventListener('menu-dismiss', dismiss);
    };
  }, [onSelect]);
  const show = () => {
    const rect = anchor.current?.getBoundingClientRect();
    const layer = popover.current;
    const element = menu.current;
    if (!rect || !layer || !element) return;
    const upwards = rect.top > window.innerHeight / 2;
    layer.style.left = `${Math.max(16, Math.min(rect.left, window.innerWidth - 260))}px`;
    layer.style.top = `${upwards ? rect.top : rect.bottom}px`;
    element.placement = upwards ? 'top-start' : 'bottom-start';
    layer.showPopover();
    element.show('trigger');
  };
  return { menu, popover, anchor, show };
};
