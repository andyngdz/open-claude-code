use std::{
    io::{self, IsTerminal, Write},
    process::Command,
};

use open_claude_code_backend::open_code_go_public_model_id;

use super::CliError;
use crate::features::{
    errors::RuntimeEndpointError,
    launcher::{apply_proxy_env, claude_executable, claude_proxy_env},
    runtime_endpoint::{default_model_index, resolve_model_choice, RuntimeEndpoint, RuntimeModel},
    settings::SettingsStore,
};

/// Attaches to the running gateway and replaces this process with Claude Code.
pub(crate) fn launch(model: Option<String>) -> Result<(), CliError> {
    let store = SettingsStore::for_application().map_err(|_| CliError::NotRunning)?;
    let endpoint = RuntimeEndpoint::read(&store.runtime_path()).map_err(map_read_error)?;
    let model_id = select_model(&endpoint, model)?;
    exec_claude(&endpoint, &model_id)
}

fn map_read_error(error: RuntimeEndpointError) -> CliError {
    match error {
        RuntimeEndpointError::Read(source) if source.kind() == io::ErrorKind::NotFound => {
            CliError::NotRunning
        }
        RuntimeEndpointError::DirectoryUnavailable
        | RuntimeEndpointError::Read(_)
        | RuntimeEndpointError::Parse(_)
        | RuntimeEndpointError::Serialize(_)
        | RuntimeEndpointError::Write(_) => CliError::Unreachable,
    }
}

fn select_model(endpoint: &RuntimeEndpoint, requested: Option<String>) -> Result<String, CliError> {
    if let Some(model_id) = requested {
        return resolve_model_choice(&model_id, &endpoint.models, 0).ok_or(CliError::UnknownModel);
    }
    if !io::stdin().is_terminal() {
        return Err(CliError::ModelRequired);
    }
    let default_index = default_model_index(&endpoint.models, &endpoint.aliases.sonnet);
    print_model_prompt(&endpoint.models, default_index)?;
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|_| CliError::Unreachable)?;
    resolve_model_choice(&choice, &endpoint.models, default_index).ok_or(CliError::UnknownModel)
}

fn print_model_prompt(models: &[RuntimeModel], default_index: usize) -> Result<(), CliError> {
    let mut error_output = io::stderr();
    writeln!(error_output, "Choose a model:").map_err(|_| CliError::Unreachable)?;
    for (index, model) in models.iter().enumerate() {
        writeln!(error_output, "  {}  {}", index + 1, model.display_name)
            .map_err(|_| CliError::Unreachable)?;
    }
    write!(error_output, "Model [{}]: ", default_index + 1).map_err(|_| CliError::Unreachable)?;
    error_output.flush().map_err(|_| CliError::Unreachable)
}

fn exec_claude(endpoint: &RuntimeEndpoint, model_id: &str) -> Result<(), CliError> {
    let program = claude_executable().map_err(|_| CliError::ClaudeNotFound)?;
    let proxy_env = claude_proxy_env(&endpoint.base_url, &endpoint.token, &endpoint.aliases);
    let mut command = Command::new(program);
    command
        .arg(crate::constants::MODEL_FLAG)
        .arg(open_code_go_public_model_id(model_id));
    apply_proxy_env(&mut command, &proxy_env);
    let error = std::os::unix::process::CommandExt::exec(&mut command);
    Err(CliError::Spawn(error))
}

#[cfg(test)]
#[path = "service_test.rs"]
mod service_test;
