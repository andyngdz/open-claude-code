use std::{env, path::PathBuf};

use crate::features::launcher::{TerminalKind, TerminalOption};

pub(super) const XDG_TERMINAL_EXECUTABLE: &str = "xdg-terminal-exec";

/// Lists the system-default choice followed by supported terminals found on PATH.
pub(crate) fn list_available_terminals() -> Vec<TerminalOption> {
    let concrete_terminals = concrete_terminals();
    let default_available = find_executable(XDG_TERMINAL_EXECUTABLE).is_some()
        || concrete_terminals
            .iter()
            .any(|terminal| terminal.executable().and_then(find_executable).is_some());

    std::iter::once(TerminalOption {
        kind: TerminalKind::SystemDefault,
        label: TerminalKind::SystemDefault.label().to_owned(),
        is_available: default_available,
    })
    .chain(
        concrete_terminals
            .into_iter()
            .map(|terminal| TerminalOption {
                kind: terminal,
                label: terminal.label().to_owned(),
                is_available: terminal.executable().and_then(find_executable).is_some(),
            }),
    )
    .collect()
}

/// Returns supported concrete terminals in fallback priority order.
pub(super) fn concrete_terminals() -> [TerminalKind; 5] {
    [
        TerminalKind::Ghostty,
        TerminalKind::GnomeTerminal,
        TerminalKind::Konsole,
        TerminalKind::Kitty,
        TerminalKind::Alacritty,
    ]
}

/// Resolves an executable using PATH without invoking a shell.
pub(super) fn find_executable(executable_name: &str) -> Option<PathBuf> {
    let search_path = env::var_os("PATH")?;
    env::split_paths(&search_path)
        .map(|directory| directory.join(executable_name))
        .find(|candidate| candidate.is_file())
}

#[cfg(test)]
#[path = "discovery_test.rs"]
mod discovery_test;
