use super::{open_code_go_public_model_id, OpenCodeGoBackend};

#[test]
fn public_model_id_keeps_provider_and_model() {
    let public_id = open_code_go_public_model_id("qwen3.8-max");

    assert!(public_id.contains("opencode-go"));
    assert!(public_id.ends_with("/qwen3.8-max"));
}

#[test]
fn fallback_catalog_is_empty_until_authenticated_discovery() {
    let catalog = OpenCodeGoBackend::fallback_catalog();

    assert!(catalog.is_empty());
}
