use super::{find_executable, list_available_terminals};
use crate::features::launcher::TerminalKind;

#[test]
fn system_default_is_always_the_first_terminal_choice() {
    let terminals = list_available_terminals();

    assert_eq!(terminals[0].kind, TerminalKind::SystemDefault);
}

#[test]
fn list_available_terminals_only_includes_available_entries() {
    let terminals = list_available_terminals();

    assert!(
        terminals.iter().all(|terminal| terminal.is_available)
            || terminals
                .iter()
                .any(|terminal| terminal.kind == TerminalKind::SystemDefault)
    );
    assert!(terminals[1..].iter().all(|terminal| terminal.is_available));
}

#[test]
fn find_executable_resolves_a_binary_on_path() {
    let sh_path = find_executable("sh").expect("sh should be on PATH in the test environment");

    assert!(sh_path.is_file());
}

#[test]
fn find_executable_returns_none_for_missing_binary() {
    assert!(find_executable("open-claude-code-missing-binary-zzzz").is_none());
}
