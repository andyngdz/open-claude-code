use axum::{
    body::{to_bytes, Body},
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use secrecy::ExposeSecret;

use super::{response::ResponsePresenter, routes::GatewayHttpState};
use crate::constants::{API_KEY_HEADER, AUTHORIZATION_HEADER};
use crate::features::{
    gateway::{discovery_models, resolve_provider_request},
    providers::{ProviderHeader, ProviderRequest},
};

const MAX_REQUEST_BYTES: usize = 32 * 1024 * 1024;
const CLAUDE_SESSION_HEADER: &str = "x-claude-code-session-id";
const LOCAL_TOKEN_ERROR: &str = "Local gateway token is invalid";

/// Answers Claude Code's lightweight gateway health probe.
pub(super) async fn hello() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// Publishes models configured for Claude Code discovery.
pub(super) async fn list_models(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
) -> Response {
    if !is_authorized(&headers, &state) {
        return ResponsePresenter::authentication_error(LOCAL_TOKEN_ERROR);
    }
    let configuration = state.configuration.read().await;
    ResponsePresenter::models(discovery_models(&configuration))
}

/// Resolves and forwards one Anthropic Messages request.
pub(super) async fn messages(
    State(state): State<GatewayHttpState>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    if !is_authorized(&headers, &state) {
        return ResponsePresenter::authentication_error(LOCAL_TOKEN_ERROR);
    }
    let body = match to_bytes(body, MAX_REQUEST_BYTES).await {
        Ok(bytes) => bytes,
        Err(_source) => return ResponsePresenter::bad_request("Request body could not be read"),
    };
    let configuration = state.configuration.read().await;
    let resolved = match resolve_provider_request(&configuration, &body) {
        Ok(request) => request,
        Err(source) => return ResponsePresenter::bad_request(&source.to_string()),
    };
    drop(configuration);
    let provider = match state.registry.get(&resolved.provider_id) {
        Ok(provider) => provider,
        Err(source) => return ResponsePresenter::provider_error(source),
    };
    let provider_request = ProviderRequest {
        headers: GatewayRequestAdapter::provider_headers(&headers),
        body: resolved.body,
        session_id: GatewayRequestAdapter::session_id(&headers, &state).await,
    };
    match provider.forward_messages(provider_request).await {
        Ok(response) => ResponsePresenter::upstream(response),
        Err(source) => ResponsePresenter::provider_error(source),
    }
}

fn is_authorized(headers: &HeaderMap, state: &GatewayHttpState) -> bool {
    let expected = state.local_token.expose_secret();
    credential_matches(headers, AUTHORIZATION_HEADER, Some("Bearer "), expected)
        || credential_matches(headers, API_KEY_HEADER, None, expected)
}

fn credential_matches(
    headers: &HeaderMap,
    name: &str,
    prefix: Option<&str>,
    expected: &str,
) -> bool {
    let raw_value = headers.get(name).and_then(|value| value.to_str().ok());
    let credential = prefix
        .and_then(|prefix| raw_value?.strip_prefix(prefix))
        .or(raw_value);
    credential.is_some_and(|value| value == expected)
}

struct GatewayRequestAdapter;

impl GatewayRequestAdapter {
    async fn session_id(headers: &HeaderMap, state: &GatewayHttpState) -> String {
        let provided = headers
            .get(CLAUDE_SESSION_HEADER)
            .and_then(|value| value.to_str().ok());
        match provided {
            Some(session_id) => session_id.to_owned(),
            None => state.session_id.read().await.clone(),
        }
    }

    fn provider_headers(headers: &HeaderMap) -> Vec<ProviderHeader> {
        headers
            .iter()
            .map(|(name, value)| ProviderHeader {
                name: name.as_str().to_owned(),
                value: value.as_bytes().to_vec(),
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "handlers_test.rs"]
mod handlers_test;
