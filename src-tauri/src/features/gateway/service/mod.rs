//! The gateway process and the app side that owns it.

mod catalog;
mod host;
mod process;

pub(crate) use catalog::published_catalog;
pub(crate) use host::{gateway_executable, GatewayHost};
pub(crate) use process::GatewayProcess;
