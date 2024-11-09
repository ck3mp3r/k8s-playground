// Parameters for the DatabaseTrait
parameter: {
  databaseName: string
  username:     string
  password:     string
}

// Secret to store PostgreSQL credentials
output: {
  apiVersion: "v1"
  kind: "Secret"
  metadata: {
    name: "\(parameter.databaseName)-credentials"
    namespace: context.namespace
  }
  type: "Opaque"
  data: {
    username: "\(parameter.username)"
    password: "\(parameter.password)"
    connectionString: "postgres://\(parameter.username):\(parameter.password)@\(parameter.databaseName)-postgres-service.\(context.namespace).svc.cluster.local:5432/\(parameter.databaseName)"
  }
}

// CloudNativePG PostgreSQL Cluster resource
pgCluster: {
  apiVersion: "postgresql.cnpg.io/v1"
  kind: "Cluster"
  metadata: {
    name: "\(parameter.databaseName)-postgres"
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
        passwordSecret: {
          name: "\(parameter.databaseName)-credentials"
        }
        databases: [parameter.databaseName]
      }
    ]
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
