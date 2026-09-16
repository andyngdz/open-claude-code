use serde_json::json;

use super::{discovery_models, public_model_id, resolve_provider_request, MODEL_FIELD};
use crate::features::{
    gateway::GatewayConfiguration,
    providers::{ModelCatalogEntry, ProviderId},
};

fn configuration() -> GatewayConfiguration {
    GatewayConfiguration {
        provider_id: ProviderId::new("opencode-go"),
        catalog: vec![ModelCatalogEntry {
            id: "qwen3.8-max".to_owned(),
            display_name: "Qwen 3.8 Max".to_owned(),
            is_custom: false,
        }],
        custom_models: vec!["custom-model".to_owned()],
    }
}

#[test]
fn public_model_ids_round_trip_into_upstream_requests() {
    let configuration = configuration();
    let public_id = public_model_id(&configuration.provider_id, "qwen3.8-max");
    let request = serde_json::to_vec(&json!({ MODEL_FIELD: public_id }))
        .expect("request fixture should serialize");
    let resolved =
        resolve_provider_request(&configuration, &request).expect("model should resolve");
    let payload: serde_json::Value =
        serde_json::from_slice(&resolved.body).expect("resolved request should parse");

    assert!(public_id.contains("claude"));
    assert_eq!(payload[MODEL_FIELD], "qwen3.8-max");
}

#[test]
fn discovery_labels_custom_models_as_unverified() {
    let models = discovery_models(&configuration());
    let custom = models
        .iter()
        .find(|model| model.display_name == "custom-model")
        .expect("custom model should be published");

    assert!(custom.description.contains("attempted"));
}
