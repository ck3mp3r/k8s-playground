load("ext://helm_resource", "helm_resource", "helm_repo")  # type: ignore
load("ext://namespace", "namespace_yaml")  # type: ignore


def vault_deploy(values_file, secrets={}):
    rel_path = os.path.dirname(__file__)  # type: ignore
    helm_repo("hashicorp", "https://helm.releases.hashicorp.com")  # type: ignore

    k8s_yaml(namespace_yaml("vault"))  # type: ignore

    helm_resource(  # type: ignore
        "vault",
        "hashicorp/vault",
        namespace="vault",
        pod_readiness="ignore",
        flags=[
            "--values=" + values_file,
        ],
        resource_deps=["hashicorp"],
    )

    local_resource(  # type: ignore
        "vault-init-and-unseal",
        cmd=str(read_file(rel_path + "/vault/init-unseal.sh")),  # type: ignore
        resource_deps=["vault"],
    )

    seed_cmds = ""

    for path, kv_pairs in secrets.items():
        kv_string = " ".join(
            ["{}={}".format(key, value) for key, value in kv_pairs.items()]
        )
        seed_cmds += "vault kv put " + path + " " + kv_string + "\n"

    local_resource(  # type: ignore
        "vault-seed-secrets",
        cmd=str(read_file(rel_path + "/vault/seed-secrets.sh")).replace("__SEED_CMDS__", seed_cmds),  # type: ignore
        resource_deps=["vault-init-and-unseal"],
    )

    local_resource(  # type: ignore
        "vault-port-forward",
        serve_cmd="kubectl port-forward svc/vault 8200:8200 --namespace=vault",
        resource_deps=["vault-init-and-unseal"],
    )
