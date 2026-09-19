use std::path::PathBuf;
#[cfg(target_os = "macos")]
use std::process::Command;

use crate::features::launcher::{TerminalKind, TerminalOption};

#[cfg(not(target_os = "macos"))]
pub(super) const XDG_TERMINAL_EXECUTABLE: &str = "xdg-terminal-exec";

#[cfg(target_os = "macos")]
const MACOS_UTILITIES_DIR: &str = "/System/Applications/Utilities";

/// Lists System default plus each terminal that is installed on this machine.
pub(crate) fn list_available_terminals() -> Vec<TerminalOption> {
    let mut terminals = vec![TerminalOption {
        kind: TerminalKind::SystemDefault,
        label: TerminalKind::SystemDefault.label().to_owned(),
        is_available: platform_default_available(),
    }];
    for kind in TerminalKind::concrete_kinds() {
        if is_installed(*kind) {
            terminals.push(TerminalOption {
                kind: *kind,
                label: kind.label().to_owned(),
                is_available: true,
            });
        }
    }
    terminals
}

/// Returns whether the selected terminal can still be launched.
pub(crate) fn is_terminal_launchable(kind: TerminalKind) -> bool {
    match kind {
        TerminalKind::SystemDefault => platform_default_available(),
        TerminalKind::AppleTerminal
        | TerminalKind::ITerm2
        | TerminalKind::Ghostty
        | TerminalKind::Warp
        | TerminalKind::Kitty
        | TerminalKind::Alacritty
        | TerminalKind::WezTerm
        | TerminalKind::Hyper
        | TerminalKind::GnomeTerminal
        | TerminalKind::Konsole => is_installed(kind),
    }
}

fn platform_default_available() -> bool {
    #[cfg(target_os = "macos")]
    {
        is_installed(TerminalKind::AppleTerminal)
    }
    #[cfg(target_os = "linux")]
    {
        find_executable(XDG_TERMINAL_EXECUTABLE).is_some()
            || TerminalKind::concrete_kinds()
                .iter()
                .any(|kind| is_installed(*kind))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        false
    }
}

/// Returns whether a concrete terminal is installed on this platform.
pub(super) fn is_installed(kind: TerminalKind) -> bool {
    #[cfg(target_os = "macos")]
    {
        find_macos_app_path(kind).is_some() || kind.executable().and_then(find_executable).is_some()
    }
    #[cfg(target_os = "linux")]
    {
        kind.executable().and_then(find_executable).is_some()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        kind.executable().and_then(find_executable).is_some()
    }
}

#[cfg(target_os = "macos")]
fn find_macos_app_path(kind: TerminalKind) -> Option<PathBuf> {
    let app_name = kind.macos_app_name()?;
    let app_file = format!("{app_name}.app");
    for candidate in macos_app_candidates(&app_file, kind) {
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    find_macos_app_via_mdfind(kind)
}

#[cfg(target_os = "macos")]
fn macos_app_candidates(app_file: &str, kind: TerminalKind) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if kind == TerminalKind::AppleTerminal {
        candidates.push(PathBuf::from(MACOS_UTILITIES_DIR).join(app_file));
    }
    candidates.push(PathBuf::from("/Applications").join(app_file));
    if let Some(home) = std::env::var_os("HOME") {
        candidates.push(PathBuf::from(home).join("Applications").join(app_file));
    }
    if kind != TerminalKind::AppleTerminal {
        candidates.push(PathBuf::from(MACOS_UTILITIES_DIR).join(app_file));
    }
    candidates
}

#[cfg(target_os = "macos")]
fn find_macos_app_via_mdfind(kind: TerminalKind) -> Option<PathBuf> {
    let bundle_id = kind.macos_bundle_id()?;
    let output = Command::new("mdfind")
        .arg(format!("kMDItemCFBundleIdentifier == '{bundle_id}'"))
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .find(|path| path.is_dir())
}

/// Resolves an executable on PATH, including Windows PATHEXT handling via `which`.
pub(super) fn find_executable(executable_name: &str) -> Option<PathBuf> {
    which::which(executable_name).ok()
}

/// Returns supported concrete terminals in fallback priority order.
#[cfg(not(target_os = "macos"))]
pub(super) fn concrete_terminals() -> &'static [TerminalKind] {
    TerminalKind::concrete_kinds()
}

#[cfg(test)]
#[path = "discovery_test.rs"]
mod discovery_test;
