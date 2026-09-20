use std::{path::PathBuf, process::Stdio};

use open_claude_code_backend::ControlClient;
use secrecy::{ExposeSecret, SecretString};
use tokio::process::{Child, Command};

use super::{GatewayHost, GatewayStart, RunningGateway};
use crate::features::gateway::{GatewayChildError, GatewayHandshake};

/// Path no process is installed at, so a spawn attempt is guaranteed to fail.
const MISSING_EXECUTABLE: &str = "/nonexistent/open-claude-code";

/// Builds the entry the host holds around one child process.
fn running(child: Child) -> RunningGateway {
    RunningGateway {
        child,
        handshake: GatewayHandshake {
            base_url: "http://127.0.0.1:1".to_owned(),
            token: "gateway-token".to_owned(),
            control_token: "control-token".to_owned(),
        },
        client: ControlClient::new(
            "http://127.0.0.1:1".to_owned(),
            SecretString::from("control-token"),
        )
        .expect("the control client should build in a test"),
    }
}

/// A host that already holds one child process.
fn host_running(child: Child) -> GatewayHost {
    let mut host = GatewayHost::new(PathBuf::from(MISSING_EXECUTABLE));
    host.running = Some(running(child));
    host
}

/// Spawns a process that has already exited by the time it is returned.
async fn finished_process() -> Child {
    let mut child = Command::new(std::env::current_exe().expect("the test binary should resolve"))
        .arg("--list")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("the test binary should spawn");
    child.wait().await.expect("the listing process should exit");
    child
}

/// Spawns a process that lives until its stdin closes, as the gateway does.
#[cfg(unix)]
async fn process_until_stdin_ends() -> Child {
    Command::new("sh")
        .arg("-c")
        .arg("cat > /dev/null")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("the shell should spawn in a test")
}

#[test]
fn a_host_that_never_started_reports_every_lookup_as_not_running() {
    let host = GatewayHost::new(PathBuf::from(MISSING_EXECUTABLE));

    assert!(matches!(
        host.base_url(),
        Err(GatewayChildError::NotRunning)
    ));
    assert!(matches!(
        host.local_token(),
        Err(GatewayChildError::NotRunning)
    ));
    assert!(matches!(host.client(), Err(GatewayChildError::NotRunning)));
}

#[tokio::test]
async fn the_gateway_address_and_credential_come_from_the_reported_line() {
    let host = host_running(finished_process().await);

    assert_eq!(
        host.base_url()
            .expect("the reported address should read back"),
        "http://127.0.0.1:1"
    );
    assert_eq!(
        host.local_token()
            .expect("the reported token should read back")
            .expose_secret(),
        "gateway-token"
    );
}

#[tokio::test]
async fn stopping_a_host_that_never_started_is_not_a_failure() {
    let mut host = GatewayHost::new(PathBuf::from(MISSING_EXECUTABLE));

    assert!(host.shutdown().await.is_ok());
}

#[tokio::test]
async fn a_process_that_already_exited_is_replaced_instead_of_reused() {
    let mut host = host_running(finished_process().await);

    let outcome = host.ensure_running().await;

    // The replacement attempt fails because the executable is missing, which is
    // how this test sees that the exited process was not treated as serving.
    assert!(matches!(outcome, Err(GatewayChildError::Spawn(_))));
    assert!(host.running.is_none());
}

#[cfg(unix)]
#[tokio::test]
async fn a_process_that_is_still_serving_is_kept_and_stopped_by_closing_its_stdin() {
    let mut host = host_running(process_until_stdin_ends().await);

    assert_eq!(
        host.ensure_running().await.ok(),
        Some(GatewayStart::AlreadyServing)
    );
    assert!(host.shutdown().await.is_ok());
    assert!(host.running.is_none());
}
