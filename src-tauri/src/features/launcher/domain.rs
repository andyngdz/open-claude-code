use std::{path::PathBuf, process::Child, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::features::errors::LauncherError;

/// Identifies the terminal applications supported by the Linux launcher.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TerminalKind {
    #[default]
    SystemDefault,
    Ghostty,
    GnomeTerminal,
    Konsole,
    Kitty,
    Alacritty,
}

impl TerminalKind {
    /// Returns the executable used by a concrete terminal choice.
    pub(crate) fn executable(self) -> Option<&'static str> {
        match self {
            Self::SystemDefault => None,
            Self::Ghostty => Some("ghostty"),
            Self::GnomeTerminal => Some("gnome-terminal"),
            Self::Konsole => Some("konsole"),
            Self::Kitty => Some("kitty"),
            Self::Alacritty => Some("alacritty"),
        }
    }

    /// Returns the dashboard label for this terminal.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::SystemDefault => "System default",
            Self::Ghostty => "Ghostty",
            Self::GnomeTerminal => "GNOME Terminal",
            Self::Konsole => "Konsole",
            Self::Kitty => "Kitty",
            Self::Alacritty => "Alacritty",
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
