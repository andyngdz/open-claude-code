use super::{launch, prompt_default_index, select_model, CliError};
use crate::features::{
    runtime_endpoint::{RuntimeEndpoint, RuntimeModel},
    settings::ModelAliasMapping,
};

#[test]
fn launch_without_a_running_app_reports_not_running() {
    let error = launch(Some("missing".to_owned()), &[]).unwrap_err();

    assert!(matches!(
        error,
        CliError::NotRunning | CliError::UnknownModel
    ));
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

    assert_eq!(
        prompt_default_index(&endpoint, Some("qwen3.8-flash"), Some("qwen3.8-max")),
        1
    );
    assert_eq!(
        prompt_default_index(&endpoint, None, Some("qwen3.8-flash")),
        1
    );
    assert_eq!(
        prompt_default_index(&endpoint, Some("retired-model"), Some("retired-model")),
        0
    );
    assert_eq!(prompt_default_index(&endpoint, None, None), 0);
}

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
        aliases: ModelAliasMapping::default(),
    }
}
