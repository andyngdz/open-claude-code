use reqwest::header::HeaderName;

use super::{should_forward_header, should_return_header, CONTENT_LENGTH_HEADER};
use crate::constants::AUTHORIZATION_HEADER;

#[test]
fn strips_client_credentials_before_forwarding() {
    let authorization = HeaderName::from_static(AUTHORIZATION_HEADER);
    let anthropic_version = HeaderName::from_static("anthropic-version");

    assert!(!should_forward_header(&authorization));
    assert!(should_forward_header(&anthropic_version));
}

#[test]
fn strips_length_from_streaming_response() {
    let content_length = HeaderName::from_static(CONTENT_LENGTH_HEADER);

    assert!(!should_return_header(&content_length));
}
