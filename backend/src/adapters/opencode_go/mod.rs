//! OpenCode Go provider implementation.

mod catalog;
mod client;
mod credential;
mod provider;

pub(crate) use provider::{OpenCodeGoProvider, OPENCODE_GO_PROVIDER_ID};
