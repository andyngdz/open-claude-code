use thiserror::Error;

use open_claude_code_backend::ControlError;

use crate::features::{errors::LauncherError, gateway::GatewayChildError};

/// Reports a failure the dashboard can show to the user.
#[derive(Debug, Error)]
pub(crate) enum SessionError {
    /// Settings could not be loaded or saved.
    #[error("Settings could not be saved. Try again.")]
    Settings,
    /// Another desktop process already owns the local gateway handshake.
    #[error("Open Claude Code is already running.")]
    AlreadyRunning,
    /// A provider or gateway operation failed with a ready-to-show message.
    #[error("{0}")]
    Operation(String),
    /// The selected path is not a directory.
    #[error("Choose a workspace directory.")]
    InvalidWorkspace,
    /// Claude Code is not installed.
    #[error("Claude Code was not found on PATH. Install it, then try again.")]
    ClaudeNotFound,
    /// The selected terminal is not installed.
    #[error("The selected terminal was not found. Choose an installed terminal.")]
    TerminalNotFound,
    /// The operating system rejected the spawn.
    #[error("The terminal could not be opened. Try again.")]
    Spawn,
    /// The terminal window closed before Claude Code stayed open.
    #[error("The terminal closed immediately. Check that Claude Code can start, then try again.")]
    TerminalExited,
    /// A previous process could not be tracked.
    #[error("A previous terminal could not be tracked. Try again.")]
    RegistryUnavailable,
    /// The chosen model is not in the catalog or custom list.
    #[error("Choose a model from the OpenCode Go catalog.")]
    UnknownModel,
    /// Launch was requested before a credential was saved.
    #[error("Save an OpenCode Go API key before launching.")]
    NotConnected,
}

impl From<ControlError> for SessionError {
    fn from(source: ControlError) -> Self {
        Self::Operation(source.to_string())
    }
}

impl From<GatewayChildError> for SessionError {
    fn from(source: GatewayChildError) -> Self {
        Self::Operation(source.to_string())
    }
}

impl From<LauncherError> for SessionError {
    fn from(source: LauncherError) -> Self {
        match source {
            LauncherError::InvalidWorkspace => Self::InvalidWorkspace,
            LauncherError::ClaudeNotFound => Self::ClaudeNotFound,
            LauncherError::TerminalNotFound => Self::TerminalNotFound,
            LauncherError::Spawn(_source) => Self::Spawn,
            LauncherError::TerminalExited => Self::TerminalExited,
            LauncherError::RegistryUnavailable => Self::RegistryUnavailable,
        }
    }
}
