use std::{fmt, io, pin::Pin};

use bytes::Bytes;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

/// Identifies a provider across settings, model IDs, and the runtime registry.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ProviderId(String);

impl ProviderId {
    /// Creates a provider identifier from a stable lowercase slug.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the provider identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Declares an authentication flow supported by a provider.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    /// A secret pasted by the user and stored in the system keyring.
    ApiKey,
    /// A browser-based OAuth authorization flow.
    OAuth,
}

/// Declares an upstream protocol implemented by a provider adapter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderProtocol {
    /// Anthropic's Messages API.
    AnthropicMessages,
    /// OpenAI's Responses API.
    OpenAiResponses,
    /// OpenAI-compatible Chat Completions.
    OpenAiChatCompletions,
}

/// Describes a provider without exposing its implementation or credentials.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDescriptor {
    /// Stable registry identifier.
    pub id: ProviderId,
    /// Name shown in the dashboard.
    pub display_name: String,
    /// Authentication flows implemented by this adapter.
    pub auth_methods: Vec<AuthMethod>,
    /// Upstream protocols implemented by this adapter.
    pub protocols: Vec<ProviderProtocol>,
}

/// Describes one provider model available to the local gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelCatalogEntry {
    /// Provider-facing model ID.
    pub id: String,
    /// Human-readable model name.
    pub display_name: String,
    /// Marks a user-supplied model that has not been protocol-verified.
    pub is_custom: bool,
}

/// Represents whether a provider credential can be used.
///
/// The control API carries this state across the process boundary, so it reads
/// back as well as it writes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ProviderConnectionState {
    /// No credential exists in the system keyring.
    Disconnected,
    /// A credential exists and passed validation when it was saved.
    Connected,
    /// The credential store could not be queried.
    Failed { message: String },
}

/// Carries one HTTP header without coupling providers to an HTTP framework.
#[derive(Clone, Debug)]
pub struct ProviderHeader {
    /// Header name encoded as ASCII.
    pub name: String,
    /// Raw header value bytes.
    pub value: Vec<u8>,
}

/// Carries a normalized gateway request into a provider adapter.
#[derive(Clone, Debug)]
pub struct ProviderRequest {
    /// Forwardable inbound headers.
    pub headers: Vec<ProviderHeader>,
    /// JSON request body with the upstream model ID already resolved.
    pub body: Vec<u8>,
    /// Stable session identifier required by subscription providers.
    pub session_id: String,
}

/// Streams provider bytes without buffering the full response.
pub type ProviderBodyStream =
    Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + 'static>>;

/// Carries an upstream provider response back to the local HTTP adapter.
pub struct ProviderResponse {
    /// Upstream HTTP status code.
    pub status_code: u16,
    /// Forwardable upstream response headers.
    pub headers: Vec<ProviderHeader>,
    /// Streaming response body.
    pub body: ProviderBodyStream,
}

#[cfg(test)]
#[path = "domain_test.rs"]
mod domain_test;
