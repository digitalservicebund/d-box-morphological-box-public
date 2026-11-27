# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

# syntax=docker/dockerfile:1.4

# Build stage uses BUILDPLATFORM since vite outputs platform-independent
# static files (HTML/JS/CSS). Native Node modules (esbuild, etc.) run on the
# build host, not the target.
FROM --platform=$BUILDPLATFORM node:24-bookworm-slim AS app-builder

# Enable corepack (for yarn)
RUN corepack enable

WORKDIR /app

# Copy package files
COPY package.json yarn.lock .yarnrc.yml ./

# Install dependencies (will include arch-specific binaries; also resolves
# @matrix-g2x/neo-web-sdk from its GitLab npm registry, see .yarnrc.yml)
RUN yarn install --immutable

# Copy source files
COPY . .

# Build the application
RUN yarn vite build

# Build the end result which hosts the application using nginx
FROM ghcr.io/nginx/nginx-unprivileged:1.31.2-alpine AS prod

# Copy built static files to nginx html directory
COPY --from=app-builder /app/dist /usr/share/nginx/html

# Configure nginx for path-based client-side routing
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 8080
USER 101
CMD ["nginx", "-g", "daemon off;"]
