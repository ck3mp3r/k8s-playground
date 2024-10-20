v1alpha1.extension_repo(
    name="local", url="file://" + os.path.dirname(__file__) + "/src/tilt/extensions"
)
v1alpha1.extension(name="vault", repo_name="local", repo_path="vault")
v1alpha1.extension(name="oam", repo_name="local", repo_path="oam")
v1alpha1.extension(name="kafka", repo_name="local", repo_path="kafka")
v1alpha1.extension(name="postgres", repo_name="local", repo_path="postgres")
load("ext://vault", "vault_deploy")
load("ext://oam", "oam_deploy")
load("ext://kafka", "kafka_deploy")
load("ext://postgres", "postgres_deploy")

update_settings(  # type: ignore
    max_parallel_updates=4,
    k8s_upsert_timeout_secs=60,
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

postgres_deploy(values_file="./src/helm/postgres/values.yaml")
