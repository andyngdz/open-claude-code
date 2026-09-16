//! Terminal launch that attaches to the running gateway.

mod errors;
mod service;

pub(crate) use errors::CliError;
pub(crate) use service::launch;
