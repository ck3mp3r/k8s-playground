load('./tilt/vault_helm_deploy.py', 'vault_helm_deploy') # type: ignore

k8s_resource( # type: ignore
    vault_helm_deploy(), # type: ignore
    port_forwards=[8200],
    extra_pod_selectors=[{'app.kubernetes.io/name': 'vault'}],
)
