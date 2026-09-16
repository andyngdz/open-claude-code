mod features;
mod interface;

use tauri::{Manager, RunEvent};

use crate::features::session::AppSession;

/// Runs the desktop application until its event loop exits.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let session = tauri::async_runtime::block_on(AppSession::start())
                .map_err(|error| startup_failure(error.to_string()))?;
            app.manage(session);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            interface::commands::dashboard_snapshot,
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

/// Converts a session startup message into the error Tauri shows while opening.
pub(crate) fn startup_failure(message: String) -> std::io::Error {
    std::io::Error::other(message)
}

#[cfg(test)]
#[path = "lib_test.rs"]
mod lib_test;
