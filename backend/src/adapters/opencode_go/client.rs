use std::io;

use futures_util::TryStreamExt;
use reqwest::{header::HeaderName, Client, StatusCode};
use secrecy::{ExposeSecret, SecretString};

use super::catalog::{UpstreamModelList, MODEL_QWEN_38_FLASH};
use crate::constants::{API_KEY_HEADER, AUTHORIZATION_HEADER, MODEL_FIELD};
use crate::features::providers::{
    ModelCatalogEntry, ProviderError, ProviderHeader, ProviderRequest, ProviderResponse,
};

const MODELS_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/models";
const MESSAGES_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/messages";
const USER_AGENT_HEADER: &str = "user-agent";
const USER_AGENT_VALUE: &str = concat!("open-claude-code/", env!("CARGO_PKG_VERSION"));
const SESSION_HEADER: &str = "x-opencode-session";
pub(super) const CONTENT_LENGTH_HEADER: &str = "content-length";

fn probe_payload() -> serde_json::Value {
    let mut payload = serde_json::Map::new();
    payload.insert(
        MODEL_FIELD.to_owned(),
        serde_json::Value::String(MODEL_QWEN_38_FLASH.to_owned()),
    );
    payload.insert("max_tokens".to_owned(), serde_json::json!(1));
    payload.insert(
        "messages".to_owned(),
        serde_json::json!([{ "role": "user", "content": "." }]),
    );
    serde_json::Value::Object(payload)
}

/// Calls the public OpenCode Go API.
#[derive(Clone, Debug)]
pub(super) struct OpenCodeGoClient {
    control_client: Client,
    streaming_client: Client,
}

impl OpenCodeGoClient {
    /// Creates an HTTP client with bounded connection and request timeouts.
    pub(super) fn new() -> Result<Self, ProviderError> {
        let control_client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ProviderError::Request)?;
        let streaming_client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .read_timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(ProviderError::Request)?;
        Ok(Self {
            control_client,
            streaming_client,
        })
    }

    /// Makes a one-token request to prove the credential is accepted.
    pub(super) async fn validate_api_key(
        &self,
        api_key: &SecretString,
    ) -> Result<(), ProviderError> {
        let response = self
            .control_client
            .post(MESSAGES_ENDPOINT)
            .header(API_KEY_HEADER, api_key.expose_secret())
            .header("anthropic-version", "2023-06-01")
            .header(SESSION_HEADER, uuid::Uuid::new_v4().to_string())
            .json(&probe_payload())
            .send()
            .await
            .map_err(ProviderError::Request)?;
        match response.status() {
            status if status.is_success() => Ok(()),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(ProviderError::InvalidApiKey),
            _status => Err(ProviderError::UnexpectedResponse),
        }
    }

    /// Fetches the catalog and keeps models served by the Messages endpoint.
    pub(super) async fn fetch_catalog(
        &self,
        api_key: &SecretString,
    ) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        let response = self
            .control_client
            .get(MODELS_ENDPOINT)
            .header(API_KEY_HEADER, api_key.expose_secret())
            .send()
            .await
            .map_err(ProviderError::Request)?;
        if matches!(
            response.status(),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
        ) {
            return Err(ProviderError::InvalidApiKey);
        }
        let catalog = response
            .error_for_status()
            .map_err(ProviderError::Request)?
            .json::<UpstreamModelList>()
            .await
            .map_err(ProviderError::Request)?;
        Ok(catalog.into_messages_catalog())
    }

    /// Forwards a request and preserves the upstream streaming body.
    pub(super) async fn forward_messages(
        &self,
        api_key: &SecretString,
        request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        let mut upstream_request = self
            .streaming_client
            .post(MESSAGES_ENDPOINT)
            .header(API_KEY_HEADER, api_key.expose_secret())
            .header(USER_AGENT_HEADER, USER_AGENT_VALUE)
            .header(SESSION_HEADER, request.session_id)
            .body(request.body);
        for header in request.headers {
            let name = HeaderName::from_bytes(header.name.as_bytes())
                .map_err(|_source| ProviderError::InvalidHeader)?;
            if should_forward_header(&name) {
                upstream_request = upstream_request.header(name, header.value);
            }
        }
        let response = upstream_request
            .send()
            .await
            .map_err(ProviderError::Request)?;
        map_response(response)
    }
}

fn map_response(response: reqwest::Response) -> Result<ProviderResponse, ProviderError> {
    let status_code = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .filter(|(name, _value)| should_return_header(name))
        .map(|(name, value)| ProviderHeader {
            name: name.as_str().to_owned(),
            value: value.as_bytes().to_vec(),
        })
        .collect();
    let body = response.bytes_stream().map_err(io::Error::other);
    Ok(ProviderResponse {
        status_code,
        headers,
        body: Box::pin(body),
    })
}

fn should_forward_header(name: &HeaderName) -> bool {
    name != "host"
        && name != AUTHORIZATION_HEADER
        && name != API_KEY_HEADER
        && name != CONTENT_LENGTH_HEADER
        && name != USER_AGENT_HEADER
}

fn should_return_header(name: &HeaderName) -> bool {
    name != CONTENT_LENGTH_HEADER && name != "transfer-encoding" && name != "connection"
}

#[cfg(test)]
#[path = "client_test.rs"]
mod client_test;
