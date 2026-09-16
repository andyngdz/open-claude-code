//! Gateway model discovery and request resolution.

mod domain;
mod errors;
mod service;

pub(crate) use domain::{GatewayConfiguration, GatewayDiscoveryModel, ResolvedProviderRequest};
pub(crate) use errors::GatewayRequestError;
pub(crate) use service::{discovery_models, public_model_id, resolve_provider_request};
