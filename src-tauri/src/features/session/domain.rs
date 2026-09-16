use std::path::PathBuf;

use open_claude_code_backend::{ModelCatalogEntry, ProviderConnectionState};
use serde::Serialize;

use crate::features::{
    launcher::{TerminalKind, TerminalOption},
    settings::ModelAliasMapping,
};

/// Carries the non-secret dashboard state returned to the frontend.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DashboardSnapshot {
    /// Credential state without exposing the API key.
    pub(crate) connection: ProviderConnectionState,
    /// Models available for aliases and launch.
    pub(crate) models: Vec<ModelCatalogEntry>,
    /// User-supplied model IDs that are not in the provider catalog.
    pub(crate) custom_models: Vec<String>,
    /// Claude Code family names mapped to provider models.
    pub(crate) aliases: ModelAliasMapping,
    /// Terminal used for the next launch.
    pub(crate) terminal: TerminalKind,
    /// Terminals this machine can open.
    pub(crate) terminals: Vec<TerminalOption>,
    /// Workspace selected for the previous launch.
    pub(crate) last_workspace: Option<PathBuf>,
    /// Unix time of the last successful catalog refresh.
    pub(crate) catalog_refreshed_at_epoch_seconds: Option<u64>,
}
