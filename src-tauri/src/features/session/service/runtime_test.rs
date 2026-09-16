use super::nonempty_catalog;
use open_claude_code_backend::OpenCodeGoBackend;

#[test]
fn empty_catalog_uses_the_fallback_list() {
    let catalog = nonempty_catalog(Vec::new());

    assert!(!catalog.is_empty());
    assert_eq!(catalog, OpenCodeGoBackend::fallback_catalog());
}
