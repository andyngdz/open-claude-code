//! Axum runtime and routes for the local Claude-compatible gateway.

mod auth;
mod client;
mod control;
mod dto;
mod handlers;
mod response;
mod routes;
mod runtime;

pub use client::ControlClient;
pub use dto::ControlState;
pub use runtime::GatewayRuntime;
pub(crate) use runtime::{start_gateway, GatewayPublisher};
