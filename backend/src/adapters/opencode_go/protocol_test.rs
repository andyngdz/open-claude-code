use super::protocol_candidates;

#[test]
fn considers_every_supported_upstream_protocol() {
    assert_eq!(protocol_candidates().len(), 3);
}
