//! HTTP interface exposed to Claude Code.

mod errors;
mod gateway;

pub use errors::ControlError;
pub(crate) use errors::GatewayError;
pub(crate) use gateway::{start_gateway, GatewayPublisher};
pub use gateway::{ControlClient, ControlState, GatewayRuntime};
