apiVersion: core.oam.dev/v1beta1
kind: TraitDefinition
metadata:
  name: database
  namespace: vela-system
spec:
  appliesToWorkloads: 
    - "*"
  schematic:
    cue:
      template: |
        {{ .Files.Get "cue/traits/db.cue" | nindent 8 }}
