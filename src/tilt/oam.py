load('ext://helm_resource', 'helm_resource', 'helm_repo')  # type: ignore

def oam_deploy(values_file='./oam/values.yaml'):
    helm_repo( # type: ignore
        'kubevela', 
        'https://charts.kubevela.net/core'
    )

    helm_resource( # type: ignore
        'vela-core', 
        'kubevela/vela-core', 
        namespace="vela-system",
        pod_readiness='ignore',
        flags=[
            '--values=' + values_file,
            '--create-namespace',
        ],
        resource_deps=['kubevela'],
    )
