use std::collections::HashSet;

use open_claude_code_backend::{ModelCatalogEntry, OpenCodeGoBackend, ProviderConnectionState};
use tokio::sync::Mutex;

use super::{DashboardSnapshot, SessionError};
use crate::features::{
    launcher::{
        launch_claude, list_available_terminals, new_launch_session_id, LaunchClaudeInput,
        ProcessRegistry,
    },
    settings::{
        current_epoch_seconds, normalize_custom_models, AppSettings, ModelAliasMapping,
        SaveProviderSettingsInput, SettingsStore,
    },
};

/// Owns the running gateway, saved settings, and launched terminal processes.
pub(crate) struct AppSession {
    inner: Mutex<AppSessionState>,
}

struct AppSessionState {
    backend: OpenCodeGoBackend,
    settings: AppSettings,
    settings_store: SettingsStore,
    processes: ProcessRegistry,
}

impl AppSession {
    /// Loads settings and starts the local gateway.
    pub(crate) async fn start() -> Result<Self, SessionError> {
        let settings_store =
            SettingsStore::for_application().map_err(|_source| SessionError::Settings)?;
        let settings = settings_store
            .load()
            .map_err(|_source| SessionError::Settings)?;
        let catalog = published_catalog(&settings);
        let backend = OpenCodeGoBackend::start(catalog, settings.custom_models.clone()).await?;

        Ok(Self {
            inner: Mutex::new(AppSessionState {
                backend,
                settings,
                settings_store,
                processes: ProcessRegistry::default(),
            }),
        })
    }

    /// Returns the current dashboard without exposing credentials.
    pub(crate) async fn snapshot(&self) -> DashboardSnapshot {
        let inner = self.inner.lock().await;
        snapshot_from_state(&inner).await
    }

    /// Validates, stores, and publishes a new API key.
    pub(crate) async fn save_api_key(
        &self,
        api_key: &str,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let catalog = nonempty_catalog(inner.backend.save_api_key(api_key).await?);
        persist_catalog(&mut inner, catalog).await?;
        Ok(snapshot_from_state(&inner).await)
    }

    /// Removes the saved credential and returns to the fallback catalog.
    pub(crate) async fn remove_credential(&self) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        inner.backend.remove_credential().await?;
        inner.settings.cached_models = OpenCodeGoBackend::fallback_catalog();
        inner.settings.catalog_refreshed_at_epoch_seconds = None;
        save_settings(&inner)?;
        configure_gateway(&inner).await;
        Ok(snapshot_from_state(&inner).await)
    }

    /// Refreshes the catalog with the stored credential.
    pub(crate) async fn refresh_catalog(&self) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let catalog = nonempty_catalog(inner.backend.refresh_catalog().await?);
        persist_catalog(&mut inner, catalog).await?;
        Ok(snapshot_from_state(&inner).await)
    }

    /// Saves terminal, alias, and custom-model settings.
    pub(crate) async fn save_settings(
        &self,
        input: SaveProviderSettingsInput,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let custom_models = normalize_custom_models(input.custom_models);
        ensure_known_aliases(&inner.settings, &custom_models, &input.aliases)?;
        inner.settings.terminal = input.terminal;
        inner.settings.aliases = input.aliases;
        inner.settings.custom_models = custom_models;
        save_settings(&inner)?;
        configure_gateway(&inner).await;
        Ok(snapshot_from_state(&inner).await)
    }

    /// Opens Claude Code through the local gateway.
    pub(crate) async fn launch(
        &self,
        input: LaunchClaudeInput,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        ensure_can_launch(&inner, &input).await?;
        let session_id = new_launch_session_id();
        inner
            .backend
            .configure_gateway(
                published_catalog(&inner.settings),
                inner.settings.custom_models.clone(),
                session_id,
            )
            .await;
        let receipt = launch_claude(
            &inner.processes,
            inner.settings.terminal,
            &input.workspace,
            &input.model_id,
            inner.backend.gateway_base_url(),
            inner.backend.gateway_token(),
            &inner.settings.aliases,
        )?;
        inner.settings.last_workspace = Some(receipt.workspace);
        save_settings(&inner)?;
        Ok(snapshot_from_state(&inner).await)
    }

    /// Stops the owned local gateway.
    pub(crate) async fn stop(&self) -> Result<(), SessionError> {
        self.inner.lock().await.backend.stop().await?;
        Ok(())
    }
}

async fn persist_catalog(
    inner: &mut AppSessionState,
    catalog: Vec<ModelCatalogEntry>,
) -> Result<(), SessionError> {
    inner.settings.cached_models = catalog;
    inner.settings.catalog_refreshed_at_epoch_seconds = Some(current_epoch_seconds());
    save_settings(inner)?;
    configure_gateway(inner).await;
    Ok(())
}

fn save_settings(inner: &AppSessionState) -> Result<(), SessionError> {
    inner
        .settings_store
        .save(&inner.settings)
        .map_err(|_source| SessionError::Settings)
}

async fn configure_gateway(inner: &AppSessionState) {
    inner
        .backend
        .configure_gateway(
            published_catalog(&inner.settings),
            inner.settings.custom_models.clone(),
            new_launch_session_id(),
        )
        .await;
}

async fn snapshot_from_state(inner: &AppSessionState) -> DashboardSnapshot {
    DashboardSnapshot {
        connection: inner.backend.connection_state().await,
        models: published_catalog(&inner.settings),
        custom_models: inner.settings.custom_models.clone(),
        aliases: inner.settings.aliases.clone(),
        terminal: inner.settings.terminal,
        terminals: list_available_terminals(),
        last_workspace: inner.settings.last_workspace.clone(),
        catalog_refreshed_at_epoch_seconds: inner.settings.catalog_refreshed_at_epoch_seconds,
    }
}

async fn ensure_can_launch(
    inner: &AppSessionState,
    input: &LaunchClaudeInput,
) -> Result<(), SessionError> {
    let connection = inner.backend.connection_state().await;
    if !matches!(connection, ProviderConnectionState::Connected) {
        return Err(SessionError::NotConnected);
    }
    if is_known_model(&inner.settings, &input.model_id) {
        return Ok(());
    }
    Err(SessionError::UnknownModel)
}

fn published_catalog(settings: &AppSettings) -> Vec<ModelCatalogEntry> {
    if settings.cached_models.is_empty() {
        return OpenCodeGoBackend::fallback_catalog();
    }
    settings.cached_models.clone()
}

fn nonempty_catalog(catalog: Vec<ModelCatalogEntry>) -> Vec<ModelCatalogEntry> {
    if catalog.is_empty() {
        return OpenCodeGoBackend::fallback_catalog();
    }
    catalog
}

fn ensure_known_aliases(
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

fn is_known_model(settings: &AppSettings, model_id: &str) -> bool {
    known_ids(settings, &settings.custom_models).contains(model_id)
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
