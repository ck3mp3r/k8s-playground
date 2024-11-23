use kube::CustomResourceExt;
use operator::subscriber::crd::Subscriber;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Create the crds/ directory if it doesn't exist
    fs::create_dir_all("../crds")?;

    // Generate and write the Subscriber CRD
    fs::write(
        "../crds/subscriber.yaml",
        serde_yaml::to_string(&Subscriber::crd())?,
    )?;

    println!("CRDs generated successfully in the 'crds/' directory.");
    Ok(())
}
