use super::{nonempty_catalog, runtime_models};
use open_claude_code_backend::{ModelCatalogEntry, OpenCodeGoBackend};

#[test]
fn empty_catalog_uses_the_fallback_list() {
    let catalog = nonempty_catalog(Vec::new());

    assert!(!catalog.is_empty());
    assert_eq!(catalog, OpenCodeGoBackend::fallback_catalog());
}

#[test]
fn runtime_models_include_custom_models_once() {
    let catalog_models = vec![ModelCatalogEntry {
        id: "qwen3.8-max".to_owned(),
        display_name: "Qwen 3.8 Max".to_owned(),
        is_custom: false,
    }];
    let custom_model_ids = vec!["custom-model".to_owned(), "qwen3.8-max".to_owned()];

    let models = runtime_models(catalog_models, &custom_model_ids);

    assert_eq!(models.len(), 2);
    assert_eq!(models[1].id, "custom-model");
    assert_eq!(models[1].display_name, "custom-model");
}
