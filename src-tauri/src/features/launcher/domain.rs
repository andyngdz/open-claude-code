use std::{path::PathBuf, process::Child, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::features::errors::LauncherError;

const LABEL_SYSTEM_DEFAULT: &str = "System default";
const APP_TERMINAL: &str = "Terminal";
const APP_ITERM: &str = "iTerm";
const LABEL_ITERM2: &str = "iTerm2";
const APP_GHOSTTY: &str = "Ghostty";
const APP_WARP: &str = "Warp";
const APP_KITTY: &str = "kitty";
const LABEL_KITTY: &str = "Kitty";
const APP_ALACRITTY: &str = "Alacritty";
const APP_WEZTERM: &str = "WezTerm";
const APP_HYPER: &str = "Hyper";
const LABEL_GNOME_TERMINAL: &str = "GNOME Terminal";
const LABEL_KONSOLE: &str = "Konsole";

/// Identifies the terminal applications supported by the launcher.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TerminalKind {
    #[default]
    SystemDefault,
    AppleTerminal,
    ITerm2,
    Ghostty,
    Warp,
    Kitty,
    Alacritty,
    WezTerm,
    Hyper,
    GnomeTerminal,
    Konsole,
}

impl TerminalKind {
    /// Returns concrete terminals in discovery priority order.
    pub(crate) fn concrete_kinds() -> &'static [Self] {
        &[
            Self::AppleTerminal,
            Self::ITerm2,
            Self::Ghostty,
            Self::Warp,
            Self::Kitty,
            Self::Alacritty,
            Self::WezTerm,
            Self::Hyper,
            Self::GnomeTerminal,
            Self::Konsole,
        ]
    }

    /// Returns the Linux PATH executable for this terminal, when one exists.
    pub(crate) fn executable(self) -> Option<&'static str> {
        match self {
            Self::SystemDefault
            | Self::AppleTerminal
            | Self::ITerm2
            | Self::Warp
            | Self::WezTerm
            | Self::Hyper => None,
            Self::Ghostty => Some("ghostty"),
            Self::Kitty => Some(APP_KITTY),
            Self::Alacritty => Some("alacritty"),
            Self::GnomeTerminal => Some("gnome-terminal"),
            Self::Konsole => Some("konsole"),
        }
    }

    /// Returns the dashboard label for this terminal.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::SystemDefault => LABEL_SYSTEM_DEFAULT,
            Self::AppleTerminal => APP_TERMINAL,
            Self::ITerm2 => LABEL_ITERM2,
            Self::Ghostty => APP_GHOSTTY,
            Self::Warp => APP_WARP,
            Self::Kitty => LABEL_KITTY,
            Self::Alacritty => APP_ALACRITTY,
            Self::WezTerm => APP_WEZTERM,
            Self::Hyper => APP_HYPER,
            Self::GnomeTerminal => LABEL_GNOME_TERMINAL,
            Self::Konsole => LABEL_KONSOLE,
        }
    }

    /// Returns the macOS bundle identifier used to locate the app.
    pub(crate) fn macos_bundle_id(self) -> Option<&'static str> {
        match self {
            Self::AppleTerminal => Some("com.apple.Terminal"),
            Self::ITerm2 => Some("com.googlecode.iterm2"),
            Self::Ghostty => Some("com.mitchellh.ghostty"),
            Self::Warp => Some("dev.warp.Warp-Stable"),
            Self::Kitty => Some("net.kovidgoyal.kitty"),
            Self::Alacritty => Some("org.alacritty"),
            Self::WezTerm => Some("com.github.wez.wezterm"),
            Self::Hyper => Some("co.zeit.hyper"),
            Self::SystemDefault | Self::GnomeTerminal | Self::Konsole => None,
        }
    }

    /// Returns the macOS app name passed to `open -na`.
    pub(crate) fn macos_app_name(self) -> Option<&'static str> {
        match self {
            Self::AppleTerminal => Some(APP_TERMINAL),
            Self::ITerm2 => Some(APP_ITERM),
            Self::Ghostty => Some(APP_GHOSTTY),
            Self::Warp => Some(APP_WARP),
            Self::Kitty => Some(APP_KITTY),
            Self::Alacritty => Some(APP_ALACRITTY),
            Self::WezTerm => Some(APP_WEZTERM),
            Self::Hyper => Some(APP_HYPER),
            Self::SystemDefault | Self::GnomeTerminal | Self::Konsole => None,
        }
    }
}

/// Describes one terminal choice and whether it is available on this machine.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TerminalOption {
    pub(crate) kind: TerminalKind,
    pub(crate) label: String,
    pub(crate) is_available: bool,
}

/// Carries the workspace and model selected for a new Claude Code session.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchClaudeInput {
    pub(crate) workspace: PathBuf,
    pub(crate) model_id: String,
}

/// Confirms the terminal process created for a Claude Code session.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchReceipt {
    pub(crate) process_id: u32,
    pub(crate) terminal: TerminalKind,
    pub(crate) workspace: PathBuf,
    pub(crate) model_id: String,
}

/// Retains terminal children so completed processes can be reaped on later launches.
#[derive(Debug, Default)]
pub(crate) struct ProcessRegistry {
    children: Mutex<Vec<Child>>,
}

impl ProcessRegistry {
    /// Reaps completed terminals and registers a newly spawned process.
    pub(crate) fn register(&self, child: Child) -> Result<(), LauncherError> {
        let mut children = self
            .children
            .lock()
            .map_err(|_poisoned| LauncherError::RegistryUnavailable)?;
        children.retain_mut(|existing_child| match existing_child.try_wait() {
            Ok(Some(_status)) => false,
            Ok(None) => true,
            Err(_source) => true,
        });
        children.push(child);
        Ok(())
    }
}
