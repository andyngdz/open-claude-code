//! Provider contracts and registry.

mod domain;
mod errors;
mod service;
mod traits;

pub(crate) use domain::{
    AuthMethod, ProviderBodyStream, ProviderDescriptor, ProviderHeader, ProviderId,
    ProviderProtocol, ProviderRequest, ProviderResponse,
};
pub use domain::{ModelCatalogEntry, ProviderConnectionState};
pub(crate) use errors::ProviderError;
pub(crate) use service::ProviderRegistry;
pub(crate) use traits::Provider;
