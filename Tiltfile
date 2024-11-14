v1alpha1.extension_repo(
    name="local", url="file://" + os.path.dirname(__file__) + "/src/tilt/extensions"
)
v1alpha1.extension(name="vault", repo_name="local", repo_path="vault")
v1alpha1.extension(name="oam", repo_name="local", repo_path="oam")
v1alpha1.extension(name="kafka", repo_name="local", repo_path="kafka")
v1alpha1.extension(name="cloudnative-pg", repo_name="local", repo_path="cloudnative-pg")
load("ext://vault", "vault_deploy")
load("ext://oam", "oam_deploy")
load("ext://kafka", "kafka_deploy")
load("ext://cloudnative-pg", "cloudnative_pg_deploy")
load("ext://helm_resource", "helm_resource", "helm_repo")
load("ext://namespace", "namespace_yaml")

update_settings(  # type: ignore
    max_parallel_updates=4,
    k8s_upsert_timeout_secs=240,
    suppress_unused_image_warnings=None,
)

vault_deploy(
    values_file="./src/helm/vault/values.yaml",
    secrets={
        "cubbyhole/myapp": {
            "foo": "bar",
            "bar": "baz",
        }
    },
)

oam_deploy(values_file="./src/helm/oam/values.yaml")

kafka_deploy(values_file="./src/helm/kafka/values.yaml")

cloudnative_pg_deploy(values_file="./src/helm/postgres/values.yaml")

k8s_yaml(namespace_yaml("foo"))
helm_resource(
    "oam-local",
    "src/helm/oam-local",
    pod_readiness="ignore",
    namespace="foo",
    labels="oam-local",
    resource_deps=["vela-core", "cloudnativepg"],
)

helm_resource(
    "silly",
    "src/helm/silly",
    namespace="foo",
    labels="silly",
    resource_deps=["oam-local"],
)
