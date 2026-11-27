// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import { NavLink } from 'react-router';
import './tabbar.css';

export interface TabItem {
  /** Route path (passed to NavLink `to`) */
  to: string;
  /** Display label */
  label: string;
  /** Partial aliases for the active check */
  aliases?: string[];
}

export interface TabBarProps {
  tabs: TabItem[];
}

export default function TabBar({ tabs }: TabBarProps) {
  const { t } = useTranslation();
  return (
    <nav className="tabbar" aria-label={t('mainNavigation', 'Hauptnavigation')}>
      <div className="tabbar-inner">
        <div className="tabbar-section">
          {tabs.map(({ to, label, aliases }) => (
            <NavLink
              key={to}
              to={to}
              end={to === '/'}
              className={({ isActive }) =>
                // Check if the current path matches exactly or contains fully or partially any of the aliases
                `tabbar-tab${isActive || (aliases && aliases.some((alias) => window.location.pathname.includes(alias))) ? ' tabbar-tab-active' : ''}`
              }
            >
              {label}
            </NavLink>
          ))}
        </div>
      </div>
    </nav>
  );
}
