use std::{fs, path::PathBuf, time::Duration};

use uuid::Uuid;

#[cfg(not(target_os = "macos"))]
use super::should_check_terminal_liveness;
use super::{ensure_helper_succeeded, spawn_terminal};
use crate::features::errors::LauncherError;
use crate::features::launcher::service::{
    claude_proxy_env,
    terminal_args::{CommandEnvironment, CommandSpec},
};
#[cfg(not(target_os = "macos"))]
use crate::features::launcher::TerminalKind;
use crate::features::settings::ModelAliasMapping;

#[test]
#[cfg(not(target_os = "macos"))]
fn system_default_does_not_require_terminal_liveness_check() {
    assert!(!should_check_terminal_liveness(TerminalKind::SystemDefault));
    assert!(should_check_terminal_liveness(TerminalKind::Ghostty));
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
        environment: CommandEnvironment::Proxy,
    };

    let result = spawn_terminal(&command_spec, PathBuf::from("/").as_path(), &proxy_env);

    assert!(result.is_err());
    assert!(!script_path.exists());
}

#[test]
fn failed_helper_removes_the_pending_launch_script() {
    let script_path =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    fs::write(&script_path, "token").expect("pending script should be written");
    let aliases = ModelAliasMapping::default();
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command_spec = CommandSpec {
        program: PathBuf::from("/bin/sh"),
        arguments: vec!["-c".to_owned(), "exit 1".to_owned()],
        cleanup_path: Some(script_path.clone()),
        environment: CommandEnvironment::Inherit,
    };
    let mut child = spawn_terminal(&command_spec, PathBuf::from("/").as_path(), &proxy_env)
        .expect("helper should spawn");

    let result = ensure_helper_succeeded(&mut child, &command_spec, Duration::from_secs(1));

    assert!(matches!(result, Err(LauncherError::TerminalExited)));
    assert!(!script_path.exists());
}

#[test]
fn successful_helper_keeps_the_wrapper_for_the_terminal() {
    let script_path =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    fs::write(&script_path, "token").expect("pending script should be written");
    let aliases = ModelAliasMapping::default();
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command_spec = CommandSpec {
        program: PathBuf::from("/bin/true"),
        arguments: Vec::new(),
        cleanup_path: Some(script_path.clone()),
        environment: CommandEnvironment::Inherit,
    };
    let mut child = spawn_terminal(&command_spec, PathBuf::from("/").as_path(), &proxy_env)
        .expect("helper should spawn");

    ensure_helper_succeeded(&mut child, &command_spec, Duration::from_secs(1))
        .expect("helper should succeed");

    assert!(script_path.exists());
    fs::remove_file(script_path).expect("pending script should be removable");
}

#[test]
fn timed_out_helper_is_stopped_and_removes_the_pending_launch_script() {
    let script_path =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    fs::write(&script_path, "token").expect("pending script should be written");
    let aliases = ModelAliasMapping::default();
    let proxy_env = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command_spec = CommandSpec {
        program: PathBuf::from("/bin/sleep"),
        arguments: vec!["5".to_owned()],
        cleanup_path: Some(script_path.clone()),
        environment: CommandEnvironment::Inherit,
    };
    let mut child = spawn_terminal(&command_spec, PathBuf::from("/").as_path(), &proxy_env)
        .expect("helper should spawn");

    let result = ensure_helper_succeeded(&mut child, &command_spec, Duration::ZERO);

    assert!(matches!(result, Err(LauncherError::TerminalExited)));
    assert!(!script_path.exists());
    assert!(child
        .try_wait()
        .expect("helper status should be readable")
        .is_some());
}
