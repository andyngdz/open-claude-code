use std::{fs, path::PathBuf};

use uuid::Uuid;

use super::{should_check_terminal_liveness, spawn_terminal};
use crate::features::launcher::service::{claude_proxy_env, terminal_args::CommandSpec};
use crate::features::launcher::TerminalKind;
use crate::features::settings::ModelAliasMapping;

#[test]
fn system_default_does_not_require_terminal_liveness_check() {
    assert!(!should_check_terminal_liveness(TerminalKind::SystemDefault));
    #[cfg(not(target_os = "macos"))]
    assert!(should_check_terminal_liveness(TerminalKind::Ghostty));
    #[cfg(target_os = "macos")]
    assert!(!should_check_terminal_liveness(TerminalKind::Ghostty));
}

#[test]
fn failed_spawn_removes_the_pending_launch_script() {
    let script_path =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    fs::write(&script_path, "token").expect("pending script should be written");
    let aliases = ModelAliasMapping::default();
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command_spec = CommandSpec {
        program: PathBuf::from("/missing/open-claude-code-terminal"),
        arguments: Vec::new(),
        cleanup_path: Some(script_path.clone()),
    };

    let result = spawn_terminal(&command_spec, PathBuf::from("/").as_path(), &proxy_env);

    assert!(result.is_err());
    assert!(!script_path.exists());
}
