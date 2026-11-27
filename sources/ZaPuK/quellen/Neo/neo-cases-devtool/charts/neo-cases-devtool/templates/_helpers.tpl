{{/* SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation) */}}
{{/* SPDX-License-Identifier: EUPL-1.2 */}}

{{- define "neo-cases-devtool.name" -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "neo-cases-devtool.selectorLabels" -}}
app.kubernetes.io/name: {{ .Chart.Name }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{- define "neo-cases-devtool.labels" -}}
{{ include "neo-cases-devtool.selectorLabels" . }}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}
