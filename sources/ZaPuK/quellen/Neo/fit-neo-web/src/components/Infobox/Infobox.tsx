// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useMatrixHtmlMedia } from '../../hooks/useMatrixHtmlMedia';
import type { LoadMediaContent } from '../../hooks/useMediaObjectUrl';
import Info from '../Icons/Info';
import Link from '../Link/Link';
import './infobox.css';

export interface InfoboxProps {
  title: string;
  body: string;
  /** Pre-sanitized HTML to render instead of `body`. */
  bodyHtml?: string;
  loadMediaContent?: LoadMediaContent;
  link?: {
    href: string;
    label: string;
  };
}

export default function Infobox({
  title,
  body,
  bodyHtml,
  loadMediaContent,
  link,
}: InfoboxProps) {
  const bodyRef = useMatrixHtmlMedia(bodyHtml, loadMediaContent);
  return (
    <div className="infobox">
      <div className="infobox-header">
        <Info />
        <strong>{title}</strong>
      </div>
      <div className="infobox-body-section">
        {bodyHtml ? (
          <p
            ref={bodyRef}
            className="infobox-body"
            dangerouslySetInnerHTML={{ __html: bodyHtml }}
          />
        ) : (
          <p className="infobox-body">{body}</p>
        )}
        {link && (
          <div className="infobox-footer">
            <Link href={link.href}>{link.label}</Link>
          </div>
        )}
      </div>
    </div>
  );
}
