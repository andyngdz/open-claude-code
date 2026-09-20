use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use secrecy::ExposeSecret;

use super::{
    auth::is_control_authorized,
    dto::{ControlErrorBody, ControlState, CredentialPayload, GatewayPayload},
    routes::GatewayHttpState,
};
use crate::{
    errors::OpenCodeGoBackendError,
    features::providers::{Provider, ProviderError},
};

/// Message returned when a caller does not hold the control token.
const CONTROL_TOKEN_ERROR: &str = "Control token is invalid";

/// Returns the provider connection state and the published catalog.
pub(super) async fn state(State(state): State<GatewayHttpState>, headers: HeaderMap) -> Response {
    let provider = match controlled_provider(&state, &headers).await {
        Ok(provider) => provider,
        Err(failure) => return failure.into_response(),
    };
    let catalog = state.publisher.configuration().await.catalog;
    let connection = provider.connection_state().await;
    Json(ControlState {
        connection,
        catalog,
    })
    .into_response()
}

/// Returns the stored API key so the desktop settings form can show it.
pub(super) async fn credential(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
) -> Response {
    let provider = match controlled_provider(&state, &headers).await {
        Ok(provider) => provider,
        Err(failure) => return failure.into_response(),
    };
    let payload = match provider.load_saved_api_key().await {
        Ok(api_key) => CredentialPayload {
            api_key: api_key.expose_secret().to_owned(),
        },
        Err(ProviderError::CredentialNotFound) => CredentialPayload {
            api_key: String::new(),
        },
        Err(source) => return provider_failure(source),
    };
    Json(payload).into_response()
}

/// Validates and stores an API key, then returns the refreshed catalog.
pub(super) async fn save_credential(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
    Json(payload): Json<CredentialPayload>,
) -> Response {
    let provider = match controlled_provider(&state, &headers).await {
        Ok(provider) => provider,
        Err(failure) => return failure.into_response(),
    };
    match provider.save_and_test_api_key(&payload.api_key).await {
        Ok(catalog) => Json(catalog).into_response(),
        Err(source) => provider_failure(source),
    }
}

/// Removes the stored provider credential.
pub(super) async fn remove_credential(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
) -> Response {
    let provider = match controlled_provider(&state, &headers).await {
        Ok(provider) => provider,
        Err(failure) => return failure.into_response(),
    };
    match provider.remove_credential().await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(source) => provider_failure(source),
    }
}

/// Refreshes the provider model catalog with the stored credential.
pub(super) async fn refresh_catalog(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
) -> Response {
    let provider = match controlled_provider(&state, &headers).await {
        Ok(provider) => provider,
        Err(failure) => return failure.into_response(),
    };
    match provider.refresh_catalog().await {
        Ok(catalog) => Json(catalog).into_response(),
        Err(source) => provider_failure(source),
    }
}

/// Publishes the models and fallback session the gateway serves next.
pub(super) async fn publish_gateway(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
    Json(payload): Json<GatewayPayload>,
) -> Response {
    if !is_control_authorized(&headers, &state.control_token) {
        return unauthorized();
    }
    state
        .publisher
        .configure(payload.catalog, payload.custom_models, payload.session_id)
        .await;
    StatusCode::NO_CONTENT.into_response()
}

/// Why a control route could not reach the provider it acts on.
///
/// Kept apart from the response itself, which is far larger than the success
/// value and would otherwise sit in every `Result` this route tree returns.
enum ControlFailure {
    /// The caller did not carry the control token.
    Unauthorized,
    /// The provider refused the operation.
    Provider(ProviderError),
}

impl IntoResponse for ControlFailure {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => unauthorized(),
            Self::Provider(source) => provider_failure(source),
        }
    }
}

/// Returns the provider every control route acts on, or why the route cannot.
async fn controlled_provider(
    state: &GatewayHttpState,
    headers: &HeaderMap,
) -> Result<Arc<dyn Provider>, ControlFailure> {
    require_control_token(headers, state)?;
    let provider_id = state.publisher.provider_id().await;
    let lookup = state.registry.get(&provider_id);
    lookup.map_err(ControlFailure::Provider)
}

/// Rejects a request that did not carry the control token.
fn require_control_token(
    headers: &HeaderMap,
    state: &GatewayHttpState,
) -> Result<(), ControlFailure> {
    if is_control_authorized(headers, &state.control_token) {
        return Ok(());
    }
    Err(ControlFailure::Unauthorized)
}

/// Returns the response for a request that did not carry the control token.
fn unauthorized() -> Response {
    (StatusCode::UNAUTHORIZED, CONTROL_TOKEN_ERROR).into_response()
}

/// Returns the response for a provider failure, with the message the dashboard shows.
fn provider_failure(source: ProviderError) -> Response {
    let error = OpenCodeGoBackendError::from(source).to_string();
    (StatusCode::BAD_REQUEST, Json(ControlErrorBody { error })).into_response()
}

#[cfg(test)]
#[path = "control_test.rs"]
mod control_test;
