//! HTTP interface exposed to Claude Code.

mod errors;
mod gateway;

pub(crate) use errors::GatewayError;
pub(crate) use gateway::start_gateway;
pub use gateway::GatewayRuntime;
