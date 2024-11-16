parameter: {
  databaseName: string
  username: string | *"defaultuser"
}

outputs: PostgresCluster: {
  apiVersion: "postgresql.cnpg.io/v1"
  kind: "Cluster"
  metadata: {
    name: "\(parameter.databaseName)-postgres-cluster"
    namespace: context.namespace
  }
  spec: {
    instances: 1
    primaryUpdateStrategy: "unsupervised"
    bootstrap: {
      initdb: {
        database: parameter.databaseName
        owner: parameter.username
        encoding: "UTF8"
      }
    }
    postgresql: {
      pg_hba: [
        "host all all 0.0.0.0/0 md5"
      ]
    }
    users: [
      {
        name: parameter.username
        managed: true
        databases: [parameter.databaseName]
      }
    ]
    bootstrap: {
      initdb: {
        database: parameter.databaseName
        owner: parameter.username
        encoding: "UTF8"
      }
    }
    service: {
      type: "ClusterIP"
      primary: {
        name: "\(parameter.databaseName)-postgres-service"
      }
    }
    storage: {
      size: "1Gi"
    }
  }
}
