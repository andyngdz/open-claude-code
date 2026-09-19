use std::path::{Path, PathBuf};

use open_claude_code_backend::open_code_go_public_model_id;

use super::macos_shell::{applescript_escape, shell_launch_command, write_launch_script};
use super::proxy::ClaudeProxyEnv;
use super::terminal_args::{
    CommandSpec, DIRECTORY_ARGUMENT, EXECUTE_ARGUMENT, WORKING_DIRECTORY_ARGUMENT,
};
use crate::features::{errors::LauncherError, launcher::TerminalKind};

const OPEN_PROGRAM: &str = "/usr/bin/open";
const OSASCRIPT_PROGRAM: &str = "/usr/bin/osascript";

/// Resolves a macOS launch command for the selected terminal.
pub(super) fn resolve_macos_command(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<CommandSpec, LauncherError> {
    let concrete = match terminal {
        TerminalKind::SystemDefault => TerminalKind::AppleTerminal,
        TerminalKind::AppleTerminal
        | TerminalKind::ITerm2
        | TerminalKind::Ghostty
        | TerminalKind::Warp
        | TerminalKind::Kitty
        | TerminalKind::Alacritty
        | TerminalKind::WezTerm
        | TerminalKind::Hyper
        | TerminalKind::GnomeTerminal
        | TerminalKind::Konsole => terminal,
    };
    resolve_concrete_macos(concrete, workspace, claude_path, model_id, proxy_env)
}

fn resolve_concrete_macos(
    concrete: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<CommandSpec, LauncherError> {
    match concrete {
        TerminalKind::AppleTerminal => osascript_for_app(
            concrete,
            workspace,
            claude_path,
            model_id,
            proxy_env,
            OsascriptKind::AppleTerminal,
        ),
        TerminalKind::ITerm2 => osascript_for_app(
            concrete,
            workspace,
            claude_path,
            model_id,
            proxy_env,
            OsascriptKind::ITerm2,
        ),
        TerminalKind::Ghostty => open_with_wrapper(
            concrete,
            workspace,
            claude_path,
            model_id,
            proxy_env,
            WrapperStyle::Ghostty,
        ),
        TerminalKind::Kitty => open_with_wrapper(
            concrete,
            workspace,
            claude_path,
            model_id,
            proxy_env,
            WrapperStyle::Kitty,
        ),
        TerminalKind::Alacritty => open_with_wrapper(
            concrete,
            workspace,
            claude_path,
            model_id,
            proxy_env,
            WrapperStyle::Alacritty,
        ),
        TerminalKind::Warp | TerminalKind::WezTerm | TerminalKind::Hyper => {
            open_with_command_file(concrete, workspace, claude_path, model_id, proxy_env)
        }
        TerminalKind::GnomeTerminal | TerminalKind::Konsole | TerminalKind::SystemDefault => {
            Err(LauncherError::TerminalNotFound)
        }
    }
}

enum OsascriptKind {
    AppleTerminal,
    ITerm2,
}

enum WrapperStyle {
    Ghostty,
    Kitty,
    Alacritty,
}

fn require_app_name(kind: TerminalKind) -> Result<&'static str, LauncherError> {
    kind.macos_app_name().ok_or(LauncherError::TerminalNotFound)
}

fn osascript_for_app(
    kind: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
    style: OsascriptKind,
) -> Result<CommandSpec, LauncherError> {
    let app_name = require_app_name(kind)?;
    let shell = shell_launch_command(workspace, claude_path, model_id, proxy_env);
    let escaped = applescript_escape(&shell);
    let script = match style {
        OsascriptKind::AppleTerminal => {
            format!("tell application \"{app_name}\"\nactivate\ndo script \"{escaped}\"\nend tell")
        }
        OsascriptKind::ITerm2 => format!(
            "tell application \"{app_name}\"\ncreate window with default profile\ntell current session of current window\nwrite text \"{escaped}\"\nend tell\nend tell"
        ),
    };
    Ok(CommandSpec {
        program: PathBuf::from(OSASCRIPT_PROGRAM),
        arguments: vec!["-e".to_owned(), script],
    })
}

fn open_with_wrapper(
    kind: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
    style: WrapperStyle,
) -> Result<CommandSpec, LauncherError> {
    let app_name = require_app_name(kind)?;
    let public_model = open_code_go_public_model_id(model_id);
    let wrapper = write_launch_script(workspace, claude_path, &public_model, proxy_env)?;
    let wrapper_value = wrapper.to_string_lossy().into_owned();
    let workspace_value = workspace.to_string_lossy().into_owned();
    let mut arguments = vec!["-na".to_owned(), app_name.to_owned(), "--args".to_owned()];
    match style {
        WrapperStyle::Ghostty => {
            arguments.push(format!("{WORKING_DIRECTORY_ARGUMENT}={workspace_value}"));
            arguments.push(EXECUTE_ARGUMENT.to_owned());
            arguments.push(wrapper_value);
        }
        WrapperStyle::Kitty => {
            arguments.push(DIRECTORY_ARGUMENT.to_owned());
            arguments.push(workspace_value);
            arguments.push(wrapper_value);
        }
        WrapperStyle::Alacritty => {
            arguments.push(WORKING_DIRECTORY_ARGUMENT.to_owned());
            arguments.push(workspace_value);
            arguments.push(EXECUTE_ARGUMENT.to_owned());
            arguments.push(wrapper_value);
        }
    }
    Ok(CommandSpec {
        program: PathBuf::from(OPEN_PROGRAM),
        arguments,
    })
}

fn open_with_command_file(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<CommandSpec, LauncherError> {
    let app_name = require_app_name(terminal)?;
    let public_model = open_code_go_public_model_id(model_id);
    let command_file = write_launch_script(workspace, claude_path, &public_model, proxy_env)?;
    Ok(CommandSpec {
        program: PathBuf::from(OPEN_PROGRAM),
        arguments: vec![
            "-na".to_owned(),
            app_name.to_owned(),
            command_file.to_string_lossy().into_owned(),
        ],
    })
}

#[cfg(test)]
#[path = "macos_launch_test.rs"]
mod macos_launch_test;
