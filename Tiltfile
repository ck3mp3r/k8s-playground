v1alpha1.extension_repo(
    name="custom", url="https://github.com/ck3mp3r/tilt-extensions", load_host="custom"
)

load("ext://custom/vault", "vault_deploy")
load("ext://custom/kubevela", "kubevela_deploy")
load("ext://custom/kafka", "kafka_deploy")
load("ext://custom/cloudnative_pg", "cloudnative_pg_deploy")
load("ext://helm_resource", "helm_resource", "helm_repo")
load("ext://namespace", "namespace_yaml")

update_settings(
    max_parallel_updates=4,
    k8s_upsert_timeout_secs=240,
    suppress_unused_image_warnings=None,
)

vault_deploy(
    values_file="./helm/vault/values.yaml",
    secrets={
        "cubbyhole/myapp": {
            "foo": "bar",
            "bar": "baz",
        }
    },
)

kubevela_deploy()
cloudnative_pg_deploy()
kafka_deploy(values_file="./helm/kafka/values.yaml")

k8s_yaml(namespace_yaml("foo"))
helm_resource(
    "kubevela-local",
    "helm/kubevela-local",
    pod_readiness="ignore",
    namespace="foo",
    labels="kubevela-local",
    resource_deps=["vela-core", "cloudnativepg"],
)

helm_resource(
    "silly",
    "helm/silly",
    namespace="foo",
    labels="silly",
    resource_deps=["kubevela-local"],
)
