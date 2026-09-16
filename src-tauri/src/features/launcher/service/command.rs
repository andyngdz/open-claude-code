use std::{
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use super::discovery::{concrete_terminals, find_executable, XDG_TERMINAL_EXECUTABLE};
use super::proxy::{apply_proxy_env, claude_executable, claude_proxy_env};
use open_claude_code_backend::open_code_go_public_model_id;

use crate::features::{
    errors::LauncherError,
    launcher::{LaunchReceipt, ProcessRegistry, TerminalKind},
    settings::{is_workspace_directory, ModelAliasMapping},
};

const MODEL_ARGUMENT: &str = crate::constants::MODEL_FLAG;
const EXECUTE_ARGUMENT: &str = "-e";
const WORKING_DIRECTORY_ARGUMENT: &str = "--working-directory";
const GHOSTTY_OWN_WINDOW: &str = "--gtk-single-instance=false";
const TERMINAL_START_GRACE: Duration = Duration::from_millis(700);

/// Spawns Claude Code in the configured terminal with proxy-only credentials.
pub(crate) fn launch_claude(
    registry: &ProcessRegistry,
    terminal: TerminalKind,
    workspace: &Path,
    model_id: &str,
    proxy_base_url: &str,
    proxy_token: &SecretString,
    aliases: &ModelAliasMapping,
) -> Result<LaunchReceipt, LauncherError> {
    if !is_workspace_directory(workspace) {
        return Err(LauncherError::InvalidWorkspace);
    }
    let claude_path = claude_executable()?;
    let command_spec = resolve_command_spec(terminal, workspace, &claude_path, model_id)?;
    let mut child = spawn_terminal(
        &command_spec,
        workspace,
        proxy_base_url,
        proxy_token,
        aliases,
    )?;
    ensure_terminal_stayed_open(&mut child)?;
    let process_id = child.id();
    registry.register(child)?;

    Ok(LaunchReceipt {
        process_id,
        terminal,
        workspace: workspace.to_path_buf(),
        model_id: model_id.to_owned(),
    })
}

#[derive(Debug, Eq, PartialEq)]
struct CommandSpec {
    program: PathBuf,
    arguments: Vec<String>,
}

fn spawn_terminal(
    command_spec: &CommandSpec,
    workspace: &Path,
    proxy_base_url: &str,
    proxy_token: &SecretString,
    aliases: &ModelAliasMapping,
) -> Result<std::process::Child, LauncherError> {
    let proxy_env = claude_proxy_env(proxy_base_url, proxy_token.expose_secret(), aliases);
    let mut command = Command::new(&command_spec.program);
    command.args(&command_spec.arguments).current_dir(workspace);
    apply_proxy_env(&mut command, &proxy_env);
    command.spawn().map_err(LauncherError::Spawn)
}

fn resolve_command_spec(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
) -> Result<CommandSpec, LauncherError> {
    if terminal != TerminalKind::SystemDefault {
        return resolve_concrete_terminal(terminal, workspace, claude_path, model_id);
    }
    if let Some(xdg_terminal_path) = find_executable(XDG_TERMINAL_EXECUTABLE) {
        return Ok(CommandSpec {
            program: xdg_terminal_path,
            arguments: claude_arguments(claude_path, model_id),
        });
    }

    let fallback_terminal = concrete_terminals()
        .into_iter()
        .find(|candidate| candidate.executable().and_then(find_executable).is_some())
        .ok_or(LauncherError::TerminalNotFound)?;
    resolve_concrete_terminal(fallback_terminal, workspace, claude_path, model_id)
}

fn resolve_concrete_terminal(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
) -> Result<CommandSpec, LauncherError> {
    let executable_name = terminal
        .executable()
        .ok_or(LauncherError::TerminalNotFound)?;
    let terminal_path = find_executable(executable_name).ok_or(LauncherError::TerminalNotFound)?;
    Ok(command_spec_for_terminal(
        terminal,
        terminal_path,
        workspace,
        claude_path,
        model_id,
    ))
}

fn command_spec_for_terminal(
    terminal: TerminalKind,
    terminal_path: PathBuf,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
) -> CommandSpec {
    let workspace_value = workspace.to_string_lossy();
    let claude_value = claude_path.to_string_lossy();
    let public_model = open_code_go_public_model_id(model_id);
    let arguments = match terminal {
        TerminalKind::Ghostty => ghostty_arguments(&workspace_value, &claude_value, &public_model),
        TerminalKind::GnomeTerminal => {
            gnome_arguments(&workspace_value, &claude_value, &public_model)
        }
        TerminalKind::Konsole => konsole_arguments(&workspace_value, &claude_value, &public_model),
        TerminalKind::Kitty => kitty_arguments(&workspace_value, &claude_value, &public_model),
        TerminalKind::Alacritty => {
            alacritty_arguments(&workspace_value, &claude_value, &public_model)
        }
        TerminalKind::SystemDefault => claude_arguments(claude_path, model_id),
    };
    CommandSpec {
        program: terminal_path,
        arguments,
    }
}

fn ensure_terminal_stayed_open(child: &mut std::process::Child) -> Result<(), LauncherError> {
    thread::sleep(TERMINAL_START_GRACE);
    match child.try_wait() {
        Ok(Some(_status)) => Err(LauncherError::TerminalExited),
        Ok(None) => Ok(()),
        Err(source) => Err(LauncherError::Spawn(source)),
    }
}

fn ghostty_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        GHOSTTY_OWN_WINDOW.to_owned(),
        format!("{WORKING_DIRECTORY_ARGUMENT}={workspace}"),
        EXECUTE_ARGUMENT.to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

fn gnome_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        WORKING_DIRECTORY_ARGUMENT.to_owned(),
        workspace.to_owned(),
        "--".to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

fn konsole_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        "--workdir".to_owned(),
        workspace.to_owned(),
        EXECUTE_ARGUMENT.to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

fn kitty_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        "--directory".to_owned(),
        workspace.to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

fn alacritty_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        WORKING_DIRECTORY_ARGUMENT.to_owned(),
        workspace.to_owned(),
        EXECUTE_ARGUMENT.to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

fn claude_arguments(claude_path: &Path, model_id: &str) -> Vec<String> {
    vec![
        claude_path.to_string_lossy().into_owned(),
        MODEL_ARGUMENT.to_owned(),
        open_code_go_public_model_id(model_id),
    ]
}

/// Creates a fresh fallback session ID for requests missing Claude's session header.
pub(crate) fn new_launch_session_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
#[path = "command_test.rs"]
mod command_test;
