use super::{OpenCodeGoProvider, OPENCODE_GO_PROVIDER_ID};
use crate::features::providers::{AuthMethod, Provider};

#[test]
fn descriptor_exposes_provider_capabilities() {
    let provider = OpenCodeGoProvider::new().expect("HTTP client should initialize");

    assert_eq!(provider.descriptor().id.as_str(), OPENCODE_GO_PROVIDER_ID);
    assert_eq!(provider.descriptor().auth_methods, vec![AuthMethod::ApiKey]);
}
