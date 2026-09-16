use std::sync::Arc;

use axum::{routing::get, routing::head, routing::post, Router};
use secrecy::SecretString;
use tokio::sync::RwLock;

use super::{
    handlers::{hello, list_models, messages},
    runtime::GatewayConfiguration,
};
use crate::features::providers::ProviderRegistry;

/// Shared dependencies used by Axum request handlers.
#[derive(Clone)]
pub(super) struct GatewayHttpState {
    pub(super) registry: ProviderRegistry,
    pub(super) configuration: Arc<RwLock<GatewayConfiguration>>,
    pub(super) session_id: Arc<RwLock<String>>,
    pub(super) local_token: SecretString,
}

/// Builds the Claude-compatible HTTP router.
pub(super) fn gateway_router(state: GatewayHttpState) -> Router {
    Router::new()
        .route("/api/hello", head(hello))
        .route("/v1/models", get(list_models))
        .route("/v1/messages", post(messages))
        .with_state(state)
}
