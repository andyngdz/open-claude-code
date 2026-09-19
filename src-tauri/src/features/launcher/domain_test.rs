use super::TerminalKind;

#[test]
fn terminal_wire_values_match_the_dashboard_contract() {
    assert_eq!(
        serde_json::to_string(&TerminalKind::ITerm2).expect("iTerm2 should serialize"),
        "\"iterm2\""
    );
    assert_eq!(
        serde_json::to_string(&TerminalKind::WezTerm).expect("WezTerm should serialize"),
        "\"wezterm\""
    );
    assert_eq!(
        serde_json::from_str::<TerminalKind>("\"iterm2\"")
            .expect("dashboard iTerm2 value should deserialize"),
        TerminalKind::ITerm2
    );
    assert_eq!(
        serde_json::from_str::<TerminalKind>("\"wezterm\"")
            .expect("dashboard WezTerm value should deserialize"),
        TerminalKind::WezTerm
    );
}
