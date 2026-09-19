use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

use open_claude_code_backend::{
    ModelCatalogEntry, DEFAULT_FAST_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID,
};

use crate::features::{errors::SettingsError, launcher::TerminalKind};

const SETTINGS_VERSION: u8 = 1;

/// Maps Claude Code's four model families to provider model IDs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelAliasMapping {
    pub(crate) fable: String,
    pub(crate) opus: String,
    pub(crate) sonnet: String,
    pub(crate) haiku: String,
}

impl Default for ModelAliasMapping {
    fn default() -> Self {
        Self {
            fable: DEFAULT_PRIMARY_MODEL_ID.to_owned(),
            opus: DEFAULT_PRIMARY_MODEL_ID.to_owned(),
            sonnet: DEFAULT_PRIMARY_MODEL_ID.to_owned(),
            haiku: DEFAULT_FAST_MODEL_ID.to_owned(),
        }
    }
}

/// Stores the non-secret settings persisted by the desktop application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppSettings {
    version: u8,
    pub(crate) terminal: TerminalKind,
    pub(crate) last_workspace: Option<PathBuf>,
    pub(crate) aliases: ModelAliasMapping,
    #[serde(default)]
    pub(crate) launch_model_id: Option<String>,
    /// Model picked at the terminal prompt. Only the CLI writes it.
    #[serde(default)]
    pub(crate) cli_launch_model_id: Option<String>,
    pub(crate) custom_models: Vec<String>,
    pub(crate) cached_models: Vec<ModelCatalogEntry>,
    pub(crate) catalog_refreshed_at_epoch_seconds: Option<u64>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            terminal: TerminalKind::SystemDefault,
            last_workspace: None,
            aliases: ModelAliasMapping::default(),
            launch_model_id: None,
            cli_launch_model_id: None,
            custom_models: Vec::new(),
            cached_models: Vec::new(),
            catalog_refreshed_at_epoch_seconds: None,
        }
    }
}

/// Carries provider settings accepted from the dashboard.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SaveProviderSettingsInput {
    pub(crate) terminal: TerminalKind,
    pub(crate) model_id: String,
    pub(crate) aliases: ModelAliasMapping,
    pub(crate) custom_models: Vec<String>,
}

/// Persists versioned non-secret settings as JSON under the OS config directory.
#[derive(Clone, Debug)]
pub(crate) struct SettingsStore {
    path: PathBuf,
}

/// Holds exclusive ownership of the session files for one desktop process.
#[derive(Debug)]
pub(crate) struct InstanceGuard {
    _lock_file: File,
}

impl InstanceGuard {
    /// Acquires the per-user application lock until this process exits.
    pub(crate) fn acquire(lock_path: &Path) -> Result<Self, SettingsError> {
        let Some(parent_directory) = lock_path.parent() else {
            return Err(SettingsError::DirectoryUnavailable);
        };
        fs::create_dir_all(parent_directory).map_err(SettingsError::Write)?;
        let lock_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(lock_path)
            .map_err(SettingsError::Lock)?;
        match lock_file.try_lock_exclusive() {
            Ok(()) => Ok(Self {
                _lock_file: lock_file,
            }),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                Err(SettingsError::InstanceAlreadyRunning)
            }
            Err(error) => Err(SettingsError::Lock(error)),
        }
    }
}

impl SettingsStore {
    /// Creates the settings store for this application.
    pub(crate) fn for_application() -> Result<Self, SettingsError> {
        let project_directories =
            directories::ProjectDirs::from("dev", "andyng", crate::constants::APP_NAME)
                .ok_or(SettingsError::DirectoryUnavailable)?;

        Ok(Self {
            path: project_directories.config_dir().join("settings.json"),
        })
    }

    /// Loads settings or returns defaults when no settings file exists yet.
    pub(crate) fn load(&self) -> Result<AppSettings, SettingsError> {
        if !self.path.exists() {
            return Ok(AppSettings::default());
        }

        let serialized = fs::read_to_string(&self.path).map_err(SettingsError::Read)?;
        serde_json::from_str(&serialized).map_err(SettingsError::Parse)
    }

    /// Returns the private file the CLI reads while this app is running.
    pub(crate) fn runtime_path(&self) -> PathBuf {
        self.path
            .with_file_name(crate::constants::RUNTIME_FILE_NAME)
    }

    /// Returns the lock file used to prevent concurrent app sessions.
    pub(crate) fn instance_lock_path(&self) -> PathBuf {
        self.path.with_file_name("session.lock")
    }

    /// Atomically replaces the persisted settings document.
    pub(crate) fn save(&self, settings: &AppSettings) -> Result<(), SettingsError> {
        let Some(parent_directory) = self.path.parent() else {
            return Err(SettingsError::DirectoryUnavailable);
        };
        fs::create_dir_all(parent_directory).map_err(SettingsError::Write)?;

        let serialized = serde_json::to_vec_pretty(settings).map_err(SettingsError::Serialize)?;
        let temporary_path = self.path.with_extension("json.tmp");
        fs::write(&temporary_path, serialized).map_err(SettingsError::Write)?;
        fs::rename(temporary_path, &self.path).map_err(SettingsError::Write)
    }
}

/// Normalizes and validates custom model IDs before they reach the proxy.
pub(crate) fn normalize_custom_models(models: Vec<String>) -> Vec<String> {
    let mut normalized = models
        .into_iter()
        .map(|model| model.trim().to_owned())
        .filter(|model| !model.is_empty())
        .collect::<Vec<_>>();
    normalized.sort();
    normalized.dedup();
    normalized
}

/// Returns the current Unix timestamp for cache freshness metadata.
pub(crate) fn current_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

/// Returns whether a path points to a directory that can be used as a workspace.
pub(crate) fn is_workspace_directory(path: &Path) -> bool {
    path.is_dir()
}

#[cfg(test)]
#[path = "settings_test.rs"]
mod settings_test;
