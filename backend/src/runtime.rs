use std::sync::Arc;

use secrecy::SecretString;
use tokio::sync::RwLock;
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
    interface::{start_gateway, GatewayRuntime},
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
            Arc::new(RwLock::new(configuration)),
            Arc::new(RwLock::new(Uuid::new_v4().to_string())),
        )
        .await?;

        Ok(Self { provider, gateway })
    }

    /// Returns the current credential state without exposing the credential.
    pub async fn connection_state(&self) -> ProviderConnectionState {
        self.provider.connection_state().await
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

    /// Returns the checked-in model catalog used before provider discovery succeeds.
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
