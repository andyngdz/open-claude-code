use super::{OpenCodeGoProvider, OPENCODE_GO_PROVIDER_ID};
use crate::features::providers::{AuthMethod, Provider, ProviderError};

#[test]
fn descriptor_exposes_provider_capabilities() {
    let provider = OpenCodeGoProvider::new().expect("HTTP client should initialize");

    assert_eq!(provider.descriptor().id.as_str(), OPENCODE_GO_PROVIDER_ID);
    assert_eq!(provider.descriptor().auth_methods, vec![AuthMethod::ApiKey]);
}

#[tokio::test]
async fn rejects_a_blank_api_key_before_saving() {
    let provider = OpenCodeGoProvider::new().expect("HTTP client should initialize");

    let error = provider
        .save_api_key("   ")
        .await
        .expect_err("blank keys must not reach the keyring");

    assert!(matches!(error, ProviderError::EmptyApiKey));
}
