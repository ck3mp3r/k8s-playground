apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: postgres-test-app
  namespace: {{ .Release.Namespace }}
spec:
  components:
    - name: dummy-workload
      type: worker  # A minimal workload type; required by OAM but not used for the database itself
      properties:
        image: "busybox"
        cmd: ["sh", "-c", "sleep 6000"]
      traits:
        - type: labels
          properties:
            foo: bar

        - type: database
          properties:
            databaseName: "testdb"  # Specify the database name
            # Optional username; will default to "defaultuser" if omitted
