use std::{fs, io::Write, os::unix::fs::OpenOptionsExt, path::Path};

use serde::{Deserialize, Serialize};

use crate::features::{errors::RuntimeEndpointError, settings::ModelAliasMapping};

/// Model row the CLI can print without opening the settings window.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RuntimeModel {
    pub(crate) id: String,
    pub(crate) display_name: String,
}

/// Loopback handshake written while the tray app owns the gateway.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RuntimeEndpoint {
    pub(crate) base_url: String,
    pub(crate) token: String,
    pub(crate) models: Vec<RuntimeModel>,
    pub(crate) aliases: ModelAliasMapping,
}

impl RuntimeEndpoint {
    /// Writes the handshake so only the current user can read the proxy token.
    pub(crate) fn write(&self, path: &Path) -> Result<(), RuntimeEndpointError> {
        let Some(parent_directory) = path.parent() else {
            return Err(RuntimeEndpointError::DirectoryUnavailable);
        };
        fs::create_dir_all(parent_directory).map_err(RuntimeEndpointError::Write)?;
        let serialized =
            serde_json::to_vec_pretty(self).map_err(RuntimeEndpointError::Serialize)?;
        write_private(path, &serialized)
    }

    /// Loads the handshake written by the running app.
    pub(crate) fn read(path: &Path) -> Result<Self, RuntimeEndpointError> {
        let serialized = fs::read_to_string(path).map_err(RuntimeEndpointError::Read)?;
        serde_json::from_str(&serialized).map_err(RuntimeEndpointError::Parse)
    }

    /// Removes the handshake when the app exits.
    pub(crate) fn remove(path: &Path) -> Result<(), RuntimeEndpointError> {
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(RuntimeEndpointError::Write(error)),
        }
    }
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), RuntimeEndpointError> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .map_err(RuntimeEndpointError::Write)?;
    file.write_all(bytes).map_err(RuntimeEndpointError::Write)
}

/// Returns the model id selected by a number, an id, or an empty default line.
pub(crate) fn resolve_model_choice(
    input: &str,
    models: &[RuntimeModel],
    default_index: usize,
) -> Option<String> {
    let choice = input.trim();
    if choice.is_empty() {
        return models.get(default_index).map(|model| model.id.clone());
    }
    if let Ok(number) = choice.parse::<usize>() {
        return number
            .checked_sub(1)
            .and_then(|index| models.get(index))
            .map(|model| model.id.clone());
    }
    models
        .iter()
        .find(|model| model.id == choice)
        .map(|model| model.id.clone())
}

/// Picks the saved Sonnet alias when it is still in the catalog.
pub(crate) fn default_model_index(models: &[RuntimeModel], sonnet: &str) -> usize {
    models
        .iter()
        .position(|model| model.id == sonnet)
        .unwrap_or(0)
}

#[cfg(test)]
const MISSING_MODEL_FLAG: &str = "Pass a model id after --model.";

/// Reads one launch model flag, if the caller passed one.
#[cfg(test)]
pub(crate) fn parse_model_flag(args: &[String]) -> Result<Option<String>, String> {
    let mut model = None;
    let mut index = 0;
    while index < args.len() {
        let argument = &args[index];
        if let Some(value) = argument.strip_prefix(&format!("{}=", crate::constants::MODEL_FLAG)) {
            if value.is_empty() {
                return Err(MISSING_MODEL_FLAG.to_owned());
            }
            model = Some(value.to_owned());
        } else if argument == crate::constants::MODEL_FLAG {
            let Some(value) = args.get(index + 1) else {
                return Err(MISSING_MODEL_FLAG.to_owned());
            };
            if value.is_empty() {
                return Err(MISSING_MODEL_FLAG.to_owned());
            }
            model = Some(value.clone());
            index += 1;
        } else {
            return Err(format!("Unexpected argument: {argument}"));
        }
        index += 1;
    }
    Ok(model)
}

/// Returns a path that tests can pass without touching the real config directory.
#[cfg(test)]
pub(crate) fn runtime_path_in(directory: &Path) -> std::path::PathBuf {
    directory.join(crate::constants::RUNTIME_FILE_NAME)
}

#[cfg(test)]
#[path = "runtime_endpoint_test.rs"]
mod runtime_endpoint_test;
