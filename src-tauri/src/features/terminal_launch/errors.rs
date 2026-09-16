use thiserror::Error;

/// Reports a CLI launch failure that should be printed and not opened in a window.
#[derive(Debug, Error)]
pub(crate) enum CliError {
    /// The tray app has not published a handshake.
    #[error("Open Claude Code is not running. Open the app, then try again.")]
    NotRunning,
    /// The handshake file exists but cannot be used.
    #[error("Open Claude Code could not be reached. Quit it from the tray and open it again.")]
    Unreachable,
    /// A script asked to launch without choosing a model.
    #[error("Pass --model. Example: open-claude-code launch --model qwen3.8-max")]
    ModelRequired,
    /// The chosen id is not in the published catalog.
    #[error("Unknown model. Run open-claude-code launch to see the list.")]
    UnknownModel,
    /// Claude Code is not installed.
    #[error("Claude Code was not found on PATH. Install it, then try again.")]
    ClaudeNotFound,
    /// Replacing this process with Claude Code failed.
    #[error("Claude Code could not be started. Try again.")]
    Spawn(#[source] std::io::Error),
}
