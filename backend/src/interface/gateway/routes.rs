use axum::{
    routing::{get, head, post, put},
    Router,
};
use secrecy::SecretString;

use super::{
    auth::is_gateway_authorized,
    control,
    handlers::{hello, list_models, messages},
    runtime::GatewayPublisher,
};
use crate::{
    constants::{
        CONTROL_CATALOG_REFRESH_PATH, CONTROL_CREDENTIAL_PATH, CONTROL_GATEWAY_PATH,
        CONTROL_STATE_PATH, HEALTH_PATH,
    },
    features::providers::ProviderRegistry,
};

/// Shared dependencies used by Axum request handlers.
#[derive(Clone)]
pub(super) struct GatewayHttpState {
    pub(super) registry: ProviderRegistry,
    pub(super) publisher: GatewayPublisher,
    pub(super) local_token: SecretString,
    pub(super) control_token: SecretString,
}

impl GatewayHttpState {
    /// Returns whether the request carries the token Claude Code was launched with.
    pub(super) fn is_gateway_request(&self, headers: &axum::http::HeaderMap) -> bool {
        is_gateway_authorized(headers, &self.local_token)
    }
}

/// Builds the router that serves Claude Code and the owning desktop process.
///
/// Both surfaces share one loopback listener; the token each caller holds
/// decides which routes it can reach.
pub(super) fn gateway_router(state: GatewayHttpState) -> Router {
    Router::new()
        .route(HEALTH_PATH, head(hello))
        .route("/v1/models", get(list_models))
        .route("/v1/messages", post(messages))
        .route(CONTROL_STATE_PATH, get(control::state))
        .route(
            CONTROL_CREDENTIAL_PATH,
            get(control::credential)
                .put(control::save_credential)
                .delete(control::remove_credential),
        )
        .route(CONTROL_CATALOG_REFRESH_PATH, post(control::refresh_catalog))
        .route(CONTROL_GATEWAY_PATH, put(control::publish_gateway))
        .with_state(state)
}
