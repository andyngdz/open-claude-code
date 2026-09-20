use open_claude_code_backend::{ModelCatalogEntry, OpenCodeGoBackend};

use crate::features::settings::AppSettings;

/// Returns the catalog the app publishes to Claude Code.
///
/// A settings file that never completed a discovery falls back to the provider
/// catalog, so a fresh install still shows the models it can run.
pub(crate) fn published_catalog(settings: &AppSettings) -> Vec<ModelCatalogEntry> {
    if settings.cached_models.is_empty() {
        return OpenCodeGoBackend::fallback_catalog();
    }
    settings.cached_models.clone()
}

#[cfg(test)]
#[path = "catalog_test.rs"]
mod catalog_test;
