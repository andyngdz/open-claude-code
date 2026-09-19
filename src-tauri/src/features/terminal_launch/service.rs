use std::{
    io::{self, IsTerminal, Write},
    process::Command,
};

use open_claude_code_backend::open_code_go_public_model_id;

use super::CliError;
use crate::features::{
    errors::RuntimeEndpointError,
    launcher::{apply_appimage_host_env, apply_proxy_env, claude_executable, claude_proxy_env},
    runtime_endpoint::{
        default_model_index, remembered_model_index, resolve_model_choice, RuntimeEndpoint,
        RuntimeModel,
    },
    settings::{AppSettings, SettingsStore},
};

/// Attaches to the running gateway and replaces this process with Claude Code.
///
/// `claude_args` arrive from the `--` suffix and go to Claude Code unchanged.
pub(crate) fn launch(model: Option<String>, claude_args: &[String]) -> Result<(), CliError> {
    let store = SettingsStore::for_application().map_err(|_| CliError::NotRunning)?;
    let endpoint = RuntimeEndpoint::read(&store.runtime_path()).map_err(map_read_error)?;
    // Settings only supply the prompt default, so an unusable file must not stop a launch.
    let mut settings = store.load().ok();
    let remember_model = model.is_none();
    let cli_model_id = settings
        .as_ref()
        .and_then(|settings| settings.cli_launch_model_id.as_deref());
    let dashboard_model_id = settings
        .as_ref()
        .and_then(|settings| settings.launch_model_id.as_deref());
    let default_index = prompt_default_index(&endpoint, cli_model_id, dashboard_model_id);
    let model_id = select_model(&endpoint, model, default_index)?;
    if remember_model {
        remember_cli_launch_model(&store, settings.as_mut(), &model_id);
    }
    exec_claude(&endpoint, &model_id, claude_args)
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

fn select_model(
    endpoint: &RuntimeEndpoint,
    requested: Option<String>,
    default_index: usize,
) -> Result<String, CliError> {
    if let Some(model_id) = requested {
        return resolve_model_choice(&model_id, &endpoint.models, 0).ok_or(CliError::UnknownModel);
    }
    if !io::stdin().is_terminal() {
        return Err(CliError::ModelRequired);
    }
    print_model_prompt(&endpoint.models, default_index)?;
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .map_err(|_| CliError::Unreachable)?;
    resolve_model_choice(&choice, &endpoint.models, default_index).ok_or(CliError::UnknownModel)
}

/// Picks the prompt default: the CLI's last pick, then the dashboard's, then the Sonnet alias.
///
/// The dashboard keeps its own default in `launch_model_id`, so the terminal only
/// borrows it as a fallback and never writes it back.
fn prompt_default_index(
    endpoint: &RuntimeEndpoint,
    cli_model_id: Option<&str>,
    dashboard_model_id: Option<&str>,
) -> usize {
    remembered_model_index(&endpoint.models, cli_model_id)
        .or_else(|| remembered_model_index(&endpoint.models, dashboard_model_id))
        .unwrap_or_else(|| default_model_index(&endpoint.models, &endpoint.aliases.sonnet))
}

/// Stores the model picked at the prompt so the next terminal launch defaults to it.
fn remember_cli_launch_model(
    store: &SettingsStore,
    settings: Option<&mut AppSettings>,
    model_id: &str,
) {
    let Some(settings) = settings else {
        return;
    };
    if settings.cli_launch_model_id.as_deref() == Some(model_id) {
        return;
    }
    settings.cli_launch_model_id = Some(model_id.to_owned());
    // The launch is already under way, so a settings write that fails is not fatal.
    store.save(settings).ok();
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

fn exec_claude(
    endpoint: &RuntimeEndpoint,
    model_id: &str,
    claude_args: &[String],
) -> Result<(), CliError> {
    let program = claude_executable().map_err(|_| CliError::ClaudeNotFound)?;
    let proxy_env = claude_proxy_env(&endpoint.base_url, &endpoint.token, &endpoint.aliases);
    let mut command = Command::new(program);
    command
        .arg(crate::constants::MODEL_FLAG)
        .arg(open_code_go_public_model_id(model_id))
        .args(claude_args);
    apply_appimage_host_env(&mut command);
    apply_proxy_env(&mut command, &proxy_env);
    run_claude(&mut command).map_err(CliError::Spawn)
}

#[cfg(unix)]
fn run_claude(command: &mut Command) -> io::Result<()> {
    let error = std::os::unix::process::CommandExt::exec(command);
    Err(error)
}

#[cfg(not(unix))]
fn run_claude(command: &mut Command) -> io::Result<()> {
    command.status().map(|_| ())
}

#[cfg(test)]
#[path = "service_test.rs"]
mod service_test;
