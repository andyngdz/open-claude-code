use thiserror::Error;

/// Reports failures while starting or stopping the local HTTP gateway.
#[derive(Debug, Error)]
pub enum GatewayError {
    /// The loopback listener could not bind.
    #[error("the local gateway could not bind to a loopback port")]
    Bind(#[source] std::io::Error),
    /// The bound listener did not expose its local address.
    #[error("the local gateway address is unavailable")]
    LocalAddress(#[source] std::io::Error),
    /// The gateway lifecycle lock was poisoned.
    #[error("the local gateway lifecycle is unavailable")]
    LifecycleUnavailable,
    /// The gateway task could not be joined.
    #[error("the local gateway task could not be joined")]
    Join(#[source] tokio::task::JoinError),
    /// The HTTP server stopped with an I/O failure.
    #[error("the local gateway stopped unexpectedly")]
    Serve(#[source] std::io::Error),
    /// Graceful shutdown exceeded its bounded wait.
    #[error("the local gateway did not stop before the timeout")]
    StopTimeout,
}

/// Reports failures while the desktop app drives the gateway it spawned.
#[derive(Debug, Error)]
pub enum ControlError {
    /// The control request never reached a listening gateway.
    #[error("the local gateway could not be reached")]
    Unreachable(#[source] reqwest::Error),
    /// The control route refused the request with a message ready to show.
    #[error("{0}")]
    Rejected(String),
    /// The control response carried a body the client could not read.
    #[error("the local gateway returned an unreadable response")]
    Unexpected(#[source] reqwest::Error),
}
