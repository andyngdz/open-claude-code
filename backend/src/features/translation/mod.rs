//! Translates Claude-compatible gateway requests to provider protocol payloads.

mod service;

pub(crate) use service::{translate_request, translate_response_stream, UpstreamProtocol};
