# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

FROM quay.io/keycloak/keycloak:26.2.2
ARG EXTENSION_BUNDID_VERSION=2.4.3-26.2
ADD --chown=keycloak:keycloak --chmod=644 "https://repo1.maven.org/maven2/de/arbeitsagentur/opdt/keycloak-extension-bundid/$EXTENSION_BUNDID_VERSION/keycloak-extension-bundid-$EXTENSION_BUNDID_VERSION.jar" /opt/keycloak/providers/keycloak-extension-bundid.jar
RUN /opt/keycloak/bin/kc.sh build --http-relative-path=/auth --health-enabled=true --http-management-relative-path=/auth --metrics-enabled=true
