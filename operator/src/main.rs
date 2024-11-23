mod crd;

mod operator;

use k8s_openapi::chrono;
use kube::Client;
use operator::shared_state::SharedState;
use operator::subscriber::controller;
use std::io::Write;
use std::sync::Arc;
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
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }),
            )
            .unwrap();
            writeln!(buf)
        })
        .init();

    // Initialize Kubernetes client
    let client = Client::try_default().await?;

    // Shared state for the reconciler
    let shared_state = Arc::new(SharedState::new(client.clone()));

    let controller1 = tokio::spawn(controller(shared_state.clone()));

    tokio::try_join!(controller1)?;

    Ok(())
}
