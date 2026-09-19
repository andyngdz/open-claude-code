use std::collections::HashSet;

use open_claude_code_backend::{ModelCatalogEntry, OpenCodeGoBackend, ProviderConnectionState};
use secrecy::ExposeSecret;

use super::super::{DashboardSnapshot, SessionError};
use crate::features::{
    launcher::{
        list_available_terminals, new_launch_session_id, LaunchClaudeInput, TerminalKind,
        TerminalOption,
    },
    runtime_endpoint::{RuntimeEndpoint, RuntimeModel},
    settings::{current_epoch_seconds, AppSettings, ModelAliasMapping},
};

use super::AppSessionState;

/// Stores a refreshed catalog and republishes the CLI handshake.
pub(super) async fn persist_catalog(
    inner: &mut AppSessionState,
    catalog: Vec<ModelCatalogEntry>,
) -> Result<(), SessionError> {
    inner.settings.cached_models = catalog;
    inner.settings.catalog_refreshed_at_epoch_seconds = Some(current_epoch_seconds());
    save_settings(inner)?;
    configure_gateway(inner).await;
    publish_runtime(inner)?;
    Ok(())
}

/// Writes the private gateway handshake for the CLI.
pub(super) fn publish_runtime(inner: &AppSessionState) -> Result<(), SessionError> {
    let endpoint = RuntimeEndpoint {
        base_url: inner.backend.gateway_base_url().to_owned(),
        token: inner.backend.gateway_token().expose_secret().to_owned(),
        models: published_runtime_models(&inner.settings),
        aliases: inner.settings.aliases.clone(),
    };
    endpoint
        .write(&inner.settings_store.runtime_path())
        .map_err(|_| SessionError::Settings)
}

/// Deletes the CLI handshake when the app exits.
pub(super) fn remove_runtime(inner: &AppSessionState) -> Result<(), SessionError> {
    RuntimeEndpoint::remove(&inner.settings_store.runtime_path())
        .map_err(|_| SessionError::Settings)
}

/// Persists the current non-secret settings.
pub(super) fn save_settings(inner: &AppSessionState) -> Result<(), SessionError> {
    inner
        .settings_store
        .save(&inner.settings)
        .map_err(|_| SessionError::Settings)
}

/// Pushes the saved catalog into the local gateway.
pub(super) async fn configure_gateway(inner: &AppSessionState) {
    inner
        .backend
        .configure_gateway(
            published_catalog(&inner.settings),
            inner.settings.custom_models.clone(),
            new_launch_session_id(),
        )
        .await;
}

/// Builds the dashboard snapshot from the current session.
pub(super) async fn snapshot_from_state(inner: &AppSessionState) -> DashboardSnapshot {
    let terminals = list_available_terminals();
    let terminal = coerce_terminal_selection(inner.settings.terminal, &terminals);
    DashboardSnapshot {
        connection: inner.backend.connection_state().await,
        models: published_catalog(&inner.settings),
        custom_models: inner.settings.custom_models.clone(),
        aliases: inner.settings.aliases.clone(),
        launch_model_id: launch_model_id(&inner.settings),
        terminal,
        terminals,
        last_workspace: inner.settings.last_workspace.clone(),
        catalog_refreshed_at_epoch_seconds: inner.settings.catalog_refreshed_at_epoch_seconds,
    }
}

fn coerce_terminal_selection(selected: TerminalKind, terminals: &[TerminalOption]) -> TerminalKind {
    if terminals.iter().any(|option| option.kind == selected) {
        return selected;
    }
    TerminalKind::SystemDefault
}

/// Rejects a launch when the provider or model is not ready.
/// Rejects a launch when the provider or model is not ready.
pub(super) async fn ensure_can_launch(
    inner: &AppSessionState,
    input: &LaunchClaudeInput,
) -> Result<(), SessionError> {
    let connection = inner.backend.connection_state().await;
    if !matches!(connection, ProviderConnectionState::Connected) {
        return Err(SessionError::NotConnected);
    }
    ensure_known_model(
        &inner.settings,
        &inner.settings.custom_models,
        &input.model_id,
    )
}

/// Returns the cached catalog, or the fallback catalog when none is cached.
pub(super) fn published_catalog(settings: &AppSettings) -> Vec<ModelCatalogEntry> {
    if settings.cached_models.is_empty() {
        return OpenCodeGoBackend::fallback_catalog();
    }
    settings.cached_models.clone()
}

/// Publishes both discovered and custom models for the terminal CLI picker.
fn published_runtime_models(settings: &AppSettings) -> Vec<RuntimeModel> {
    runtime_models(published_catalog(settings), &settings.custom_models)
}

fn runtime_models(
    catalog_models: Vec<ModelCatalogEntry>,
    custom_model_ids: &[String],
) -> Vec<RuntimeModel> {
    let mut published_model_ids = HashSet::new();
    let mut runtime_models = catalog_models
        .into_iter()
        .map(|model| {
            published_model_ids.insert(model.id.clone());
            RuntimeModel {
                id: model.id,
                display_name: model.display_name,
            }
        })
        .collect::<Vec<_>>();
    runtime_models.extend(
        custom_model_ids
            .iter()
            .filter(|model_id| published_model_ids.insert((*model_id).clone()))
            .map(|model_id| RuntimeModel {
                id: model_id.clone(),
                display_name: model_id.clone(),
            }),
    );
    runtime_models
}

/// Keeps the upstream catalog unchanged, including a legitimate empty response.
pub(super) fn nonempty_catalog(catalog: Vec<ModelCatalogEntry>) -> Vec<ModelCatalogEntry> {
    catalog
}

/// Rejects alias mappings that point at unknown model ids.
pub(super) fn ensure_known_aliases(
    settings: &AppSettings,
    custom_models: &[String],
    aliases: &ModelAliasMapping,
) -> Result<(), SessionError> {
    let known_ids = known_ids(settings, custom_models);
    let alias_ids = [
        &aliases.fable,
        &aliases.opus,
        &aliases.sonnet,
        &aliases.haiku,
    ];
    if alias_ids
        .into_iter()
        .all(|model_id| known_ids.contains(model_id))
    {
        return Ok(());
    }
    Err(SessionError::UnknownModel)
}

/// Rejects a selected model unless it belongs to the catalog or custom list.
pub(super) fn ensure_known_model(
    settings: &AppSettings,
    custom_models: &[String],
    model_id: &str,
) -> Result<(), SessionError> {
    if known_ids(settings, custom_models).contains(model_id) {
        return Ok(());
    }
    Err(SessionError::UnknownModel)
}

fn launch_model_id(settings: &AppSettings) -> String {
    settings
        .launch_model_id
        .clone()
        .filter(|model_id| known_ids(settings, &settings.custom_models).contains(model_id))
        .unwrap_or_else(|| settings.aliases.sonnet.clone())
}

fn known_ids(settings: &AppSettings, custom_models: &[String]) -> HashSet<String> {
    let mut known_ids = settings
        .cached_models
        .iter()
        .map(|model| model.id.clone())
        .collect::<HashSet<_>>();
    if known_ids.is_empty() {
        known_ids = OpenCodeGoBackend::fallback_catalog()
            .into_iter()
            .map(|model| model.id)
            .collect();
    }
    known_ids.extend(custom_models.iter().cloned());
    known_ids
}

#[cfg(test)]
#[path = "runtime_test.rs"]
mod runtime_test;
