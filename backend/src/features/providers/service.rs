use std::{collections::HashMap, sync::Arc};

use super::{Provider, ProviderError, ProviderId};

/// Resolves provider implementations by stable identifier.
#[derive(Clone)]
pub struct ProviderRegistry {
    providers: Arc<HashMap<ProviderId, Arc<dyn Provider>>>,
}

impl ProviderRegistry {
    /// Builds a registry and lets the last duplicate identifier win.
    pub fn new(providers: Vec<Arc<dyn Provider>>) -> Self {
        let providers = providers
            .into_iter()
            .map(|provider| (provider.descriptor().id.clone(), provider))
            .collect();
        Self {
            providers: Arc::new(providers),
        }
    }

    /// Returns a registered provider implementation.
    pub fn get(&self, provider_id: &ProviderId) -> Result<Arc<dyn Provider>, ProviderError> {
        self.providers
            .get(provider_id)
            .cloned()
            .ok_or_else(|| ProviderError::NotRegistered(provider_id.clone()))
    }
}

#[cfg(test)]
#[path = "service_test.rs"]
mod service_test;
