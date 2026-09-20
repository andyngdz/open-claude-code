use std::sync::Arc;

use secrecy::SecretString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::{
    adapters::opencode_go::{OpenCodeGoProvider, OPENCODE_GO_PROVIDER_ID},
    errors::OpenCodeGoBackendError,
    features::{
        gateway::{public_model_id, GatewayConfiguration},
        providers::{
            ModelCatalogEntry, Provider, ProviderConnectionState, ProviderId, ProviderRegistry,
        },
    },
    interface::{start_gateway, GatewayPublisher, GatewayRuntime},
};

/// Owns the OpenCode Go provider and its Claude-compatible local gateway.
pub struct OpenCodeGoBackend {
    provider: Arc<OpenCodeGoProvider>,
    gateway: GatewayRuntime,
}

impl OpenCodeGoBackend {
    /// Starts the provider and local gateway with the supplied model configuration.
    pub async fn start(
        catalog: Vec<ModelCatalogEntry>,
        custom_models: Vec<String>,
    ) -> Result<Self, OpenCodeGoBackendError> {
        let provider = Arc::new(OpenCodeGoProvider::new()?);
        let registry = ProviderRegistry::new(vec![provider.clone()]);
        let configuration = GatewayConfiguration {
            provider_id: provider.descriptor().id.clone(),
            catalog,
            custom_models,
        };
        let gateway = start_gateway(
            registry,
            GatewayPublisher::new(configuration, Uuid::new_v4().to_string()),
        )
        .await?;

        Ok(Self { provider, gateway })
    }

    /// Returns the current credential state without exposing the credential.
    pub async fn connection_state(&self) -> ProviderConnectionState {
        self.provider.connection_state().await
    }

    /// Loads the saved API key for the local desktop settings surface.
    ///
    /// Reports a provider error when no key is stored, so the caller does not
    /// mistake an absent credential for an empty one.
    pub async fn load_saved_api_key(&self) -> Result<SecretString, OpenCodeGoBackendError> {
        self.provider.load_saved_api_key().await.map_err(Into::into)
    }

    /// Validates and stores an API key, then returns the current model catalog.
    pub async fn save_api_key(
        &self,
        api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, OpenCodeGoBackendError> {
        self.provider
            .save_and_test_api_key(api_key)
            .await
            .map_err(Into::into)
    }

    /// Removes the stored OpenCode Go credential.
    pub async fn remove_credential(&self) -> Result<(), OpenCodeGoBackendError> {
        self.provider.remove_credential().await.map_err(Into::into)
    }

    /// Refreshes the provider model catalog with the stored credential.
    pub async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, OpenCodeGoBackendError> {
        self.provider.refresh_catalog().await.map_err(Into::into)
    }

    /// Returns an empty catalog before provider discovery succeeds.
    pub fn fallback_catalog() -> Vec<ModelCatalogEntry> {
        OpenCodeGoProvider::fallback_catalog()
    }

    /// Updates models and the fallback session used by subsequent gateway requests.
    pub async fn configure_gateway(
        &self,
        catalog: Vec<ModelCatalogEntry>,
        custom_models: Vec<String>,
        session_id: String,
    ) {
        self.gateway
            .configure(catalog, custom_models, session_id)
            .await;
    }

    /// Returns the loopback URL passed only to launched Claude Code processes.
    pub fn gateway_base_url(&self) -> &str {
        self.gateway.base_url()
    }

    /// Returns the in-memory gateway credential passed only to launched processes.
    pub fn gateway_token(&self) -> &SecretString {
        self.gateway.local_token()
    }

    /// Returns the credential only the process that spawned this backend holds.
    pub fn control_token(&self) -> &SecretString {
        self.gateway.control_token()
    }

    /// Stops the owned gateway task.
    pub async fn stop(&self) -> Result<(), OpenCodeGoBackendError> {
        self.gateway.stop().await.map_err(Into::into)
    }
}

#[cfg(test)]
#[path = "runtime_test.rs"]
mod runtime_test;

/// Builds the public model ID used for Claude Code model aliases and launch arguments.
pub fn open_code_go_public_model_id(model_id: &str) -> String {
    public_model_id(&ProviderId::new(OPENCODE_GO_PROVIDER_ID), model_id)
}

/// Marker that tells Claude Code to widen a model's context window to 1M tokens.
const ONE_MILLION_SUFFIX: &str = "[1m]";

/// Context window Claude Code is told a model has.
///
/// The stored and wire shape stays a plain checkbox boolean, so the state is
/// named only where the value is read.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContextWindow {
    /// Claude Code applies its own window for the model.
    #[default]
    Standard,
    /// Claude Code is told the window is 1M tokens.
    OneMillion,
}

/// Keeps a checkbox and the named window in step without a branch at the call site.
impl Serialize for ContextWindow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(matches!(self, Self::OneMillion))
    }
}

/// Reads the stored checkbox back into the named window.
impl<'de> Deserialize<'de> for ContextWindow {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let checked = bool::deserialize(deserializer)?;
        Ok(match checked {
            true => Self::OneMillion,
            false => Self::Standard,
        })
    }
}

impl ContextWindow {
    /// Picks the wider window, so a row asking for 1M is never overruled.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_claude_code_backend::ContextWindow;
    ///
    /// assert_eq!(
    ///     ContextWindow::Standard.widest(ContextWindow::OneMillion),
    ///     ContextWindow::OneMillion
    /// );
    /// ```
    pub fn widest(self, other: Self) -> Self {
        match (self, other) {
            (Self::OneMillion, Self::OneMillion) => Self::OneMillion,
            (Self::OneMillion, Self::Standard) => Self::OneMillion,
            (Self::Standard, Self::OneMillion) => Self::OneMillion,
            (Self::Standard, Self::Standard) => Self::Standard,
        }
    }
}

/// Splits the trailing 1M marker off a model id and names the window it asks for.
///
/// The marker compares ignoring ASCII case. An id that is only the marker comes
/// back whole, so stripping never leaves an empty model id for a caller to
/// launch.
///
/// # Examples
///
/// ```
/// use open_claude_code_backend::{split_one_million_suffix, ContextWindow};
///
/// assert_eq!(
///     split_one_million_suffix("qwen3.8-max[1m]"),
///     ("qwen3.8-max", ContextWindow::OneMillion)
/// );
/// assert_eq!(
///     split_one_million_suffix("qwen3.8-max"),
///     ("qwen3.8-max", ContextWindow::Standard)
/// );
/// ```
pub fn split_one_million_suffix(model_id: &str) -> (&str, ContextWindow) {
    let marker_start = model_id.len().checked_sub(ONE_MILLION_SUFFIX.len());
    let Some((bare_id, marker)) = marker_start.and_then(|start| model_id.split_at_checked(start))
    else {
        return (model_id, ContextWindow::Standard);
    };
    if !bare_id.is_empty() && marker.eq_ignore_ascii_case(ONE_MILLION_SUFFIX) {
        (bare_id, ContextWindow::OneMillion)
    } else {
        (model_id, ContextWindow::Standard)
    }
}

/// Returns the model id carrying the 1M marker exactly when `window` asks for it.
///
/// The marker is added at most once, so an id that already carries one is not
/// marked twice.
///
/// # Examples
///
/// ```
/// use open_claude_code_backend::{with_one_million_suffix, ContextWindow};
///
/// assert_eq!(
///     with_one_million_suffix("qwen3.8-max", ContextWindow::OneMillion),
///     "qwen3.8-max[1m]"
/// );
/// assert_eq!(
///     with_one_million_suffix("qwen3.8-max[1m]", ContextWindow::OneMillion),
///     "qwen3.8-max[1m]"
/// );
/// assert_eq!(
///     with_one_million_suffix("qwen3.8-max", ContextWindow::Standard),
///     "qwen3.8-max"
/// );
/// ```
pub fn with_one_million_suffix(model_id: &str, window: ContextWindow) -> String {
    let (bare_id, _) = split_one_million_suffix(model_id);
    match window {
        ContextWindow::OneMillion => format!("{bare_id}{ONE_MILLION_SUFFIX}"),
        ContextWindow::Standard => bare_id.to_owned(),
    }
}
