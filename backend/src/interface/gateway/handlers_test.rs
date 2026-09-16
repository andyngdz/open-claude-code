use axum::http::{HeaderMap, HeaderValue};
use secrecy::SecretString;
use tokio::sync::RwLock;

use super::is_authorized;
use crate::constants::AUTHORIZATION_HEADER;
use crate::{
    features::{
        gateway::GatewayConfiguration,
        providers::{ProviderId, ProviderRegistry},
    },
    interface::gateway::routes::GatewayHttpState,
};

#[test]
fn accepts_the_in_memory_bearer_token() {
    let state = GatewayHttpState {
        registry: ProviderRegistry::new(Vec::new()),
        configuration: std::sync::Arc::new(RwLock::new(GatewayConfiguration {
            provider_id: ProviderId::new("test"),
            catalog: Vec::new(),
            custom_models: Vec::new(),
        })),
        session_id: std::sync::Arc::new(RwLock::new("session".to_owned())),
        local_token: SecretString::from("local-token".to_owned()),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION_HEADER,
        HeaderValue::from_static("Bearer local-token"),
    );

    assert!(is_authorized(&headers, &state));
}
