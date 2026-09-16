use std::sync::Arc;

use tokio::sync::RwLock;

use super::{start_gateway, GatewayConfiguration};
use crate::features::providers::{ProviderId, ProviderRegistry};

#[tokio::test]
async fn binds_loopback_and_stops_owned_server_task() {
    let configuration = GatewayConfiguration {
        provider_id: ProviderId::new("test"),
        catalog: Vec::new(),
        custom_models: Vec::new(),
    };
    let runtime = start_gateway(
        ProviderRegistry::new(Vec::new()),
        Arc::new(RwLock::new(configuration)),
        Arc::new(RwLock::new("test-session".to_owned())),
    )
    .await
    .expect("gateway should bind loopback");

    assert!(runtime.base_url().starts_with("http://127.0.0.1:"));
    runtime.stop().await.expect("gateway should stop");
}
