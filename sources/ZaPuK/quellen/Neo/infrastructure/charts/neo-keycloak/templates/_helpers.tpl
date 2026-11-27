{{/*
SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: EUPL-1.2
*/}}
{{/* vim: set filetype=mustache: */}}
{{/*
Expand the name of the chart.
*/}}
{{- define "neo-keycloak.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "neo-keycloak.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "neo-keycloak.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "neo-keycloak.labels" -}}
helm.sh/chart: {{ include "neo-keycloak.chart" . }}
{{ include "neo-keycloak.selectorLabels" . }}
app.kubernetes.io/version: {{ .Values.keycloakx.image.tag | default .Chart.AppVersion | toString | trunc 63 | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "neo-keycloak.selectorLabels" -}}
app.kubernetes.io/name: {{ include "neo-keycloak.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}
