use serde::{Deserialize, Serialize};

use crate::features::providers::{ModelCatalogEntry, ProviderConnectionState};

/// Carries the connection state and published models the desktop app reads.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlState {
    /// Whether the provider credential can be used.
    pub connection: ProviderConnectionState,
    /// Models the gateway publishes to Claude Code.
    pub catalog: Vec<ModelCatalogEntry>,
}

/// Carries an API key into and out of the credential route.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CredentialPayload {
    pub(super) api_key: String,
}

/// Carries the models and session the gateway serves next.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GatewayPayload {
    pub(super) catalog: Vec<ModelCatalogEntry>,
    pub(super) custom_models: Vec<String>,
    pub(super) session_id: String,
}

/// Carries a ready-to-show failure message back to the desktop app.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlErrorBody {
    /// Message the dashboard shows without further mapping.
    pub error: String,
}
