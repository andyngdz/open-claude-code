use super::{nonempty_catalog, runtime_models, with_cli_launch_model};
use crate::features::settings::AppSettings;
use open_claude_code_backend::ModelCatalogEntry;

#[test]
fn empty_catalog_remains_empty_until_upstream_discovery_returns_models() {
    let catalog = nonempty_catalog(Vec::new());

    assert!(catalog.is_empty());
}

#[test]
fn saving_keeps_the_model_the_cli_remembered() {
    let mut in_memory = AppSettings::default();
    in_memory.launch_model_id = Some("qwen3.8-max".to_owned());
    let mut persisted = AppSettings::default();
    persisted.cli_launch_model_id = Some("qwen3.8-flash".to_owned());

    let merged = with_cli_launch_model(in_memory, Some(persisted));

    assert_eq!(merged.cli_launch_model_id.as_deref(), Some("qwen3.8-flash"));
    assert_eq!(merged.launch_model_id.as_deref(), Some("qwen3.8-max"));
}

#[test]
fn an_unreadable_settings_document_keeps_the_value_this_process_has() {
    let mut in_memory = AppSettings::default();
    in_memory.cli_launch_model_id = Some("qwen3.8-flash".to_owned());

    let merged = with_cli_launch_model(in_memory, None);

    assert_eq!(merged.cli_launch_model_id.as_deref(), Some("qwen3.8-flash"));
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
    let custom_model = &models[1];
    assert_eq!(custom_model.id, "custom-model");
    assert_eq!(custom_model.display_name, "custom-model");
}
