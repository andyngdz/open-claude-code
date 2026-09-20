use axum::http::{HeaderMap, HeaderValue};
use secrecy::SecretString;

use super::{is_control_authorized, is_gateway_authorized};
use crate::constants::{API_KEY_HEADER, AUTHORIZATION_HEADER};

const GATEWAY_TOKEN: &str = "gateway-token";
const CONTROL_TOKEN: &str = "control-token";

#[test]
fn the_bearer_token_authorizes_a_gateway_request() {
    let headers = bearer_headers(GATEWAY_TOKEN);

    assert!(is_gateway_authorized(&headers, &gateway_token()));
}

#[test]
fn the_api_key_header_authorizes_a_gateway_request() {
    let mut headers = HeaderMap::new();
    headers.insert(API_KEY_HEADER, HeaderValue::from_static(GATEWAY_TOKEN));

    assert!(is_gateway_authorized(&headers, &gateway_token()));
}

#[test]
fn the_control_token_does_not_authorize_a_gateway_request() {
    let headers = bearer_headers(CONTROL_TOKEN);

    assert!(!is_gateway_authorized(&headers, &gateway_token()));
}

#[test]
fn a_missing_or_unreadable_credential_is_rejected() {
    assert!(!is_gateway_authorized(&HeaderMap::new(), &gateway_token()));

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION_HEADER,
        HeaderValue::from_bytes(b"Bearer \xff").expect("a raw byte value should build"),
    );

    assert!(!is_gateway_authorized(&headers, &gateway_token()));
}

#[test]
fn only_the_control_token_authorizes_a_control_request() {
    assert!(is_control_authorized(
        &bearer_headers(CONTROL_TOKEN),
        &control_token()
    ));
    assert!(!is_control_authorized(
        &bearer_headers(GATEWAY_TOKEN),
        &control_token()
    ));
}

#[test]
fn a_prefixless_control_credential_is_rejected() {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION_HEADER,
        HeaderValue::from_static(CONTROL_TOKEN),
    );
    headers.insert(API_KEY_HEADER, HeaderValue::from_static(CONTROL_TOKEN));

    assert!(!is_control_authorized(&headers, &control_token()));
}

fn bearer_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let value = HeaderValue::from_str(&format!("Bearer {token}")).expect("a token should encode");
    headers.insert(AUTHORIZATION_HEADER, value);
    headers
}

fn gateway_token() -> SecretString {
    SecretString::from(GATEWAY_TOKEN.to_owned())
}

fn control_token() -> SecretString {
    SecretString::from(CONTROL_TOKEN.to_owned())
}
