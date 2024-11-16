{{- define "helpers.fullname" -}}
{{- printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "helpers.name" -}}
{{- .Chart.Name -}}
{{- end -}}

{{- define "helpers.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version -}}
{{- end -}}

{{- define "helpers.labels" -}}
helm.sh/chart: {{ include "helpers.chart" . }}
{{ include "helpers.selectorLabels" . }}
{{- end -}}

{{- define "helpers.selectorLabels" -}}
app.kubernetes.io/name: {{ include "helpers.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ .Chart.AppVersion }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{- define "helpers.serviceAccountName" -}}
{{- if .Values.serviceAccount.create -}}
    {{- default (include "helpers.fullname" .) .Values.serviceAccount.name -}}
{{- else -}}
    {{- default "default" .Values.serviceAccount.name -}}
{{- end -}}
{{- end -}}

{{- define "helpers.imagePullSecrets" -}}
{{- if .Values.imagePullSecrets -}}
  imagePullSecrets:
  {{- toYaml .Values.imagePullSecrets | nindent 2 }}
{{- end -}}
{{- end -}}

{{- define "helpers.deploymentName" -}}
{{- printf "%s-%s" (include "helpers.fullname" .) "deployment" -}}
{{- end -}}

{{- define "helpers.configmapName" -}}
{{- printf "%s-%s" (include "helpers.fullname" .) "configmap" -}}
{{- end -}}
