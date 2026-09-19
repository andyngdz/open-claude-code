use std::{fs, path::Path, process::Command, thread, time::Duration};

use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use super::discovery::is_terminal_launchable;
use super::proxy::{apply_proxy_env, claude_executable, claude_proxy_env, ClaudeProxyEnv};
use super::terminal_args::CommandSpec;

#[cfg(target_os = "macos")]
use super::macos_launch::resolve_macos_command;

#[cfg(not(target_os = "macos"))]
use super::discovery::{
    concrete_terminals, find_executable, is_installed, XDG_TERMINAL_EXECUTABLE,
};
#[cfg(not(target_os = "macos"))]
use super::terminal_args::{claude_arguments, command_spec_for_terminal};

use crate::features::{
    errors::LauncherError,
    launcher::{LaunchReceipt, ProcessRegistry, TerminalKind},
    settings::{is_workspace_directory, ModelAliasMapping},
};

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
    let launch_terminal = effective_terminal(terminal);
    let claude_path = claude_executable()?;
    let proxy_env = claude_proxy_env(proxy_base_url, proxy_token.expose_secret(), aliases);
    let command_spec = resolve_command_spec(
        launch_terminal,
        workspace,
        &claude_path,
        model_id,
        &proxy_env,
    )?;
    let mut child = spawn_terminal(&command_spec, workspace, &proxy_env)?;
    if should_check_terminal_liveness(launch_terminal) {
        ensure_terminal_stayed_open(&mut child)?;
    }
    let process_id = child.id();
    registry.register(child)?;

    Ok(LaunchReceipt {
        process_id,
        terminal: launch_terminal,
        workspace: workspace.to_path_buf(),
        model_id: model_id.to_owned(),
    })
}

/// Falls back to System default when the saved terminal is no longer installed.
pub(crate) fn effective_terminal(terminal: TerminalKind) -> TerminalKind {
    if is_terminal_launchable(terminal) {
        return terminal;
    }
    TerminalKind::SystemDefault
}

fn spawn_terminal(
    command_spec: &CommandSpec,
    workspace: &Path,
    proxy_env: &ClaudeProxyEnv,
) -> Result<std::process::Child, LauncherError> {
    let mut command = Command::new(&command_spec.program);
    command.args(&command_spec.arguments).current_dir(workspace);
    apply_proxy_env(&mut command, proxy_env);
    match command.spawn() {
        Ok(child) => Ok(child),
        Err(source) => {
            if let Some(cleanup_path) = &command_spec.cleanup_path {
                fs::remove_file(cleanup_path).ok();
            }
            Err(LauncherError::Spawn(source))
        }
    }
}

#[cfg(target_os = "macos")]
fn resolve_command_spec(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<CommandSpec, LauncherError> {
    resolve_macos_command(terminal, workspace, claude_path, model_id, proxy_env)
}

#[cfg(not(target_os = "macos"))]
fn resolve_command_spec(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    _proxy_env: &ClaudeProxyEnv,
) -> Result<CommandSpec, LauncherError> {
    resolve_linux_command(terminal, workspace, claude_path, model_id)
}

#[cfg(not(target_os = "macos"))]
fn resolve_linux_command(
    terminal: TerminalKind,
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
) -> Result<CommandSpec, LauncherError> {
    if terminal == TerminalKind::SystemDefault {
        if let Some(xdg_terminal_path) = find_executable(XDG_TERMINAL_EXECUTABLE) {
            return Ok(CommandSpec {
                program: xdg_terminal_path,
                arguments: claude_arguments(claude_path, model_id),
                cleanup_path: None,
            });
        }
        let fallback_terminal = concrete_terminals()
            .iter()
            .copied()
            .find(|kind| is_installed(*kind))
            .ok_or(LauncherError::TerminalNotFound)?;
        return resolve_concrete_terminal(fallback_terminal, workspace, claude_path, model_id);
    }
    resolve_concrete_terminal(terminal, workspace, claude_path, model_id)
}

#[cfg(not(target_os = "macos"))]
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

fn should_check_terminal_liveness(terminal: TerminalKind) -> bool {
    if cfg!(target_os = "macos") {
        return false;
    }
    terminal != TerminalKind::SystemDefault
}

fn ensure_terminal_stayed_open(child: &mut std::process::Child) -> Result<(), LauncherError> {
    thread::sleep(TERMINAL_START_GRACE);
    match child.try_wait() {
        Ok(Some(_status)) => Err(LauncherError::TerminalExited),
        Ok(None) => Ok(()),
        Err(source) => Err(LauncherError::Spawn(source)),
    }
}

/// Creates a fresh fallback session ID for requests missing Claude's session header.
pub(crate) fn new_launch_session_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
#[path = "command_test.rs"]
mod command_test;
