use std::{collections::HashMap, io, sync::Arc};

use futures_util::TryStreamExt;
use reqwest::{header::HeaderName, Client, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

use super::catalog::UpstreamModelList;
use super::protocol::{endpoint_for_protocol, protocol_candidates, validation_payload};
use crate::constants::{API_KEY_HEADER, AUTHORIZATION_HEADER, MODEL_FIELD};
use crate::features::providers::{
    ModelCatalogEntry, ProviderError, ProviderHeader, ProviderRequest, ProviderResponse,
};
use crate::features::translation::{
    translate_request, translate_response_stream, UpstreamProtocol,
};

const MODELS_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/models";
const USER_AGENT_HEADER: &str = "user-agent";
const USER_AGENT_VALUE: &str = concat!("open-claude-code/", env!("CARGO_PKG_VERSION"));
const SESSION_HEADER: &str = "x-opencode-session";
const CONTENT_TYPE_HEADER: &str = "content-type";
const SSE_CONTENT_TYPE: &[u8] = b"text/event-stream";
pub(super) const CONTENT_LENGTH_HEADER: &str = "content-length";

/// Calls the public OpenCode Go API.
#[derive(Clone, Debug)]
pub(super) struct OpenCodeGoClient {
    control_client: Client,
    streaming_client: Client,
    protocol_cache: Arc<RwLock<HashMap<String, UpstreamProtocol>>>,
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
            protocol_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Probes a discovered model through each protocol to prove the credential is accepted.
    pub(super) async fn validate_api_key(
        &self,
        api_key: &SecretString,
    ) -> Result<(), ProviderError> {
        let catalog = self.fetch_catalog(api_key).await?;
        let Some(model) = catalog.first() else {
            return Err(ProviderError::UnexpectedResponse);
        };
        for protocol in protocol_candidates() {
            let payload = validation_payload(&model.id)
                .map_err(|_source| ProviderError::UnexpectedResponse)?;
            let body = translate_request(protocol, &payload)
                .map_err(|_source| ProviderError::UnexpectedResponse)?;
            let response = self
                .control_client
                .post(endpoint_for_protocol(protocol))
                .header(API_KEY_HEADER, api_key.expose_secret())
                .header(SESSION_HEADER, uuid::Uuid::new_v4().to_string())
                .body(body)
                .send()
                .await
                .map_err(ProviderError::Request)?;
            match response.status() {
                status if status.is_success() => return Ok(()),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    return Err(ProviderError::InvalidApiKey);
                }
                StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {}
                _status => return Err(ProviderError::UnexpectedResponse),
            }
        }
        Err(ProviderError::UnexpectedResponse)
    }

    /// Fetches every model exposed by authenticated upstream discovery.
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
        Ok(catalog.into_catalog())
    }

    /// Forwards a request and preserves the upstream streaming body.
    pub(super) async fn forward_messages(
        &self,
        api_key: &SecretString,
        request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        let model_id = request_model_id(&request.body)?;
        if let Some(protocol) = self.protocol_cache.read().await.get(&model_id).copied() {
            return self.send_request(protocol, api_key, request).await;
        }
        for protocol in protocol_candidates() {
            let response = self
                .send_request(protocol, api_key, request.clone())
                .await?;
            if (200..300).contains(&response.status_code) {
                self.protocol_cache.write().await.insert(model_id, protocol);
                return Ok(response);
            }
            if response.status_code != 400 && response.status_code != 404 {
                return Ok(response);
            }
        }
        Err(ProviderError::UnexpectedResponse)
    }

    async fn send_request(
        &self,
        protocol: UpstreamProtocol,
        api_key: &SecretString,
        request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        let model_id = request_model_id(&request.body)?;
        let body = translate_request(protocol, &request.body)
            .map_err(|_source| ProviderError::UnexpectedResponse)?;
        let mut upstream_request = self
            .streaming_client
            .post(endpoint_for_protocol(protocol))
            .header(API_KEY_HEADER, api_key.expose_secret())
            .header(USER_AGENT_HEADER, USER_AGENT_VALUE)
            .header(SESSION_HEADER, request.session_id)
            .body(body);
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
        map_response(response, protocol, model_id)
    }
}

fn request_model_id(body: &[u8]) -> Result<String, ProviderError> {
    let model_id = serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .and_then(|payload| {
            payload
                .get(MODEL_FIELD)
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .ok_or(ProviderError::UnexpectedResponse)?;
    Ok(model_id)
}

fn map_response(
    response: reqwest::Response,
    protocol: UpstreamProtocol,
    model_id: String,
) -> Result<ProviderResponse, ProviderError> {
    let status_code = response.status().as_u16();
    let should_translate = response.status().is_success() && protocol != UpstreamProtocol::Messages;
    let mut headers: Vec<_> = response
        .headers()
        .iter()
        .filter(|(name, _value)| should_return_header(name, protocol))
        .map(|(name, value)| ProviderHeader {
            name: name.as_str().to_owned(),
            value: value.as_bytes().to_vec(),
        })
        .collect();
    if should_translate {
        headers.push(ProviderHeader {
            name: CONTENT_TYPE_HEADER.to_owned(),
            value: SSE_CONTENT_TYPE.to_vec(),
        });
    }
    let body = response.bytes_stream().map_err(io::Error::other);
    let body = if should_translate {
        translate_response_stream(protocol, model_id, body)
    } else {
        Box::pin(body)
    };
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

fn should_return_header(name: &HeaderName, protocol: UpstreamProtocol) -> bool {
    name != CONTENT_LENGTH_HEADER
        && name != "transfer-encoding"
        && name != "connection"
        && (protocol == UpstreamProtocol::Messages || name != CONTENT_TYPE_HEADER)
}

#[cfg(test)]
#[path = "client_test.rs"]
mod client_test;
