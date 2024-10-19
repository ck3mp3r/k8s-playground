# Import Tilt's helm extension
load('ext://helm_resource', 'helm_resource', 'helm_repo')

helm_repo('hashicorp', 'https://helm.releases.hashicorp.com')

# Deploy Vault using Helm with the Vault Helm chart from the repo, automatically creating the namespace
helm_resource(
    'vault',
    'hashicorp/vault',
    namespace="vault",
    port_forwards=[
      8200
    ],
    pod_readiness='ignore',
    flags=[
      '--values=./vault/values.yaml',
      '--create-namespace'
    ],
    resource_deps=['hashicorp'],
)

# Ensure the Vault service is forwarded so it can be accessed locally
# k8s_resource('vault:statefulset:vault:apps:1', port_forwards=8200)

# # Wait for the Vault pod to be created and in Running state (loop until found)
local_resource(
    'wait-for-vault-pod',
    cmd="""
    sleep 10
    for i in {1..30}; do
      POD_STATUS=$(kubectl get pod vault-0 --namespace=vault -o jsonpath='{.status.phase}')
      if [ "$POD_STATUS" == "Running" ]; then
        echo "vault-0 pod is running (even if not fully ready)"
        exit 0
      else
        echo "vault-0 pod not running yet (status: $POD_STATUS), retrying in 10 seconds..."
        sleep 10
      fi
    done
    echo "vault-0 pod not running after retries, exiting."
    exit 1
    """,
    resource_deps = ['vault']
)

# Create a local resource that automates initialization of Vault
# and stores the unseal key in a Kubernetes secret
local_resource(
    'vault-init',
    cmd='kubectl exec -ti vault-0 --namespace=vault -- vault operator init -key-shares=1 -key-threshold=1 | tee /tmp/vault-init-output.txt | awk \'/Unseal Key 1/ {print $4}\' | xargs -I {} kubectl create secret generic vault-unseal-secret --namespace=vault --from-literal=unseal-key={}',
    resource_deps=['wait-for-vault-pod'],
)

# Wait for the unseal secret to be created before unsealing
local_resource(
    'wait-for-unseal-secret',
    cmd="""
    while ! kubectl get secret vault-unseal-secret --namespace=vault --no-headers; do
      echo "Waiting for unseal secret to be created..."
      sleep 5
    done
    """,
    resource_deps=['vault-init'],
)

# Automate unsealing process after initialization and secret creation
local_resource(
    'vault-unseal',
    cmd='kubectl exec -ti vault-0 --namespace=vault -- vault operator unseal $(kubectl get secret vault-unseal-secret --namespace=vault -o jsonpath="{.data.unseal-key}" | base64 --decode)',
    resource_deps=['wait-for-unseal-secret'],
)
