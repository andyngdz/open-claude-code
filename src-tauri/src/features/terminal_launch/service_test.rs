use std::io;

use open_claude_code_backend::ContextWindow;

use super::{
    launch_context_window, map_read_error, prompt_default_index, requested_model, select_model,
    CliError,
};
use crate::features::{
    errors::RuntimeEndpointError,
    runtime_endpoint::{RuntimeEndpoint, RuntimeModel},
    settings::ModelAliasMapping,
};

#[test]
fn a_missing_handshake_tells_the_user_to_open_the_app() {
    let error = map_read_error(RuntimeEndpointError::Read(io::Error::new(
        io::ErrorKind::NotFound,
        "no handshake on disk",
    )));

    assert!(matches!(error, CliError::NotRunning));
}

#[test]
fn an_unreadable_handshake_tells_the_user_to_restart_the_app() {
    let error = map_read_error(RuntimeEndpointError::Read(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "the handshake is not readable",
    )));

    assert!(matches!(error, CliError::Unreachable));
}

#[test]
fn a_handshake_that_cannot_be_parsed_tells_the_user_to_restart_the_app() {
    let parse_error = serde_json::from_str::<serde_json::Value>("not json")
        .expect_err("invalid JSON should fail to parse");
    let error = map_read_error(RuntimeEndpointError::Parse(parse_error));

    assert!(matches!(error, CliError::Unreachable));
}

#[test]
fn an_unknown_requested_model_is_rejected_before_claude_starts() {
    let endpoint = sample_endpoint();
    let error = select_model(&endpoint, Some("missing".to_owned()), 0).unwrap_err();

    assert!(matches!(error, CliError::UnknownModel));
}

#[test]
fn a_requested_model_wins_over_the_prompt_default() {
    let endpoint = sample_endpoint();
    let model_id = select_model(&endpoint, Some("qwen3.8-max".to_owned()), 1)
        .expect("a catalog model id should resolve");

    assert_eq!(model_id, "qwen3.8-max");
}

#[test]
fn the_prompt_default_follows_the_cli_then_the_dashboard_then_the_alias() {
    let endpoint = sample_endpoint();

    // The CLI's own pick wins even though the dashboard's is earlier in the catalog.
    assert_eq!(
        prompt_default_index(&endpoint, Some("qwen3.8-flash"), Some("qwen3.8-max")),
        1
    );
    // With no CLI pick the dashboard default applies and the Sonnet alias would not.
    assert_eq!(
        prompt_default_index(&endpoint, None, Some("qwen3.8-max")),
        0
    );
    // A model the catalog no longer offers falls through to the alias, not to the first entry.
    assert_eq!(
        prompt_default_index(&endpoint, Some("retired-model"), Some("retired-model")),
        1
    );
    assert_eq!(prompt_default_index(&endpoint, None, None), 1);
}

/// Sonnet points at the second catalog entry so an alias fallback is not mistaken for the first.
fn sample_endpoint() -> RuntimeEndpoint {
    RuntimeEndpoint {
        base_url: "http://127.0.0.1:9".to_owned(),
        token: "local-token".to_owned(),
        models: vec![
            RuntimeModel {
                id: "qwen3.8-max".to_owned(),
                display_name: "Qwen 3.8 Max".to_owned(),
            },
            RuntimeModel {
                id: "qwen3.8-flash".to_owned(),
                display_name: "Qwen 3.8 Flash".to_owned(),
            },
        ],
        aliases: ModelAliasMapping {
            fable: "qwen3.8-max".to_owned(),
            opus: "qwen3.8-max".to_owned(),
            sonnet: "qwen3.8-flash".to_owned(),
            haiku: "qwen3.8-flash".to_owned(),
            ..ModelAliasMapping::default()
        },
    }
}

#[test]
fn a_marker_on_the_requested_model_is_split_off_before_the_catalog_lookup() {
    let (model_id, window) = requested_model(Some("deepseek-v4.1-flash[1m]".to_owned()));

    assert_eq!(model_id.as_deref(), Some("deepseek-v4.1-flash"));
    assert_eq!(window, ContextWindow::OneMillion);
}

#[test]
fn a_requested_model_without_a_marker_keeps_the_default_window() {
    let (model_id, window) = requested_model(Some("deepseek-v4.1-flash".to_owned()));

    assert_eq!(model_id.as_deref(), Some("deepseek-v4.1-flash"));
    assert_eq!(window, ContextWindow::Standard);
}

#[test]
fn marking_a_model_the_catalog_lacks_is_still_an_unknown_model() {
    let endpoint = sample_endpoint();
    let (model_id, _) = requested_model(Some("missing[1m]".to_owned()));

    let error = select_model(&endpoint, model_id, 0).unwrap_err();

    assert!(matches!(error, CliError::UnknownModel));
}

#[test]
fn a_marker_typed_on_the_command_line_wins_over_the_alias_rows() {
    let endpoint = sample_endpoint();

    assert_eq!(
        launch_context_window(&endpoint, "qwen3.8-max", ContextWindow::OneMillion),
        ContextWindow::OneMillion
    );
    assert_eq!(
        launch_context_window(&endpoint, "qwen3.8-max", ContextWindow::Standard),
        ContextWindow::Standard
    );
}

#[test]
fn a_ticked_alias_row_marks_the_model_the_cli_launches() {
    let mut endpoint = sample_endpoint();
    endpoint.aliases.extended.sonnet = ContextWindow::OneMillion;

    assert_eq!(
        launch_context_window(&endpoint, "qwen3.8-flash", ContextWindow::Standard),
        ContextWindow::OneMillion
    );
    assert_eq!(
        launch_context_window(&endpoint, "qwen3.8-max", ContextWindow::Standard),
        ContextWindow::Standard
    );
}
