use super::GatewayHandshake;
use crate::features::gateway::GatewayChildError;

/// The one line the app and the gateway process agree on.
fn handshake() -> GatewayHandshake {
    GatewayHandshake {
        base_url: "http://127.0.0.1:43210".to_owned(),
        token: "gateway-token".to_owned(),
        control_token: "control-token".to_owned(),
    }
}

#[test]
fn the_reported_line_carries_the_address_and_both_credentials() {
    let line = handshake().to_line().expect("a handshake should render");

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&line).expect("the line should be JSON"),
        serde_json::json!({
            "baseUrl": "http://127.0.0.1:43210",
            "token": "gateway-token",
            "controlToken": "control-token",
        })
    );
}

#[test]
fn a_reported_line_reads_back_as_the_handshake_that_wrote_it() {
    let line = handshake().to_line().expect("a handshake should render");

    assert_eq!(
        GatewayHandshake::parse(&line).expect("the rendered line should parse"),
        handshake()
    );
}

#[test]
fn a_line_without_an_address_is_reported_as_unreadable() {
    let error = GatewayHandshake::parse(r#"{"token":"a","controlToken":"b"}"#)
        .expect_err("a line without a base url should not parse");

    assert!(matches!(error, GatewayChildError::Handshake(_)));
}

#[test]
fn a_line_that_is_not_json_is_reported_as_unreadable() {
    let error =
        GatewayHandshake::parse("gateway listening").expect_err("a sentence should not parse");

    assert!(matches!(error, GatewayChildError::Handshake(_)));
}
