use super::should_check_terminal_liveness;
use crate::features::launcher::TerminalKind;

#[test]
fn system_default_does_not_require_terminal_liveness_check() {
    assert!(!should_check_terminal_liveness(TerminalKind::SystemDefault));
    #[cfg(not(target_os = "macos"))]
    assert!(should_check_terminal_liveness(TerminalKind::Ghostty));
    #[cfg(target_os = "macos")]
    assert!(!should_check_terminal_liveness(TerminalKind::Ghostty));
}
