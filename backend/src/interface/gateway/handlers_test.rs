use axum::http::{HeaderMap, HeaderValue};
use secrecy::SecretString;

use crate::constants::AUTHORIZATION_HEADER;
use crate::features::{
    gateway::GatewayConfiguration,
    providers::{ProviderId, ProviderRegistry},
};
use crate::interface::gateway::{routes::GatewayHttpState, runtime::GatewayPublisher};

const LOCAL_TOKEN: &str = "local-token";

#[test]
fn accepts_the_in_memory_bearer_token() {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION_HEADER,
        HeaderValue::from_static("Bearer local-token"),
    );

    assert!(gateway_state().is_gateway_request(&headers));
}

#[test]
fn rejects_the_control_token_on_a_data_plane_route() {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION_HEADER,
        HeaderValue::from_static("Bearer control-token"),
    );

    assert!(!gateway_state().is_gateway_request(&headers));
}

fn gateway_state() -> GatewayHttpState {
    GatewayHttpState {
        registry: ProviderRegistry::new(Vec::new()),
        publisher: GatewayPublisher::new(
            GatewayConfiguration {
                provider_id: ProviderId::new("test"),
                catalog: Vec::new(),
                custom_models: Vec::new(),
            },
            "session".to_owned(),
        ),
        local_token: SecretString::from(LOCAL_TOKEN.to_owned()),
        control_token: SecretString::from("control-token".to_owned()),
    }
}
