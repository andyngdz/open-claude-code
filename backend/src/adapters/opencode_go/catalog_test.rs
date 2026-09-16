use indoc::indoc;

use super::{UpstreamModelList, MODEL_QWEN_38_MAX};

#[test]
fn excludes_models_that_require_another_protocol() {
    let upstream: UpstreamModelList = serde_json::from_str(indoc! {r#"
        {
          "data": [
            { "id": "qwen3.8-max" },
            { "id": "grok-4.6" }
          ]
        }
    "#})
    .expect("catalog fixture should parse");

    let catalog = upstream.into_messages_catalog();

    assert_eq!(catalog.len(), 1);
    assert_eq!(catalog[0].id, MODEL_QWEN_38_MAX);
}
