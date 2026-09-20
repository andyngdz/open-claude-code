use super::{nonempty_catalog, runtime_models, save_settings_with, with_cli_launch_model};
use crate::features::settings::{AppSettings, SettingsStore};
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

/// A hand-written document, the shape the app finds on disk when it starts.
fn settings_document_on_disk() -> Vec<u8> {
    serde_json::to_vec_pretty(&serde_json::json!({
        "version": 1,
        "terminal": "system_default",
        "lastWorkspace": null,
        "aliases": {
            "fable": "qwen3.8-max",
            "opus": "qwen3.8-max",
            "sonnet": "qwen3.8-max",
            "haiku": "qwen3.8-flash",
        },
        "launchModelId": "qwen3.8-max",
        "cliLaunchModelId": "qwen3.8-flash",
        "customModels": [],
        "cachedModels": [],
        "catalogRefreshedAtEpochSeconds": null,
    }))
    .expect("fixture should serialize in a test")
}

#[test]
fn an_app_save_keeps_the_cli_model_that_is_on_disk() {
    let directory =
        std::env::temp_dir().join(format!("open-claude-code-app-save-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("test directory should be created");
    let path = directory.join("settings.json");
    std::fs::write(&path, settings_document_on_disk()).expect("fixture should be written");
    let store = SettingsStore::for_path(path.clone());

    // This process loaded its copy before the CLI wrote the field, so its own value is absent.
    save_settings_with(&store, &AppSettings::default()).expect("a save should succeed");

    let written = std::fs::read_to_string(&path).expect("settings should be readable");
    assert!(
        written.contains("\"cliLaunchModelId\": \"qwen3.8-flash\""),
        "the model the CLI remembered should survive an app save, wrote {written}"
    );
    std::fs::remove_dir_all(&directory).expect("test directory should be removable");
}
