load("ext://helm_resource", "helm_resource", "helm_repo")  # type: ignore
load("ext://namespace", "namespace_yaml")  # type: ignore


def postgres_deploy(values_file):
    helm_repo("bitnami", "https://charts.bitnami.com/bitnami", "bitnami-postgres")  # type: ignore

    k8s_yaml(namespace_yaml("postgres"))  # type: ignore

    helm_resource(  # type: ignore
        "postgres",
        "bitnami/postgresql",
        namespace="postgres",
        pod_readiness="ignore",
        flags=[
            "--values=" + values_file,
        ],
        resource_deps=["bitnami-postgres"],
    )
