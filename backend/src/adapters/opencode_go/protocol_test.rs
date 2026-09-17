use super::{protocol_candidates, validation_payload};

#[test]
fn builds_validation_payload_from_the_discovered_model_id() {
    let body = validation_payload("dynamic-model").expect("payload should serialize");
    let value: serde_json::Value = serde_json::from_slice(&body).expect("payload should parse");

    assert_eq!(value["model"], "dynamic-model");
}

#[test]
fn considers_every_supported_upstream_protocol() {
    assert_eq!(protocol_candidates().len(), 3);
}
