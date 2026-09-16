use crate::features::providers::{ModelCatalogEntry, ProviderId};

/// Holds the active provider and models published by gateway discovery.
#[derive(Clone, Debug)]
pub(crate) struct GatewayConfiguration {
    pub(crate) provider_id: ProviderId,
    pub(crate) catalog: Vec<ModelCatalogEntry>,
    pub(crate) custom_models: Vec<String>,
}

/// Carries a validated provider target and normalized request body.
#[derive(Debug)]
pub(crate) struct ResolvedProviderRequest {
    pub(crate) provider_id: ProviderId,
    pub(crate) body: Vec<u8>,
}

/// Describes one model published through Claude Code gateway discovery.
#[derive(Debug)]
pub(crate) struct GatewayDiscoveryModel {
    pub(crate) id: String,
    pub(crate) display_name: String,
    pub(crate) description: &'static str,
}
