use kube::Client;

/// Shared state structure
#[derive(Clone)]
pub struct SharedState {
    pub client: Client,
}

impl SharedState {
    pub fn new(client: Client) -> Self {
        SharedState { client }
    }
}
