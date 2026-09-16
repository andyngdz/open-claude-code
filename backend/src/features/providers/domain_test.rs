use super::{AuthMethod, ProviderDescriptor, ProviderId, ProviderProtocol};

#[test]
fn provider_descriptor_keeps_extension_metadata_together() {
    let descriptor = ProviderDescriptor {
        id: ProviderId::new("example"),
        display_name: "Example".to_owned(),
        auth_methods: vec![AuthMethod::ApiKey, AuthMethod::OAuth],
        protocols: vec![ProviderProtocol::AnthropicMessages],
    };

    assert_eq!(descriptor.id.as_str(), "example");
    assert_eq!(descriptor.auth_methods.len(), 2);
}
