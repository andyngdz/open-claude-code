use thiserror::Error;

/// Reports failures while loading or persisting application settings.
#[derive(Debug, Error)]
pub(crate) enum SettingsError {
    /// Another desktop process already owns this application's session files.
    #[error("Open Claude Code is already running")]
    InstanceAlreadyRunning,
    /// The operating system did not provide a config directory.
    #[error("settings directory is unavailable")]
    DirectoryUnavailable,
    /// The settings file could not be read.
    #[error("settings file could not be read")]
    Read(#[source] std::io::Error),
    /// The settings file is not valid JSON.
    #[error("settings file is invalid")]
    Parse(#[source] serde_json::Error),
    /// The settings file could not be serialized.
    #[error("settings could not be serialized")]
    Serialize(#[source] serde_json::Error),
    /// The settings file could not be written.
    #[error("settings could not be written")]
    Write(#[source] std::io::Error),
    /// The application ownership lock could not be acquired.
    #[error("application ownership lock could not be acquired")]
    Lock(#[source] std::io::Error),
}

/// Reports failures while reading or writing the private CLI handshake file.
#[derive(Debug, Error)]
pub(crate) enum RuntimeEndpointError {
    /// The config directory could not be created.
    #[error("settings directory is unavailable")]
    DirectoryUnavailable,
    /// The handshake file could not be read.
    #[error("runtime file could not be read")]
    Read(#[source] std::io::Error),
    /// The handshake file is not the expected JSON.
    #[error("runtime file is invalid")]
    Parse(#[source] serde_json::Error),
    /// The handshake file could not be serialized.
    #[error("runtime file could not be serialized")]
    Serialize(#[source] serde_json::Error),
    /// The handshake file could not be written.
    #[error("runtime file could not be written")]
    Write(#[source] std::io::Error),
}

/// Reports failures while building or spawning a Claude Code terminal process.
#[derive(Debug, Error)]
pub(crate) enum LauncherError {
    /// The selected path is not a directory.
    #[error("the selected workspace is not a directory")]
    InvalidWorkspace,
    /// The Claude Code executable is not on PATH.
    #[error("Claude Code is not installed or is not available on PATH")]
    ClaudeNotFound,
    /// The selected terminal executable is not on PATH.
    #[error("the selected terminal is not installed")]
    TerminalNotFound,
    /// The operating system rejected the process spawn.
    #[error("the terminal process could not be started")]
    Spawn(#[source] std::io::Error),
    /// The terminal process exited before Claude Code stayed open.
    #[error("the terminal closed immediately")]
    TerminalExited,
    /// The process registry lock was poisoned.
    #[error("the terminal process registry is unavailable")]
    RegistryUnavailable,
}
