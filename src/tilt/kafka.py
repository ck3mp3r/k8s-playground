load("ext://helm_resource", "helm_resource", "helm_repo")  # type: ignore
load("ext://namespace", "namespace_yaml")  # type: ignore


def kafka_deploy(values_file):
    helm_repo("bitnami", "https://charts.bitnami.com/bitnami", "bitnami-kafka")  # type: ignore

    k8s_yaml(namespace_yaml("kafka"))  # type: ignore
    
    helm_resource(  # type: ignore
        "kafka",
        "bitnami/kafka",
        namespace="kafka",
        pod_readiness="ignore",
        flags=[
            "--values=" + values_file,
        ],
        resource_deps=["bitnami-kafka"],
    )
