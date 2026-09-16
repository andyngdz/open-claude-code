//! Axum runtime and routes for the local Claude-compatible gateway.

mod handlers;
mod response;
mod routes;
mod runtime;

pub(crate) use runtime::start_gateway;
pub use runtime::GatewayRuntime;
