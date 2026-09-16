use async_trait::async_trait;
use secrecy::SecretString;

use super::{
    catalog::fallback_catalog, client::OpenCodeGoClient, credential::OpenCodeGoCredentialStore,
};
use crate::features::providers::{
    AuthMethod, ModelCatalogEntry, Provider, ProviderConnectionState, ProviderDescriptor,
    ProviderError, ProviderId, ProviderProtocol, ProviderRequest, ProviderResponse,
};

pub(crate) const OPENCODE_GO_PROVIDER_ID: &str = "opencode-go";

/// Implements the provider contract for an OpenCode Go subscription.
pub(crate) struct OpenCodeGoProvider {
    client: OpenCodeGoClient,
    credentials: OpenCodeGoCredentialStore,
    descriptor: ProviderDescriptor,
}

impl OpenCodeGoProvider {
    /// Creates the OpenCode Go adapter and its shared provider metadata.
    pub(crate) fn new() -> Result<Self, ProviderError> {
        Ok(Self {
            client: OpenCodeGoClient::new()?,
            credentials: OpenCodeGoCredentialStore,
            descriptor: ProviderDescriptor {
                id: ProviderId::new(OPENCODE_GO_PROVIDER_ID),
                display_name: "OpenCode Go".to_owned(),
                auth_methods: vec![AuthMethod::ApiKey],
                protocols: vec![ProviderProtocol::AnthropicMessages],
            },
        })
    }

    /// Returns the checked-in catalog used before the first refresh.
    pub(crate) fn fallback_catalog() -> Vec<ModelCatalogEntry> {
        fallback_catalog()
    }
}

#[async_trait]
impl Provider for OpenCodeGoProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    async fn connection_state(&self) -> ProviderConnectionState {
        match self.credentials.load().await {
            Ok(_api_key) => ProviderConnectionState::Connected,
            Err(ProviderError::CredentialNotFound) => ProviderConnectionState::Disconnected,
            Err(source) => ProviderConnectionState::Failed {
                message: source.to_string(),
            },
        }
    }

    async fn save_and_test_api_key(
        &self,
        raw_api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        let trimmed_api_key = raw_api_key.trim();
        if trimmed_api_key.is_empty() {
            return Err(ProviderError::EmptyApiKey);
        }
        let api_key = SecretString::from(trimmed_api_key.to_owned());
        self.client.validate_api_key(&api_key).await?;
        let catalog = self.client.fetch_catalog(&api_key).await?;
        self.credentials.save(api_key).await?;
        Ok(catalog)
    }

    async fn remove_credential(&self) -> Result<(), ProviderError> {
        self.credentials.remove().await
    }

    async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        let api_key = self.credentials.load().await?;
        self.client.fetch_catalog(&api_key).await
    }

    async fn forward_messages(
        &self,
        request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        let api_key = self.credentials.load().await?;
        self.client.forward_messages(&api_key, request).await
    }
}

#[cfg(test)]
#[path = "provider_test.rs"]
mod provider_test;
