use open_claude_code_backend::ControlError;
use thiserror::Error;

/// Reports failures while the app spawns and drives the gateway process.
#[derive(Debug, Error)]
pub(crate) enum GatewayChildError {
    /// The gateway process could not be spawned.
    #[error("The local gateway could not be started. Try again.")]
    Spawn(#[source] std::io::Error),
    /// The running binary could not be located.
    #[error("Open Claude Code could not locate itself. Reopen the app.")]
    Executable(#[source] std::io::Error),
    /// The process exited or stayed silent before reporting its address.
    #[error("The local gateway did not report its address. Try again.")]
    StartFailed,
    /// The reported address could not be read or written.
    #[error("The local gateway reported an unreadable address. Try again.")]
    Handshake(#[source] serde_json::Error),
    /// The pipe a freshly spawned process reports on failed before it spoke.
    #[error("The local gateway could not be read. Try again.")]
    Unreadable(#[source] std::io::Error),
    /// The process did not exit after it was told to stop.
    #[error("The local gateway did not stop cleanly. Reopen the app.")]
    Reap(#[source] std::io::Error),
    /// A control call to the gateway failed.
    #[error("{0}")]
    Control(#[from] ControlError),
    /// No gateway process is running for this call.
    #[error("The local gateway is not running. Try again.")]
    NotRunning,
    /// The settings the gateway process reads could not be loaded.
    #[error("Settings could not be read. Try again.")]
    Settings,
    /// The pipe carrying the app's lifetime to the gateway process failed.
    #[error("The app that started the local gateway is gone. Reopen the app.")]
    ParentPipe(#[source] std::io::Error),
    /// The backend could not start inside the gateway process.
    #[error("{0}")]
    Backend(String),
}
