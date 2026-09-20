use super::published_catalog;
use crate::features::settings::AppSettings;
use open_claude_code_backend::{ModelCatalogEntry, OpenCodeGoBackend};

#[test]
fn settings_that_never_finished_a_discovery_publish_the_fallback_catalog() {
    let catalog = published_catalog(&AppSettings::default());

    assert_eq!(catalog, OpenCodeGoBackend::fallback_catalog());
}

#[test]
fn a_discovered_catalog_is_published_as_it_was_stored() {
    let mut settings = AppSettings::default();
    settings.cached_models = vec![ModelCatalogEntry {
        id: "qwen3.8-max".to_owned(),
        display_name: "Qwen 3.8 Max".to_owned(),
        is_custom: false,
    }];

    assert_eq!(published_catalog(&settings), settings.cached_models);
}
