//! Reads the state the gateway publishes for the dashboard.

use open_claude_code_backend::ProviderConnectionState;

use super::super::SessionError;
use crate::features::{
    gateway::{GatewayChildError, GatewayHost},
    settings::{current_epoch_seconds, AppSettings},
};

/// Adopts the catalog the gateway discovered with the stored credential.
///
/// The gateway process validates that credential as it starts, so the app reads
/// the result back before publishing its own settings over it. An empty catalog
/// means the credential unlocked nothing, and leaves the stored one alone.
pub(super) async fn adopt_published_catalog(
    settings: &mut AppSettings,
    gateway: &GatewayHost,
) -> Result<(), SessionError> {
    let catalog = gateway.client()?.state().await?.catalog;
    if catalog.is_empty() {
        return Ok(());
    }
    settings.cached_models = catalog;
    settings.catalog_refreshed_at_epoch_seconds = Some(current_epoch_seconds());
    Ok(())
}

/// Returns the connection state the dashboard shows.
///
/// The gateway holds the provider now, so a control call that cannot be made is
/// the connection failure the dashboard reports.
pub(super) async fn published_connection(gateway: &GatewayHost) -> ProviderConnectionState {
    let Ok(client) = gateway.client() else {
        return unavailable_connection();
    };
    match client.state().await {
        Ok(state) => state.connection,
        Err(source) => ProviderConnectionState::Failed {
            message: source.to_string(),
        },
    }
}

/// Names a gateway the app cannot reach at all.
fn unavailable_connection() -> ProviderConnectionState {
    ProviderConnectionState::Failed {
        message: GatewayChildError::NotRunning.to_string(),
    }
}

#[cfg(test)]
#[path = "connection_test.rs"]
mod connection_test;
