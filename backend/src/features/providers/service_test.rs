use std::sync::Arc;

use async_trait::async_trait;
use secrecy::SecretString;

use super::ProviderRegistry;
use crate::features::providers::{
    AuthMethod, ModelCatalogEntry, Provider, ProviderConnectionState, ProviderDescriptor,
    ProviderError, ProviderId, ProviderProtocol, ProviderRequest, ProviderResponse,
};

struct FakeProvider {
    descriptor: ProviderDescriptor,
}

#[async_trait]
impl Provider for FakeProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    async fn connection_state(&self) -> ProviderConnectionState {
        ProviderConnectionState::Disconnected
    }

    async fn load_saved_api_key(&self) -> Result<SecretString, ProviderError> {
        Err(ProviderError::CredentialNotFound)
    }

    async fn save_and_test_api_key(
        &self,
        _api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        Ok(Vec::new())
    }

    async fn remove_credential(&self) -> Result<(), ProviderError> {
        Ok(())
    }

    async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        Ok(Vec::new())
    }

    async fn forward_messages(
        &self,
        _request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        Err(ProviderError::UnexpectedResponse)
    }
}

#[test]
fn resolves_provider_by_stable_identifier() {
    let provider_id = ProviderId::new("fake");
    let provider = Arc::new(FakeProvider {
        descriptor: ProviderDescriptor {
            id: provider_id.clone(),
            display_name: "Fake".to_owned(),
            auth_methods: vec![AuthMethod::ApiKey],
            protocols: vec![ProviderProtocol::AnthropicMessages],
        },
    });
    let registry = ProviderRegistry::new(vec![provider]);

    let resolved = registry
        .get(&provider_id)
        .expect("provider should be registered");

    assert_eq!(resolved.descriptor().id, provider_id);
}
