use std::{path::PathBuf, process::Stdio, time::Duration};

use open_claude_code_backend::ControlClient;
use secrecy::SecretString;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    time::timeout,
};

use crate::features::gateway::{GatewayChildError, GatewayHandshake};

/// Hidden subcommand that runs the app binary as the gateway process.
const GATEWAY_SUBCOMMAND: &str = "gateway";

/// How long the app waits for a spawned process to report its address.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

/// How long the app waits for the gateway to stop before killing it.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(6);

/// Reports whether the gateway was already serving when the app asked for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GatewayStart {
    /// This call spawned the process.
    Started,
    /// The process an earlier call spawned is still serving.
    AlreadyServing,
}

/// Owns the gateway process and the address and credentials it reported.
pub(crate) struct GatewayHost {
    executable: PathBuf,
    running: Option<RunningGateway>,
}

/// One live gateway process and everything it reported.
struct RunningGateway {
    child: Child,
    handshake: GatewayHandshake,
    client: ControlClient,
}

impl GatewayHost {
    /// Owns the gateway process at one executable path.
    pub(crate) fn new(executable: PathBuf) -> Self {
        Self {
            executable,
            running: None,
        }
    }

    /// Returns the gateway, spawning it when none is serving.
    ///
    /// A process that exited without the app asking is replaced here, so a
    /// crashed gateway costs one respawn rather than a dead session.
    pub(crate) async fn ensure_running(&mut self) -> Result<GatewayStart, GatewayChildError> {
        if is_serving(&mut self.running) {
            return Ok(GatewayStart::AlreadyServing);
        }
        self.running = Some(self.spawn_gateway().await?);
        Ok(GatewayStart::Started)
    }

    /// Returns the client that drives the running gateway.
    pub(crate) fn client(&self) -> Result<&ControlClient, GatewayChildError> {
        Ok(&self.running()?.client)
    }

    /// Returns the loopback URL passed only to launched Claude Code processes.
    pub(crate) fn base_url(&self) -> Result<&str, GatewayChildError> {
        Ok(self.running()?.handshake.base_url.as_str())
    }

    /// Returns the gateway credential passed only to launched processes.
    pub(crate) fn local_token(&self) -> Result<SecretString, GatewayChildError> {
        Ok(SecretString::from(self.running()?.handshake.token.clone()))
    }

    /// Stops the gateway, waiting for it to exit before killing it.
    pub(crate) async fn shutdown(&mut self) -> Result<(), GatewayChildError> {
        let Some(mut running) = self.running.take() else {
            return Ok(());
        };
        // Closing the app's end of stdin is the stop signal: the process reads
        // EOF and stops its own gateway. The wait below covers that path.
        drop(running.child.stdin.take());
        match timeout(SHUTDOWN_TIMEOUT, running.child.wait()).await {
            Ok(Ok(_status)) => Ok(()),
            Ok(Err(source)) => Err(GatewayChildError::Reap(source)),
            Err(_elapsed) => running.child.kill().await.map_err(GatewayChildError::Reap),
        }
    }

    /// Returns the running gateway, or the error naming why there is none.
    fn running(&self) -> Result<&RunningGateway, GatewayChildError> {
        self.running.as_ref().ok_or(GatewayChildError::NotRunning)
    }

    /// Spawns the process and reads the one line it reports.
    ///
    /// A child that never reports is dropped on the way out, which kills it:
    /// `spawn_child` asked for that, so a failed start leaves no stray process.
    async fn spawn_gateway(&self) -> Result<RunningGateway, GatewayChildError> {
        let mut child = self.spawn_child()?;
        let handshake = read_handshake(&mut child).await?;
        build_running(child, handshake)
    }

    /// Starts one gateway process on the pipes the app owns.
    ///
    /// Nothing reads the child's stderr, so it goes to the null device rather
    /// than filling an unread pipe. `kill_on_drop` is the net under a panic:
    /// the process dies with this one even if `shutdown` never runs.
    fn spawn_child(&self) -> Result<Child, GatewayChildError> {
        Command::new(&self.executable)
            .arg(GATEWAY_SUBCOMMAND)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(GatewayChildError::Spawn)
    }
}

/// Returns the path of the running binary, which is also the gateway process.
pub(crate) fn gateway_executable() -> Result<PathBuf, GatewayChildError> {
    std::env::current_exe().map_err(GatewayChildError::Executable)
}

/// Reports whether a spawned gateway is still alive, forgetting it when it is not.
fn is_serving(running: &mut Option<RunningGateway>) -> bool {
    let Some(entry) = running.as_mut() else {
        return false;
    };
    // A wait that reports no status is the only answer that means "still there".
    // An exited process and a failed probe read the same way: not serving.
    if entry.child.try_wait().is_ok_and(|status| status.is_none()) {
        return true;
    }
    *running = None;
    false
}

/// Reads the single line a freshly spawned process reports.
async fn read_handshake(child: &mut Child) -> Result<GatewayHandshake, GatewayChildError> {
    let Some(stdout) = child.stdout.take() else {
        return Err(GatewayChildError::StartFailed);
    };
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    match timeout(HANDSHAKE_TIMEOUT, reader.read_line(&mut line)).await {
        Ok(Ok(read)) if read > 0 => GatewayHandshake::parse(line.trim_end()),
        // The process exited without saying anything.
        Ok(Ok(_read)) => Err(GatewayChildError::StartFailed),
        // Reading the pipe itself failed.
        Ok(Err(source)) => Err(GatewayChildError::Unreadable(source)),
        // The process is still starting after the whole budget.
        Err(_elapsed) => Err(GatewayChildError::StartFailed),
    }
}

/// Builds the running gateway around the address the process reported.
fn build_running(
    child: Child,
    handshake: GatewayHandshake,
) -> Result<RunningGateway, GatewayChildError> {
    let client = ControlClient::new(
        handshake.base_url.clone(),
        SecretString::from(handshake.control_token.clone()),
    )?;
    Ok(RunningGateway {
        child,
        handshake,
        client,
    })
}

#[cfg(test)]
#[path = "host_test.rs"]
mod host_test;
