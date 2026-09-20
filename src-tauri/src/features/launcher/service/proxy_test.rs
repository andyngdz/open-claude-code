use std::{fs, path::Path};

use open_claude_code_backend::{open_code_go_public_model_id, ContextWindow};

use super::super::macos_shell::write_launch_script;
use super::{claude_proxy_env, proxy_variables, ClaudeProxyEnv};
use crate::features::settings::ModelAliasMapping;

const EXPECTED_NAMES: [&str; 7] = [
    "ANTHROPIC_BASE_URL",
    "ANTHROPIC_AUTH_TOKEN",
    "CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY",
    "ANTHROPIC_DEFAULT_FABLE_MODEL",
    "ANTHROPIC_DEFAULT_OPUS_MODEL",
    "ANTHROPIC_DEFAULT_SONNET_MODEL",
    "ANTHROPIC_DEFAULT_HAIKU_MODEL",
];

fn sample_proxy_env() -> ClaudeProxyEnv {
    claude_proxy_env("http://127.0.0.1:9", "token", &ModelAliasMapping::default())
}

#[test]
fn proxy_variables_name_every_setting_claude_code_reads() {
    let names: Vec<&str> = proxy_variables(&sample_proxy_env())
        .into_iter()
        .map(|variable| variable.name)
        .collect();

    assert_eq!(names, EXPECTED_NAMES);
}

#[test]
fn a_ticked_alias_carries_the_one_million_marker() {
    let mut aliases = ModelAliasMapping::default();
    aliases.extended.opus = ContextWindow::OneMillion;
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);

    assert_eq!(
        proxy_env.opus,
        format!("{}[1m]", open_code_go_public_model_id(&aliases.opus))
    );
    assert!(!proxy_env.fable.ends_with("[1m]"));
    assert!(!proxy_env.sonnet.ends_with("[1m]"));
    assert!(!proxy_env.haiku.ends_with("[1m]"));
}

#[test]
fn an_unticked_mapping_leaves_every_proxy_id_bare() {
    let aliases = ModelAliasMapping::default();
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);

    assert_eq!(
        proxy_env.fable,
        open_code_go_public_model_id(&aliases.fable)
    );
    assert_eq!(proxy_env.opus, open_code_go_public_model_id(&aliases.opus));
    assert_eq!(
        proxy_env.sonnet,
        open_code_go_public_model_id(&aliases.sonnet)
    );
    assert_eq!(
        proxy_env.haiku,
        open_code_go_public_model_id(&aliases.haiku)
    );
}

#[test]
fn every_proxy_variable_reaches_the_macos_launch_script() {
    let proxy_env = sample_proxy_env();
    let script_path = write_launch_script(
        Path::new("/Users/me/src/app"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
        &proxy_env,
    )
    .expect("wrapper should write");
    let body = fs::read_to_string(&script_path).expect("wrapper should be readable");
    fs::remove_file(&script_path).ok();

    for variable in proxy_variables(&proxy_env) {
        assert!(
            body.contains(&format!("export {}=", variable.name)),
            "{} is missing from the macOS launch script",
            variable.name
        );
    }
}
