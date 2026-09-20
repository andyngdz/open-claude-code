#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use open_claude_code_backend::ContextWindow;

use super::{
    default_model_index, parse_model_flag, remembered_model_index, resolve_model_choice,
    runtime_path_in, RuntimeEndpoint, RuntimeModel,
};
use crate::features::settings::ModelAliasMapping;

#[test]
fn empty_choice_uses_the_sonnet_alias() {
    let models = sample_models();
    let default_index = default_model_index(&models, "qwen3.8-flash");

    assert_eq!(
        resolve_model_choice("\n", &models, default_index).as_deref(),
        Some("qwen3.8-flash")
    );
}

#[test]
fn numbered_choice_and_model_id_both_resolve() {
    let models = sample_models();

    assert_eq!(
        resolve_model_choice("2", &models, 0).as_deref(),
        Some("qwen3.8-flash")
    );
    assert_eq!(
        resolve_model_choice("qwen3.8-max", &models, 0).as_deref(),
        Some("qwen3.8-max")
    );
}

#[test]
fn model_flag_parser_accepts_equals_and_rejects_unknown_args() {
    let selected = parse_model_flag(&[
        crate::constants::MODEL_FLAG.to_owned(),
        "qwen3.8-max".to_owned(),
    ]);
    let inline = parse_model_flag(&[format!("{}=qwen3.8-flash", crate::constants::MODEL_FLAG)]);
    let rejected = parse_model_flag(&["--workspace".to_owned()]);

    assert_eq!(selected.unwrap().as_deref(), Some("qwen3.8-max"));
    assert_eq!(inline.unwrap().as_deref(), Some("qwen3.8-flash"));
    assert!(rejected.is_err());
}

#[test]
fn runtime_file_round_trips() {
    let directory =
        std::env::temp_dir().join(format!("open-claude-code-runtime-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    let path = runtime_path_in(&directory);
    let endpoint = sample_endpoint("http://127.0.0.1:9", "local-token");

    endpoint.write(&path).unwrap();
    let loaded = RuntimeEndpoint::read(&path).unwrap();
    assert_eq!(loaded.base_url, endpoint.base_url);
    assert_eq!(loaded.token, "local-token");
    #[cfg(unix)]
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o077,
        0
    );
    RuntimeEndpoint::remove(&path).unwrap();
    std::fs::remove_dir_all(&directory).unwrap();
}

#[test]
fn runtime_file_replaces_a_previous_handshake() {
    let directory = std::env::temp_dir().join(format!(
        "open-claude-code-runtime-replace-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    let path = runtime_path_in(&directory);
    let initial_endpoint = sample_endpoint("http://127.0.0.1:9", "initial-token");
    let replacement_endpoint = sample_endpoint("http://127.0.0.1:10", "replacement-token");

    initial_endpoint.write(&path).unwrap();
    replacement_endpoint.write(&path).unwrap();

    let loaded = RuntimeEndpoint::read(&path).unwrap();
    assert_eq!(loaded.base_url, replacement_endpoint.base_url);
    assert_eq!(loaded.token, replacement_endpoint.token);
    RuntimeEndpoint::remove(&path).unwrap();
    std::fs::remove_dir_all(&directory).unwrap();
}

#[test]
fn remembered_model_index_only_accepts_a_model_still_in_the_catalog() {
    let models = sample_models();

    assert_eq!(
        remembered_model_index(&models, Some("qwen3.8-flash")),
        Some(1)
    );
    assert_eq!(remembered_model_index(&models, Some("retired-model")), None);
    assert_eq!(remembered_model_index(&models, None), None);
}

#[test]
fn a_handshake_without_the_default_row_keys_reads_as_unticked() {
    let serialized = r#"{"baseUrl":"http://127.0.0.1:9","token":"local-token","models":[],"aliases":{"fable":"qwen3.8-max","opus":"qwen3.8-max","sonnet":"qwen3.8-max","haiku":"qwen3.8-flash"}}"#;

    let endpoint: RuntimeEndpoint = serde_json::from_str(serialized).unwrap();

    assert_eq!(endpoint.launch_model_id, None);
    assert_eq!(endpoint.launch_extended_context, ContextWindow::Standard);
}

fn sample_endpoint(base_url: &str, token: &str) -> RuntimeEndpoint {
    RuntimeEndpoint {
        base_url: base_url.to_owned(),
        token: token.to_owned(),
        models: sample_models(),
        aliases: ModelAliasMapping::default(),
        launch_model_id: None,
        launch_extended_context: ContextWindow::Standard,
    }
}

fn sample_models() -> Vec<RuntimeModel> {
    vec![
        RuntimeModel {
            id: "qwen3.8-max".to_owned(),
            display_name: "Qwen 3.8 Max".to_owned(),
        },
        RuntimeModel {
            id: "qwen3.8-flash".to_owned(),
            display_name: "Qwen 3.8 Flash".to_owned(),
        },
    ]
}
