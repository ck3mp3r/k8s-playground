use k8s_openapi::chrono::Utc;
use kube::Client;
use operator::product::operator::controller as product_controller;
use operator::subscriber::operator::controller as subscription_controller;
use std::io::Write;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .format(|buf, record| {
            serde_json::to_writer(
                &mut *buf,
                &serde_json::json!({
                    "level": record.level().to_string(),
                    "target": record.target(),
                    "message": record.args(),
                    "timestamp": Utc::now().to_rfc3339(),
                }),
            )
            .unwrap();
            writeln!(buf)
        })
        .init();

    let client = Client::try_default().await?;

    let subscriber = tokio::spawn(subscription_controller(client.clone()));
    let product = tokio::spawn(product_controller(client.clone()));

    tokio::try_join!(subscriber, product)?;

    Ok(())
}
