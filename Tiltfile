load('./src/tilt/vault.py', 'vault_deploy') # type: ignore
load('./src/tilt/oam.py', 'oam_deploy') # type: ignore

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
