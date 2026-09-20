use secrecy::ExposeSecret;

use super::{start_gateway, GatewayConfiguration, GatewayPublisher};
use crate::features::providers::{ProviderId, ProviderRegistry};

#[tokio::test]
async fn binds_loopback_and_stops_owned_server_task() {
    let publisher = GatewayPublisher::new(
        GatewayConfiguration {
            provider_id: ProviderId::new("test"),
            catalog: Vec::new(),
            custom_models: Vec::new(),
        },
        "test-session".to_owned(),
    );
    let runtime = start_gateway(ProviderRegistry::new(Vec::new()), publisher)
        .await
        .expect("gateway should bind loopback");

    assert!(runtime.base_url().starts_with("http://127.0.0.1:"));
    assert_ne!(
        runtime.local_token().expose_secret(),
        runtime.control_token().expose_secret()
    );
    runtime.stop().await.expect("gateway should stop");
}
