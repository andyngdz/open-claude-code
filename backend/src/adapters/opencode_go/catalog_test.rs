use indoc::indoc;

use super::UpstreamModelList;

#[test]
fn preserves_every_model_returned_by_upstream_discovery() {
    let upstream: UpstreamModelList = serde_json::from_str(indoc! {r#"
        {
          "data": [
            { "id": "qwen3.8-max" },
            { "id": "grok-4.6", "name": "Grok 4.6" }
          ]
        }
    "#})
    .expect("catalog fixture should parse");

    let catalog = upstream.into_catalog();

    assert_eq!(catalog.len(), 2);
    assert_eq!(catalog[0].id, "qwen3.8-max");
    assert_eq!(catalog[1].id, "grok-4.6");
    assert_eq!(catalog[0].display_name, "qwen3.8-max");
    assert_eq!(catalog[1].display_name, "Grok 4.6");
}
