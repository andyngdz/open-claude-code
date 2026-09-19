use std::path::Path;

use super::{applescript_escape, shell_launch_command, write_launch_script};
use crate::features::launcher::service::proxy::claude_proxy_env;
use crate::features::settings::ModelAliasMapping;

#[test]
fn applescript_escape_doubles_backslashes_and_quotes() {
    assert_eq!(applescript_escape("say \"hi\""), "say \\\"hi\\\"");
}

#[test]
fn shell_launch_command_cds_into_the_selected_workspace() {
    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command = shell_launch_command(
        Path::new("/Users/me/src/app"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
        &proxy,
    );

    assert!(command.contains("cd '/Users/me/src/app'"));
    assert!(command.contains("exec '/usr/bin/claude'"));
}

#[test]
fn write_launch_script_cds_into_the_selected_workspace() {
    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let script_path = write_launch_script(
        Path::new("/Users/me/src/app"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
        &proxy,
    )
    .expect("wrapper should write");
    let body = std::fs::read_to_string(&script_path).expect("wrapper should be readable");
    std::fs::remove_file(&script_path).ok();

    assert!(body.contains("cd '/Users/me/src/app'"));
    assert!(!body.contains("cd '/var/folders"));
}
