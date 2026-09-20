use super::{
    open_code_go_public_model_id, split_one_million_suffix, with_one_million_suffix, ContextWindow,
    OpenCodeGoBackend,
};

#[test]
fn public_model_id_keeps_provider_and_model() {
    let public_id = open_code_go_public_model_id("qwen3.8-max");

    assert!(public_id.contains("opencode-go"));
    assert!(public_id.ends_with("/qwen3.8-max"));
}

#[test]
fn fallback_catalog_is_empty_until_authenticated_discovery() {
    let catalog = OpenCodeGoBackend::fallback_catalog();

    assert!(catalog.is_empty());
}

#[test]
fn the_one_million_marker_is_read_off_a_model_id() {
    let cases = [
        (
            "qwen3.8-max[1m]",
            ("qwen3.8-max", ContextWindow::OneMillion),
        ),
        (
            "qwen3.8-max[1M]",
            ("qwen3.8-max", ContextWindow::OneMillion),
        ),
        ("qwen3.8-max", ("qwen3.8-max", ContextWindow::Standard)),
        ("qwen[1m]max", ("qwen[1m]max", ContextWindow::Standard)),
    ];

    for (model_id, expected) in cases {
        assert_eq!(split_one_million_suffix(model_id), expected, "{model_id}");
    }
}

#[test]
fn a_model_id_that_is_only_the_marker_keeps_its_text() {
    // An empty model id would reach the catalog lookup and read as a missing model.
    assert_eq!(
        split_one_million_suffix("[1m]"),
        ("[1m]", ContextWindow::Standard)
    );
}

#[test]
fn an_id_shorter_than_the_marker_stays_intact() {
    assert_eq!(
        split_one_million_suffix("qw"),
        ("qw", ContextWindow::Standard)
    );
    assert_eq!(split_one_million_suffix(""), ("", ContextWindow::Standard));
}

#[test]
fn a_model_id_is_marked_at_most_once() {
    let marked_once = with_one_million_suffix("qwen3.8-max[1m]", ContextWindow::OneMillion);

    assert_eq!(marked_once, "qwen3.8-max[1m]");
}

#[test]
fn the_marker_lands_the_same_way_before_or_after_the_public_prefix() {
    let marked_then_prefixed = open_code_go_public_model_id(&with_one_million_suffix(
        "qwen3.8-max",
        ContextWindow::OneMillion,
    ));
    let prefixed_then_marked = with_one_million_suffix(
        &open_code_go_public_model_id("qwen3.8-max"),
        ContextWindow::OneMillion,
    );

    assert_eq!(marked_then_prefixed, prefixed_then_marked);
    assert_eq!(
        split_one_million_suffix(&prefixed_then_marked),
        (
            "open-claude-code/claude/opencode-go/qwen3.8-max",
            ContextWindow::OneMillion
        )
    );
}
