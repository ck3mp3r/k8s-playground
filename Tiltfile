load('./src/tilt/vault.py', 'vault_deploy') # type: ignore
load('./src/tilt/oam.py', 'oam_deploy') # type: ignore
load('./src/tilt/kafka.py', 'kafka_deploy') # type: ignore

update_settings(  #type: ignore
  max_parallel_updates = 4,
  k8s_upsert_timeout_secs = 60,
  suppress_unused_image_warnings = None
)

vault_deploy( # type: ignore
  values_file="./src/helm/vault/values.yaml",
  secrets={
    "cubbyhole/myapp":{
              "foo":"bar",
              "bar":"baz",
            }
  }
) 

oam_deploy( # type: ignore
  values_file="./src/helm/oam/values.yaml"
)

kafka_deploy( # type: ignore
  values_file="./src/helm/kafka/values.yaml"
)
