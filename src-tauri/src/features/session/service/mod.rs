use open_claude_code_backend::OpenCodeGoBackend;
use tokio::sync::Mutex;

use super::{DashboardSnapshot, SessionError};
use crate::features::{
    launcher::{launch_claude, new_launch_session_id, LaunchClaudeInput, ProcessRegistry},
    settings::{normalize_custom_models, AppSettings, SaveProviderSettingsInput, SettingsStore},
};

/// Owns the running gateway, saved settings, and launched terminal processes.
pub(crate) struct AppSession {
    inner: Mutex<AppSessionState>,
}

mod runtime;
pub(crate) mod window;

/// Running gateway, settings, and launched processes for one desktop session.
pub(super) struct AppSessionState {
    pub(super) backend: OpenCodeGoBackend,
    pub(super) settings: AppSettings,
    pub(super) settings_store: SettingsStore,
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
        let catalog = runtime::published_catalog(&settings);
        let backend = OpenCodeGoBackend::start(catalog, settings.custom_models.clone()).await?;
        let state = AppSessionState {
            backend,
            settings,
            settings_store,
            processes: ProcessRegistry::default(),
        };
        runtime::publish_runtime(&state)?;

        Ok(Self {
            inner: Mutex::new(state),
        })
    }

    /// Returns the current dashboard without exposing credentials.
    pub(crate) async fn snapshot(&self) -> DashboardSnapshot {
        let inner = self.inner.lock().await;
        runtime::snapshot_from_state(&inner).await
    }

    /// Validates, stores, and publishes a new API key.
    pub(crate) async fn save_api_key(
        &self,
        api_key: &str,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let catalog = runtime::nonempty_catalog(inner.backend.save_api_key(api_key).await?);
        runtime::persist_catalog(&mut inner, catalog).await?;
        Ok(runtime::snapshot_from_state(&inner).await)
    }

    /// Removes the saved credential and returns to the fallback catalog.
    pub(crate) async fn remove_credential(&self) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        inner.backend.remove_credential().await?;
        inner.settings.cached_models = OpenCodeGoBackend::fallback_catalog();
        inner.settings.catalog_refreshed_at_epoch_seconds = None;
        runtime::save_settings(&inner)?;
        runtime::configure_gateway(&inner).await;
        Ok(runtime::snapshot_from_state(&inner).await)
    }

    /// Refreshes the catalog with the stored credential.
    pub(crate) async fn refresh_catalog(&self) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let catalog = runtime::nonempty_catalog(inner.backend.refresh_catalog().await?);
        runtime::persist_catalog(&mut inner, catalog).await?;
        Ok(runtime::snapshot_from_state(&inner).await)
    }

    /// Saves terminal, alias, and custom-model settings.
    pub(crate) async fn save_settings(
        &self,
        input: SaveProviderSettingsInput,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        let custom_models = normalize_custom_models(input.custom_models);
        runtime::ensure_known_aliases(&inner.settings, &custom_models, &input.aliases)?;
        inner.settings.terminal = input.terminal;
        inner.settings.aliases = input.aliases;
        inner.settings.custom_models = custom_models;
        runtime::save_settings(&inner)?;
        runtime::configure_gateway(&inner).await;
        runtime::publish_runtime(&inner)?;
        Ok(runtime::snapshot_from_state(&inner).await)
    }

    /// Opens Claude Code through the local gateway.
    pub(crate) async fn launch(
        &self,
        input: LaunchClaudeInput,
    ) -> Result<DashboardSnapshot, SessionError> {
        let mut inner = self.inner.lock().await;
        runtime::ensure_can_launch(&inner, &input).await?;
        let session_id = new_launch_session_id();
        inner
            .backend
            .configure_gateway(
                runtime::published_catalog(&inner.settings),
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
        runtime::save_settings(&inner)?;
        runtime::publish_runtime(&inner)?;
        Ok(runtime::snapshot_from_state(&inner).await)
    }

    /// Stops the owned local gateway.
    pub(crate) async fn stop(&self) -> Result<(), SessionError> {
        let inner = self.inner.lock().await;
        if runtime::remove_runtime(&inner).is_err() {
            return Err(SessionError::Settings);
        }
        inner.backend.stop().await?;
        Ok(())
    }
}
