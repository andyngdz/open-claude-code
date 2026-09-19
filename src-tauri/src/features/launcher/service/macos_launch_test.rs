use std::{
    fs,
    path::{Path, PathBuf},
};

use super::resolve_macos_command;
use crate::features::launcher::service::proxy::claude_proxy_env;
use crate::features::launcher::service::terminal_args::CommandEnvironment;
use crate::features::launcher::TerminalKind;
use crate::features::settings::ModelAliasMapping;

fn sample_proxy_env() -> crate::features::launcher::service::proxy::ClaudeProxyEnv {
    let aliases = ModelAliasMapping::default();
    claude_proxy_env("http://127.0.0.1:9", "token", &aliases)
}

#[test]
fn applescript_terminals_use_private_wrappers_without_exposing_credentials() {
    for terminal in [TerminalKind::SystemDefault, TerminalKind::ITerm2] {
        let command = resolve_macos_command(
            terminal,
            Path::new("/Users/me/src/app"),
            Path::new("/usr/bin/claude"),
            "qwen3.8-max",
            &sample_proxy_env(),
        )
        .expect("AppleScript terminal should resolve");

        assert_eq!(command.program.as_os_str(), "/usr/bin/osascript");
        assert_eq!(command.environment, CommandEnvironment::Inherit);
        assert!(!command.arguments[1].contains("token"));
        let script_path = command
            .cleanup_path
            .expect("AppleScript launch should own a private wrapper");
        assert!(command.arguments[1].contains(&script_path.to_string_lossy().into_owned()));
        let body = fs::read_to_string(&script_path).expect("wrapper should be readable");
        fs::remove_file(&script_path).expect("wrapper should be removable");

        assert!(body.contains("cd '/Users/me/src/app' || exit 1"));
        assert!(body.contains("ANTHROPIC_AUTH_TOKEN='token'"));
    }
}

#[test]
fn direct_wrappers_use_a_workspace_script_without_executing_a_shell_binary() {
    for terminal in [
        TerminalKind::Ghostty,
        TerminalKind::Kitty,
        TerminalKind::Alacritty,
    ] {
        let command = resolve_macos_command(
            terminal,
            Path::new("/Users/me/src/app"),
            Path::new("/usr/bin/claude"),
            "qwen3.8-max",
            &sample_proxy_env(),
        )
        .expect("terminal should resolve");
        let script_path = PathBuf::from(command.arguments.last().expect("wrapper should exist"));
        let body = fs::read_to_string(&script_path).expect("wrapper should be readable");
        fs::remove_file(&script_path).expect("wrapper should be removable");

        assert_eq!(command.program.as_os_str(), "/usr/bin/open");
        assert_eq!(command.environment, CommandEnvironment::Inherit);
        assert!(command
            .arguments
            .iter()
            .all(|argument| argument != "/bin/sh" && argument != "-c"));
        assert!(body.contains("cd '/Users/me/src/app' || exit 1"));
    }
}

#[test]
fn command_file_terminals_change_to_the_selected_workspace_before_launching() {
    for terminal in [
        TerminalKind::Warp,
        TerminalKind::WezTerm,
        TerminalKind::Hyper,
    ] {
        let command = resolve_macos_command(
            terminal,
            Path::new("/Users/me/src/app"),
            Path::new("/usr/bin/claude"),
            "qwen3.8-max",
            &sample_proxy_env(),
        )
        .expect("terminal should resolve");
        let script_path = PathBuf::from(&command.arguments[2]);
        let body = fs::read_to_string(&script_path).expect("wrapper should be readable");
        fs::remove_file(&script_path).expect("wrapper should be removable");

        assert!(body.contains("cd '/Users/me/src/app' || exit 1"));
    }
}
