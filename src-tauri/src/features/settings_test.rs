use super::{normalize_custom_models, AppSettings, ModelAliasMapping};
use open_claude_code_backend::{DEFAULT_FAST_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID};

#[test]
fn default_aliases_use_messages_compatible_models() {
    let aliases = ModelAliasMapping::default();

    assert_eq!(aliases.opus, DEFAULT_PRIMARY_MODEL_ID);
    assert_eq!(aliases.haiku, DEFAULT_FAST_MODEL_ID);
}

#[test]
fn custom_models_are_trimmed_sorted_and_deduplicated() {
    let normalized = normalize_custom_models(vec![
        " custom-beta ".to_owned(),
        "custom-alpha".to_owned(),
        "custom-beta".to_owned(),
        " ".to_owned(),
    ]);

    assert_eq!(normalized, vec!["custom-alpha", "custom-beta"]);
}

#[test]
fn settings_round_trip_preserves_versioned_shape() {
    let settings = AppSettings::default();
    let serialized = serde_json::to_string(&settings).expect("settings should serialize in a test");
    let parsed: AppSettings =
        serde_json::from_str(&serialized).expect("settings should deserialize in a test");

    assert_eq!(parsed.version, 1);
    assert_eq!(parsed.aliases.sonnet, DEFAULT_PRIMARY_MODEL_ID);
}

#[test]
fn older_settings_without_a_launch_model_still_load() {
    let settings = AppSettings::default();
    let mut serialized =
        serde_json::to_value(&settings).expect("settings should serialize in a test");
    serialized
        .as_object_mut()
        .expect("settings JSON should be an object")
        .remove("launchModelId");

    let parsed: AppSettings =
        serde_json::from_value(serialized).expect("older settings should deserialize in a test");

    assert_eq!(parsed.launch_model_id, None);
}
