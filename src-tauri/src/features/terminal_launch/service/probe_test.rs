use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    thread::{self, JoinHandle},
};

use super::{gateway_is_serving, gateway_target};

/// Ends the status line a test server writes back.
const CRLF: &str = "\r\n";

/// Starts a one-shot server that answers the next connection with `status_code`.
///
/// The handle comes back so the test joins it: nothing should outlive the test
/// that started it.
fn serving(status_code: u16, reason: &'static str) -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("the test listener should bind");
    let address = listener
        .local_addr()
        .expect("the test listener should report its address");
    let answering = thread::spawn(move || {
        let Ok((mut socket, _peer)) = listener.accept() else {
            return;
        };
        let mut request = [0; 256];
        // Swallow the request, so the status line cannot race the client's write.
        if socket.read(&mut request).is_err() {
            return;
        }
        // The probe may close before the line lands, which is not this test's concern.
        write!(socket, "HTTP/1.1 {status_code} {reason}{CRLF}{CRLF}").ok();
    });
    (address, answering)
}

fn base_url(address: SocketAddr) -> String {
    format!("http://{address}")
}

#[test]
fn a_gateway_answering_the_health_route_is_serving() {
    let (address, answering) = serving(204, "No Content");
    assert!(gateway_is_serving(&base_url(address)));
    answering.join().ok();
}

#[test]
fn a_port_that_answers_with_another_status_is_not_serving() {
    let (address, answering) = serving(200, "OK");
    assert!(
        !gateway_is_serving(&base_url(address)),
        "connecting is not proof: another process can already hold the port"
    );
    answering.join().ok();
}

#[test]
fn a_port_nothing_answers_on_is_not_serving() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("the test listener should bind");
    let address = listener
        .local_addr()
        .expect("the test listener should report its address");
    drop(listener);
    assert!(!gateway_is_serving(&base_url(address)));
}

#[test]
fn a_url_without_a_loopback_authority_is_not_serving() {
    assert!(!gateway_is_serving("https://example.com"));
    assert!(!gateway_is_serving("not a url"));
}

#[test]
fn a_target_splits_into_the_authority_and_a_socket() {
    let (authority, address) = gateway_target("http://127.0.0.1:8123").expect("a loopback URL");
    assert_eq!(authority, "127.0.0.1:8123");
    assert_eq!(address.port(), 8123);
}
