load('ext://helm_resource', 'helm_resource', 'helm_repo')  # type: ignore

def vault_deploy(values_file='./vault/values.yaml', secrets={}):
    helm_repo('hashicorp', 'https://helm.releases.hashicorp.com')  # type: ignore

    helm_resource(  # type: ignore
        'vault',
        'hashicorp/vault',
        namespace="vault",
        pod_readiness='ignore',
        flags=[
            '--values=' + values_file,
            '--create-namespace',
        ],
        resource_deps=['hashicorp'],
    )

    local_resource(  # type: ignore
        'vault-init-and-unseal',
        cmd="""
        # Wait for the Vault pod to be in Running state
        sleep 10
        for i in $(seq 1 30); do
          POD_STATUS=$(kubectl get pod vault-0 --namespace=vault -o jsonpath='{.status.phase}')
          if [ "$POD_STATUS" == "Running" ]; then
            echo "vault-0 pod is running (even if not fully ready)"
            break
          else
            echo "vault-0 pod not running yet (status: $POD_STATUS), retrying in 10 seconds..."
            sleep 10
          fi
        done

        if [ "$POD_STATUS" != "Running" ]; then
          echo "vault-0 pod not running after retries, exiting."
          exit 1
        fi

        # Initialize Vault and store the unseal key and root token in Kubernetes secrets
        kubectl exec vault-0 --namespace=vault -- vault operator init -key-shares=1 -key-threshold=1 \
            | tee /tmp/vault-init-output.txt \
            | awk '/Unseal Key 1/ {print $4}' \
            | xargs -I {} kubectl create secret generic vault-unseal-secret --namespace=vault --from-literal=unseal-key={}

        # Extract the root token and store it in a secret
        cat /tmp/vault-init-output.txt \
            | awk '/Initial Root Token/ {print $4}' \
            | xargs -I {} kubectl create secret generic vault-root-token --namespace=vault --from-literal=root-token={}

        # Wait for the unseal secret to be created
        while ! kubectl get secret vault-unseal-secret --namespace=vault --no-headers; do
          echo "Waiting for unseal secret to be created..."
          sleep 5
        done

        # Unseal Vault
        kubectl exec vault-0 --namespace=vault -- vault operator unseal $(kubectl get secret vault-unseal-secret --namespace=vault -o jsonpath='{.data.unseal-key}' | base64 --decode)
        """,
        resource_deps=['vault'],
    )

    seed_cmds = ""

    for path, kv_pairs in secrets.items():
        kv_string = " ".join(["{}={}".format(key, value) for key, value in kv_pairs.items()])
        seed_cmds += 'vault kv put ' + path + ' ' + kv_string + '\n'

    local_resource(  # type: ignore
        'vault-seed-secrets',
        cmd = """
        # Wait for the Vault pod to be ready before seeding secrets
        for i in $(seq 1 30); do
          POD_READY=$(kubectl get pod vault-0 --namespace=vault -o jsonpath='{.status.containerStatuses[0].ready}')
          if [ "$POD_READY" == "true" ]; then
            echo "Vault is ready. Seeding secrets into cubbyhole..."
            break
          else
            echo "Vault not ready yet (status: $POD_READY), retrying in 10 seconds..."
            sleep 10
          fi
        done

        # Execute all commands in one shell session
        VAULT_TOKEN=$(kubectl get secret vault-root-token --namespace=vault -o jsonpath='{.data.root-token}' | base64 --decode)
        kubectl exec vault-0 --namespace=vault -- /bin/sh -c \"
        export VAULT_TOKEN=$VAULT_TOKEN
        __SEED_CMDS__
        \"
        """.replace("__SEED_CMDS__", seed_cmds),
        resource_deps=['vault-init-and-unseal'],
    )

    local_resource(  # type: ignore
        'vault-port-forward',
        serve_cmd='kubectl port-forward svc/vault 8200:8200 --namespace=vault',
        resource_deps=['vault'],
    )

