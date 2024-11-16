v1alpha1.extension_repo(
    name="custom", url="https://github.com/ck3mp3r/tilt-extensions"
)
v1alpha1.extension(name="vault", repo_name="custom", repo_path="vault")
v1alpha1.extension(name="kubevela", repo_name="custom", repo_path="kubevela")
v1alpha1.extension(name="kafka", repo_name="custom", repo_path="kafka")
v1alpha1.extension(name="cloudnative_pg", repo_name="custom", repo_path="cloudnative_pg")

load("ext://vault", "vault_deploy")
load("ext://kubevela", "kubevela_deploy")
load("ext://kafka", "kafka_deploy")
load("ext://cloudnative_pg", "cloudnative_pg_deploy")
load("ext://helm_resource", "helm_resource", "helm_repo")
load("ext://namespace", "namespace_yaml")

update_settings(  # type: ignore
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
kafka_deploy(values_file="./helm/kafka/values.yaml")
cloudnative_pg_deploy()

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
