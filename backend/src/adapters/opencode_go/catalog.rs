use serde::Deserialize;

use crate::features::providers::ModelCatalogEntry;

#[derive(Debug, Deserialize)]
/// Represents the model list returned by OpenCode Go discovery.
pub(super) struct UpstreamModelList {
    data: Vec<UpstreamModel>,
}

#[derive(Debug, Deserialize)]
struct UpstreamModel {
    id: String,
    #[serde(default, alias = "name")]
    display_name: Option<String>,
}

impl UpstreamModelList {
    pub(super) fn into_catalog(self) -> Vec<ModelCatalogEntry> {
        self.data
            .into_iter()
            .map(|model| ModelCatalogEntry {
                display_name: model.display_name.unwrap_or_else(|| model.id.clone()),
                id: model.id,
                is_custom: false,
            })
            .collect()
    }
}

/// Returns an empty catalog until the authenticated upstream discovery succeeds.
pub(crate) fn fallback_catalog() -> Vec<ModelCatalogEntry> {
    Vec::new()
}

#[cfg(test)]
#[path = "catalog_test.rs"]
mod catalog_test;
