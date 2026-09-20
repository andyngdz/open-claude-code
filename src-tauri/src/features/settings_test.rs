use super::{normalize_custom_models, AppSettings, ModelAliasMapping};
use open_claude_code_backend::{ContextWindow, DEFAULT_FAST_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID};

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

#[test]
fn the_cli_model_choice_round_trips_under_its_own_key() {
    let settings = AppSettings {
        cli_launch_model_id: Some("qwen3.8-flash".to_owned()),
        ..AppSettings::default()
    };

    let serialized = serde_json::to_string(&settings).expect("settings should serialize in a test");

    assert!(serialized.contains("\"cliLaunchModelId\":\"qwen3.8-flash\""));
    let parsed: AppSettings =
        serde_json::from_str(&serialized).expect("settings should deserialize in a test");
    assert_eq!(parsed.cli_launch_model_id.as_deref(), Some("qwen3.8-flash"));
}

#[test]
fn older_settings_without_a_cli_model_choice_still_load() {
    let settings = AppSettings::default();
    let mut serialized =
        serde_json::to_value(&settings).expect("settings should serialize in a test");
    serialized
        .as_object_mut()
        .expect("settings JSON should be an object")
        .remove("cliLaunchModelId");

    let parsed: AppSettings =
        serde_json::from_value(serialized).expect("older settings should deserialize in a test");

    assert_eq!(parsed.cli_launch_model_id, None);
}

#[test]
fn an_unticked_alias_leaves_its_model_on_the_default_window() {
    let aliases = ModelAliasMapping::default();

    assert_eq!(
        aliases.declared_window(DEFAULT_PRIMARY_MODEL_ID),
        ContextWindow::Standard
    );
}

#[test]
fn a_ticked_alias_marks_the_model_it_points_at_and_no_other() {
    let mut aliases = ModelAliasMapping::default();
    aliases.extended.haiku = ContextWindow::OneMillion;

    assert_eq!(
        aliases.declared_window(DEFAULT_FAST_MODEL_ID),
        ContextWindow::OneMillion
    );
    assert_eq!(
        aliases.declared_window(DEFAULT_PRIMARY_MODEL_ID),
        ContextWindow::Standard
    );
}

#[test]
fn one_ticked_row_is_enough_when_two_rows_share_a_model() {
    let mut aliases = ModelAliasMapping::default();
    aliases.extended.fable = ContextWindow::OneMillion;

    // Sonnet, Opus and Fable all point at the primary model by default, so the
    // unticked rows that follow Fable must not switch its tick back off.
    assert_eq!(
        aliases.declared_window(DEFAULT_PRIMARY_MODEL_ID),
        ContextWindow::OneMillion
    );
}

#[test]
fn a_model_no_alias_holds_has_no_one_million_path() {
    let aliases = ModelAliasMapping::default();

    assert_eq!(
        aliases.declared_window("qwen3.8-unknown"),
        ContextWindow::Standard
    );
}

#[test]
fn the_extended_context_round_trips_as_a_checkbox() {
    let mut aliases = ModelAliasMapping::default();
    aliases.extended.fable = ContextWindow::OneMillion;
    let settings = AppSettings {
        aliases,
        ..AppSettings::default()
    };

    let serialized = serde_json::to_string(&settings).expect("settings should serialize in a test");

    assert!(serialized
        .contains("\"extended\":{\"fable\":true,\"opus\":false,\"sonnet\":false,\"haiku\":false}"));
    let parsed: AppSettings =
        serde_json::from_str(&serialized).expect("settings should deserialize in a test");
    assert_eq!(parsed.aliases.extended.fable, ContextWindow::OneMillion);
    assert_eq!(parsed.aliases.extended.opus, ContextWindow::Standard);
}

#[test]
fn older_settings_without_extended_context_still_load() {
    let settings = AppSettings::default();
    let mut serialized =
        serde_json::to_value(&settings).expect("settings should serialize in a test");
    serialized
        .as_object_mut()
        .expect("settings JSON should be an object")
        .get_mut("aliases")
        .expect("settings JSON should hold aliases")
        .as_object_mut()
        .expect("aliases JSON should be an object")
        .remove("extended");

    let parsed: AppSettings =
        serde_json::from_value(serialized).expect("older settings should deserialize in a test");

    assert_eq!(parsed.aliases.extended.fable, ContextWindow::Standard);
    assert_eq!(parsed.aliases.extended.haiku, ContextWindow::Standard);
}
