//! End-to-end check of the gateway process the desktop app spawns.
//!
//! The whole design lives in this file: one line on stdout tells the app where
//! the gateway listens and which two credentials it accepts, a second token is
//! refused on the control routes, and closing the app's end of stdin stops the
//! process. The gateway opens no window, so this runs headless.

use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

/// The binary cargo built for this test run, spawned as the gateway.
const GATEWAY_EXECUTABLE: &str = env!("CARGO_BIN_EXE_open-claude-code");

/// How long the process may take to report its address before the test fails.
const HANDSHAKE_BUDGET: Duration = Duration::from_secs(30);

/// How long the process may take to exit after its stdin closes.
const EXIT_BUDGET: Duration = Duration::from_secs(20);

#[test]
fn the_gateway_reports_its_address_serves_control_and_exits_with_its_app() {
    let config_home =
        std::env::temp_dir().join(format!("open-claude-code-gateway-{}", std::process::id()));
    std::fs::create_dir_all(&config_home).expect("the test config directory should be created");
    let mut gateway = spawn_gateway(&config_home);
    let handshake = read_handshake(&mut gateway);
    let base_url = handshake["baseUrl"]
        .as_str()
        .expect("the gateway should report a string address");
    let control_token = handshake["controlToken"]
        .as_str()
        .expect("the gateway should report a control token");
    let session_token = handshake["token"]
        .as_str()
        .expect("the gateway should report a gateway token");

    assert_eq!(
        status_of(base_url, &get("/control/state", control_token),),
        "HTTP/1.1 200 OK"
    );
    assert_eq!(
        status_of(base_url, &get("/control/state", session_token)),
        "HTTP/1.1 401 Unauthorized",
        "the token handed to Claude Code should not reach the control routes"
    );
    assert_eq!(
        status_of(base_url, &publish_gateway(control_token)),
        "HTTP/1.1 204 No Content"
    );

    drop(gateway.stdin.take());
    let status = wait_for_exit(&mut gateway);
    std::fs::remove_dir_all(&config_home).expect("the test config directory should be removed");

    assert!(
        status.success(),
        "the gateway should exit cleanly once its app is gone, exited with {status}"
    );
}

/// Spawns the app binary at the subcommand the app uses, with its own config.
fn spawn_gateway(config_home: &PathBuf) -> Child {
    Command::new(GATEWAY_EXECUTABLE)
        .arg("gateway")
        .env("XDG_CONFIG_HOME", config_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("the gateway process should spawn")
}

/// Reads the one line the process reports, failing the test if none arrives.
///
/// The read runs on its own thread because a process that starts but never
/// speaks would block a bare read forever instead of failing the test.
fn read_handshake(gateway: &mut Child) -> serde_json::Value {
    let stdout = gateway
        .stdout
        .take()
        .expect("the gateway should be spawned with a readable stdout");
    let (line_sender, line_receiver) = mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut line = String::new();
        let read = BufReader::new(stdout).read_line(&mut line);
        line_sender.send((read, line)).ok();
    });
    let (read, line) = line_receiver
        .recv_timeout(HANDSHAKE_BUDGET)
        .expect("the gateway should report its address before the budget runs out");
    reader.join().ok();

    assert!(
        read.expect("the handshake line should be readable").gt(&0),
        "the gateway exited without reporting an address"
    );
    serde_json::from_str(line.trim_end()).expect("the handshake line should be JSON")
}

/// Sends one request to the gateway and returns the status line it answered.
fn status_of(base_url: &str, request: &str) -> String {
    let port = base_url
        .rsplit(':')
        .next()
        .and_then(|port| port.parse::<u16>().ok())
        .expect("the reported address should end in a port");
    let mut socket =
        TcpStream::connect(("127.0.0.1", port)).expect("the gateway should accept a connection");
    socket
        .write_all(request.as_bytes())
        .expect("the request should reach the gateway");
    let mut response = String::new();
    socket
        .read_to_string(&mut response)
        .expect("the gateway should answer with a readable response");
    response
        .lines()
        .next()
        .expect("the response should open with a status line")
        .to_owned()
}

/// Builds one authorized GET for a control route.
fn get(path: &str, token: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {token}\r\nConnection: close\r\n\r\n"
    )
}

/// Builds the publish request the app sends while starting.
fn publish_gateway(control_token: &str) -> String {
    let payload = serde_json::json!({
        "catalog": [],
        "customModels": [],
        "sessionId": "gateway-process-test",
    })
    .to_string();
    format!(
        "PUT /control/gateway HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {control_token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    )
}

/// Waits for the process to exit, failing the test if it does not.
fn wait_for_exit(gateway: &mut Child) -> std::process::ExitStatus {
    let deadline = Instant::now() + EXIT_BUDGET;
    while Instant::now() < deadline {
        if let Some(status) = gateway.try_wait().expect("the gateway should be waitable") {
            return status;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    gateway.kill().ok();
    panic!("the gateway should exit after the app closes its stdin");
}
