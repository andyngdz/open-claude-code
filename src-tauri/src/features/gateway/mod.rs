//! Gateway process the desktop app spawns and owns.

mod domain;
mod errors;
mod service;

pub(crate) use domain::GatewayHandshake;
pub(crate) use errors::GatewayChildError;
pub(crate) use service::{gateway_executable, published_catalog, GatewayHost, GatewayProcess};
