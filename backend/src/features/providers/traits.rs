use async_trait::async_trait;
use secrecy::SecretString;

use super::{
    ModelCatalogEntry, ProviderConnectionState, ProviderDescriptor, ProviderError, ProviderRequest,
    ProviderResponse,
};

/// Defines the extension contract implemented by every AI provider adapter.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Returns stable provider metadata for settings and capability discovery.
    fn descriptor(&self) -> &ProviderDescriptor;

    /// Returns the provider connection state without exposing credentials.
    async fn connection_state(&self) -> ProviderConnectionState;

    /// Loads the stored credential for the local settings surface.
    ///
    /// Reports `CredentialNotFound` when no credential is saved, so a caller
    /// distinguishes "nothing stored" from "the credential store failed".
    async fn load_saved_api_key(&self) -> Result<SecretString, ProviderError>;

    /// Validates and stores an API key, then returns the supported catalog.
    async fn save_and_test_api_key(
        &self,
        api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, ProviderError>;

    /// Removes the stored provider credential.
    async fn remove_credential(&self) -> Result<(), ProviderError>;

    /// Refreshes the provider's compatible model catalog.
    async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, ProviderError>;

    /// Forwards one Anthropic Messages request to the provider.
    async fn forward_messages(
        &self,
        request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError>;
}
