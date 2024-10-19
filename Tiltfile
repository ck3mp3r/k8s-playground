load('./tilt/vault.py', 'vault_deploy') # type: ignore
load('./tilt/oam.py', 'oam_deploy') # type: ignore

vault_deploy( # type: ignore
  values_file="./vault/values.yaml",
  secrets={
    "cubbyhole/myapp":{
              "foo":"bar",
              "bar":"baz",
            }
  }
) 

oam_deploy( # type: ignore
  values_file="./oam/values.yaml"
)
