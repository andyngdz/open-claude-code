mod probe;

use std::{
    io::{self, IsTerminal, Write},
    process::Command,
};

use open_claude_code_backend::{
    open_code_go_public_model_id, split_one_million_suffix, with_one_million_suffix, ContextWindow,
};

use super::CliError;
use crate::features::{
    errors::RuntimeEndpointError,
    launch_window,
    launcher::{
        apply_appimage_host_env, apply_proxy_env, claude_executable, claude_proxy_env,
        host_working_directory,
    },
    runtime_endpoint::{
        default_model_index, remembered_model_index, resolve_model_choice, RuntimeEndpoint,
        RuntimeModel,
    },
    settings::SettingsStore,
};

/// Attaches to the running gateway and replaces this process with Claude Code.
///
/// `claude_args` arrive from the `--` suffix and go to Claude Code unchanged.
pub(crate) fn launch(model: Option<String>, claude_args: &[String]) -> Result<(), CliError> {
    let store = SettingsStore::for_application().map_err(|_| CliError::NotRunning)?;
    let endpoint = RuntimeEndpoint::read(&store.runtime_path()).map_err(map_read_error)?;
    // The gateway dies with the app, so a handshake an app left behind without
    // closing cleanly still names a port nothing answers on.
    if !probe::gateway_is_serving(&endpoint.base_url) {
        return Err(CliError::NotRunning);
    }
    // Settings only supply the prompt default, so an unusable file must not stop a launch.
    // It then reads the same as having no remembered model at all.
    let settings = store.load().ok();
    let remember_model = model.is_none();
    let cli_model_id = settings
        .as_ref()
        .and_then(|settings| settings.cli_launch_model_id.as_deref());
    let dashboard_model_id = settings
        .as_ref()
        .and_then(|settings| settings.launch_model_id.as_deref());
    let default_index = prompt_default_index(&endpoint, cli_model_id, dashboard_model_id);
    let (requested_model, requested_window) = requested_model(model);
    let model_id = select_model(&endpoint, requested_model, default_index)?;
    if remember_model {
        remember_cli_launch_model(&store, &model_id);
    }
    let window = launch_context_window(&endpoint, &model_id, requested_window);
    exec_claude(&endpoint, &model_id, window, claude_args)
}

/// Splits the 1M marker off the model the caller named on the command line.
///
/// The catalog is searched with the bare id, so a marker on an id the catalog
/// does not offer still reads as an unknown model rather than a launch.
fn requested_model(model: Option<String>) -> (Option<String>, ContextWindow) {
    let Some(model_id) = model else {
        return (None, ContextWindow::Standard);
    };
    let (bare_id, window) = split_one_million_suffix(&model_id);
    (Some(bare_id.to_owned()), window)
}

/// Returns the window Claude Code is told for the model this launch picked.
///
/// A marker the caller typed wins over the launch rows, so `--model 'x[1m]'` is
/// never quietly downgraded.
fn launch_context_window(
    endpoint: &RuntimeEndpoint,
    model_id: &str,
    requested: ContextWindow,
) -> ContextWindow {
    requested.widest(launch_window::declared_window(
        &endpoint.aliases,
        endpoint.launch_model_id.as_deref(),
        endpoint.launch_extended_context,
        model_id,
    ))
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
///
/// Reads the document here rather than at launch time: the prompt can sit open
/// for minutes, and the app rewrites the whole file from its own memory.
fn remember_cli_launch_model(store: &SettingsStore, model_id: &str) {
    let Some(mut settings) = store.load().ok() else {
        return;
    };
    if settings.cli_launch_model_id.as_deref() == Some(model_id) {
        return;
    }
    settings.cli_launch_model_id = Some(model_id.to_owned());
    // The launch is already under way, so a settings write that fails is not fatal.
    store.save(&settings).ok();
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
    window: ContextWindow,
    claude_args: &[String],
) -> Result<(), CliError> {
    let program = claude_executable().map_err(|_| CliError::ClaudeNotFound)?;
    let proxy_env = claude_proxy_env(&endpoint.base_url, &endpoint.token, &endpoint.aliases);
    let mut command = Command::new(program);
    command
        .arg(crate::constants::MODEL_FLAG)
        .arg(open_code_go_public_model_id(&with_one_million_suffix(
            model_id, window,
        )))
        .args(claude_args);
    // The exec inherits this process's directory, which an AppImage run has moved
    // into its mount, so the session goes back to the directory the user called from.
    if let Some(working_directory) = host_working_directory() {
        command.current_dir(working_directory);
    }
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
