// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useMemo, useRef, type CSSProperties } from 'react';

interface FileImageProps {
  file: File;
  alt: string;
  className?: string;
  style?: CSSProperties;
}

export default function FileImage({
  file,
  alt,
  className,
  style,
}: FileImageProps) {
  const blobUrl = useMemo(() => URL.createObjectURL(file), [file]);
  const activeUrl = useRef<string | null>(null);

  useEffect(() => {
    activeUrl.current = blobUrl;
    return () => {
      activeUrl.current = null;
      queueMicrotask(() => {
        if (activeUrl.current !== blobUrl) {
          URL.revokeObjectURL(blobUrl);
        }
      });
    };
  }, [blobUrl]);

  return <img src={blobUrl} alt={alt} className={className} style={style} />;
}
