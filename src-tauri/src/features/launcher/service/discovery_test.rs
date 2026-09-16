use super::list_available_terminals;
use crate::features::launcher::TerminalKind;

#[test]
fn system_default_is_always_the_first_terminal_choice() {
    let terminals = list_available_terminals();

    assert_eq!(terminals[0].kind, TerminalKind::SystemDefault);
}
