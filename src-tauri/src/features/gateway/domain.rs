use serde::{Deserialize, Serialize};

use super::GatewayChildError;

/// One line the gateway process prints for the app that spawned it.
///
/// The line carries the loopback address and both credentials: the gateway
/// token a launched Claude Code session receives, and the control token only
/// the app holds.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GatewayHandshake {
    pub(crate) base_url: String,
    pub(crate) token: String,
    pub(crate) control_token: String,
}

impl GatewayHandshake {
    /// Renders the single stdout line the app reads.
    pub(crate) fn to_line(&self) -> Result<String, GatewayChildError> {
        serde_json::to_string(self).map_err(GatewayChildError::Handshake)
    }

    /// Reads the handshake line the gateway process printed.
    pub(crate) fn parse(line: &str) -> Result<Self, GatewayChildError> {
        serde_json::from_str(line).map_err(GatewayChildError::Handshake)
    }
}

#[cfg(test)]
#[path = "domain_test.rs"]
mod domain_test;
