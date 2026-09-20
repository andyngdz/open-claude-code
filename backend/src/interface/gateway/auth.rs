use axum::http::HeaderMap;
use secrecy::{ExposeSecret, SecretString};

use crate::constants::{API_KEY_HEADER, AUTHORIZATION_HEADER, BEARER_PREFIX};

/// Returns whether the request carries the gateway token Claude Code was launched with.
pub(super) fn is_gateway_authorized(headers: &HeaderMap, expected: &SecretString) -> bool {
    let expected = expected.expose_secret();
    credential_matches(headers, AUTHORIZATION_HEADER, BEARER_PREFIX, expected)
        || credential_matches(headers, API_KEY_HEADER, "", expected)
}

/// Returns whether the request carries the control token only the app holds.
pub(super) fn is_control_authorized(headers: &HeaderMap, expected: &SecretString) -> bool {
    let expected = expected.expose_secret();
    credential_matches(headers, AUTHORIZATION_HEADER, BEARER_PREFIX, expected)
}

/// Compares one header against the expected credential behind a required prefix.
///
/// A credential sent without its prefix does not match, so a route reads only
/// the header it documents. An empty prefix compares the raw header value.
fn credential_matches(headers: &HeaderMap, name: &str, prefix: &str, expected: &str) -> bool {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix(prefix))
        .is_some_and(|value| value == expected)
}

#[cfg(test)]
#[path = "auth_test.rs"]
mod auth_test;
