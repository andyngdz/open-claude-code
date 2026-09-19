#[cfg(any(test, not(target_os = "macos")))]
use std::path::Path;
use std::path::PathBuf;

#[cfg(any(test, not(target_os = "macos")))]
use open_claude_code_backend::open_code_go_public_model_id;

#[cfg(any(test, not(target_os = "macos")))]
use crate::features::launcher::TerminalKind;

pub(super) const MODEL_ARGUMENT: &str = crate::constants::MODEL_FLAG;
pub(super) const EXECUTE_ARGUMENT: &str = "-e";
pub(super) const WORKING_DIRECTORY_ARGUMENT: &str = "--working-directory";
pub(super) const DIRECTORY_ARGUMENT: &str = "--directory";

#[cfg(any(test, not(target_os = "macos")))]
pub(super) const GHOSTTY_OWN_WINDOW: &str = "--gtk-single-instance=false";

/// Program and arguments used to spawn a terminal helper process.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct CommandSpec {
    pub(super) program: PathBuf,
    pub(super) arguments: Vec<String>,
}

/// Builds Linux CLI arguments for a concrete terminal binary.
#[cfg(any(test, not(target_os = "macos")))]
pub(super) fn command_spec_for_terminal(
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
        TerminalKind::SystemDefault
        | TerminalKind::AppleTerminal
        | TerminalKind::ITerm2
        | TerminalKind::Warp
        | TerminalKind::WezTerm
        | TerminalKind::Hyper => claude_arguments(claude_path, model_id),
    };
    CommandSpec {
        program: terminal_path,
        arguments,
    }
}

#[cfg(any(test, not(target_os = "macos")))]
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

#[cfg(any(test, not(target_os = "macos")))]
fn gnome_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        "--wait".to_owned(),
        WORKING_DIRECTORY_ARGUMENT.to_owned(),
        workspace.to_owned(),
        "--".to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

#[cfg(any(test, not(target_os = "macos")))]
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

#[cfg(any(test, not(target_os = "macos")))]
fn kitty_arguments(workspace: &str, claude: &str, model: &str) -> Vec<String> {
    vec![
        DIRECTORY_ARGUMENT.to_owned(),
        workspace.to_owned(),
        claude.to_owned(),
        MODEL_ARGUMENT.to_owned(),
        model.to_owned(),
    ]
}

#[cfg(any(test, not(target_os = "macos")))]
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

/// Builds the executable-and-model argument list for xdg-terminal-exec.
#[cfg(any(test, not(target_os = "macos")))]
pub(super) fn claude_arguments(claude_path: &Path, model_id: &str) -> Vec<String> {
    vec![
        claude_path.to_string_lossy().into_owned(),
        MODEL_ARGUMENT.to_owned(),
        open_code_go_public_model_id(model_id),
    ]
}

/// Wraps a value in single quotes for POSIX shells.
pub(super) fn sh_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
#[path = "terminal_args_test.rs"]
mod terminal_args_test;
