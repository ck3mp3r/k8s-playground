load('ext://helm_resource', 'helm_resource', 'helm_repo')  # type: ignore
load('ext://namespace', 'namespace_yaml') # type: ignore

def oam_deploy(values_file='./oam/values.yaml'):
    helm_repo( # type: ignore
        'kubevela', 
        'https://charts.kubevela.net/core'
    )

    k8s_yaml(namespace_yaml('vela-system')) # type: ignore

    helm_resource( # type: ignore
        'vela-core', 
        'kubevela/vela-core', 
        namespace="vela-system",
        pod_readiness='ignore',
        flags=[
            '--values=' + values_file,
        ],
        resource_deps=['kubevela'],
    )
