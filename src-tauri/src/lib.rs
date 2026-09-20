mod constants;
mod features;
mod interface;

use tauri::{Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;

use crate::features::{gateway::GatewayProcess, session::AppSession};

/// Application name shared by the tray, CLI, and config directory.
pub const APP_NAME: &str = crate::constants::APP_NAME;

/// Runs the desktop application until its event loop exits.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // Registers the app itself, with no arguments: the dashboard switch turns
        // this into a login item, and the state lives in the OS, not in settings.
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let session = tauri::async_runtime::block_on(AppSession::start())
                .map_err(|error| startup_failure(error.to_string()))?;
            app.manage(session);
            crate::interface::tray::install(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            interface::commands::dashboard_snapshot,
            interface::commands::saved_api_key,
            interface::commands::save_api_key,
            interface::commands::remove_credential,
            interface::commands::refresh_catalog,
            interface::commands::save_dashboard_settings,
            interface::commands::launch_claude_session,
        ])
        .build(tauri::generate_context!())?
        .run(|app, event| {
            if let RunEvent::Exit = event {
                let Some(session) = app.try_state::<AppSession>() else {
                    return;
                };
                tauri::async_runtime::block_on(session.stop()).ok();
            }
        });
    Ok(())
}

/// Runs `launch` in the current terminal without opening the settings window.
///
/// `claude_args` reach Claude Code unchanged, after its model flag.
pub fn run_launch(model: Option<String>, claude_args: &[String]) -> Result<(), String> {
    crate::features::terminal_launch::launch(model, claude_args).map_err(|error| error.to_string())
}

/// Serves the local gateway until the app that spawned this process exits.
///
/// `report` hands the single handshake line back to the entrypoint, which is
/// the only place allowed to write it: the app reads that line to learn the
/// address and the two credentials, and nothing else ever reaches stdout.
pub fn run_gateway(report: impl FnOnce(&str)) -> Result<(), String> {
    let process = tauri::async_runtime::block_on(GatewayProcess::start())
        .map_err(|error| error.to_string())?;
    let line = process
        .handshake_line()
        .map_err(|error| error.to_string())?;
    report(&line);
    tauri::async_runtime::block_on(process.serve_until_parent_exits())
        .map_err(|error| error.to_string())
}

/// Converts a session startup message into the error Tauri shows while opening.
pub(crate) fn startup_failure(message: String) -> std::io::Error {
    std::io::Error::other(message)
}

#[cfg(test)]
#[path = "lib_test.rs"]
mod lib_test;
