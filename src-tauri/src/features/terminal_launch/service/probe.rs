use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use open_claude_code_backend::GATEWAY_HEALTH_PATH;

/// How long the CLI waits for the running gateway to answer the probe.
const PROBE_TIMEOUT: Duration = Duration::from_millis(300);

/// Status the gateway health route answers with.
const HEALTH_STATUS_CODE: u16 = 204;

/// Returns whether a published handshake still points at a living gateway.
///
/// The gateway dies with the app, so a handshake left behind by an app that was
/// killed names a port nothing answers on. Connecting alone is not proof: the
/// port can have been taken by another process, and only the health route
/// answers the way the gateway does.
pub(crate) fn gateway_is_serving(base_url: &str) -> bool {
    let Some((authority, address)) = gateway_target(base_url) else {
        return false;
    };
    let Ok(mut socket) = TcpStream::connect_timeout(&address, PROBE_TIMEOUT) else {
        return false;
    };
    if socket.set_read_timeout(Some(PROBE_TIMEOUT)).is_err() {
        return false;
    }
    matches!(
        health_status_code(&mut socket, authority),
        Some(code) if code == HEALTH_STATUS_CODE
    )
}

/// Splits a published loopback URL into the authority and the address to probe.
fn gateway_target(base_url: &str) -> Option<(&str, SocketAddr)> {
    let authority = base_url.strip_prefix("http://")?;
    let address = authority.to_socket_addrs().ok()?.next()?;
    Some((authority, address))
}

/// Asks the health route, which needs no credential, and reads its status code.
fn health_status_code(socket: &mut TcpStream, authority: &str) -> Option<u16> {
    let request = format!(
        "HEAD {GATEWAY_HEALTH_PATH} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n\r\n"
    );
    socket.write_all(request.as_bytes()).ok()?;
    let mut status_line = String::new();
    BufReader::new(socket).read_line(&mut status_line).ok()?;
    status_line.split_whitespace().nth(1)?.parse().ok()
}

#[cfg(test)]
#[path = "probe_test.rs"]
mod probe_test;
