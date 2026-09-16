use std::collections::HashSet;

use serde::Deserialize;

use crate::features::providers::ModelCatalogEntry;

pub(super) const MODEL_QWEN_38_FLASH: &str = "qwen3.8-flash";

const MODEL_QWEN_38_MAX: &str = "qwen3.8-max";
const MESSAGES_MODELS: [(&str, &str); 8] = [
    ("minimax-m3", "MiniMax M3"),
    ("minimax-m2.7", "MiniMax M2.7"),
    ("minimax-m2.5", "MiniMax M2.5"),
    (MODEL_QWEN_38_MAX, "Qwen 3.8 Max"),
    (MODEL_QWEN_38_FLASH, "Qwen 3.8 Flash"),
    ("qwen3.7-max", "Qwen 3.7 Max"),
    ("qwen3.7-plus", "Qwen 3.7 Plus"),
    ("qwen3.6-plus", "Qwen 3.6 Plus"),
];

#[derive(Debug, Deserialize)]
/// Represents the model list returned by OpenCode Go discovery.
pub(super) struct UpstreamModelList {
    data: Vec<UpstreamModel>,
}

#[derive(Debug, Deserialize)]
struct UpstreamModel {
    id: String,
}

impl UpstreamModelList {
    pub(super) fn into_messages_catalog(self) -> Vec<ModelCatalogEntry> {
        let upstream_ids = self
            .data
            .into_iter()
            .map(|model| model.id)
            .collect::<HashSet<_>>();
        fallback_catalog()
            .into_iter()
            .filter(|model| upstream_ids.contains(&model.id))
            .collect()
    }
}

/// Returns the checked-in catalog used when the upstream list is unavailable.
pub(crate) fn fallback_catalog() -> Vec<ModelCatalogEntry> {
    MESSAGES_MODELS
        .iter()
        .map(|(id, display_name)| ModelCatalogEntry {
            id: (*id).to_owned(),
            display_name: (*display_name).to_owned(),
            is_custom: false,
        })
        .collect()
}

#[cfg(test)]
#[path = "catalog_test.rs"]
mod catalog_test;
