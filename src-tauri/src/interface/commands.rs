use tauri::State;

use crate::features::{
    launcher::LaunchClaudeInput,
    session::{AppSession, DashboardSnapshot},
    settings::SaveProviderSettingsInput,
};

/// Returns the current non-secret dashboard state.
#[tauri::command]
pub(crate) async fn dashboard_snapshot(
    session: State<'_, AppSession>,
) -> Result<DashboardSnapshot, String> {
    Ok(session.snapshot().await)
}

/// Validates and stores an OpenCode Go API key.
#[tauri::command]
pub(crate) async fn save_api_key(
    session: State<'_, AppSession>,
    api_key: String,
) -> Result<DashboardSnapshot, String> {
    session
        .save_api_key(&api_key)
        .await
        .map_err(|error| error.to_string())
}

/// Removes the stored OpenCode Go API key.
#[tauri::command]
pub(crate) async fn remove_credential(
    session: State<'_, AppSession>,
) -> Result<DashboardSnapshot, String> {
    session
        .remove_credential()
        .await
        .map_err(|error| error.to_string())
}

/// Refreshes the OpenCode Go model catalog.
#[tauri::command]
pub(crate) async fn refresh_catalog(
    session: State<'_, AppSession>,
) -> Result<DashboardSnapshot, String> {
    session
        .refresh_catalog()
        .await
        .map_err(|error| error.to_string())
}

/// Saves terminal, alias, and custom-model settings.
#[tauri::command]
pub(crate) async fn save_dashboard_settings(
    session: State<'_, AppSession>,
    input: SaveProviderSettingsInput,
) -> Result<DashboardSnapshot, String> {
    session
        .save_settings(input)
        .await
        .map_err(|error| error.to_string())
}

/// Opens Claude Code in the selected workspace.
#[tauri::command]
pub(crate) async fn launch_claude_session(
    session: State<'_, AppSession>,
    input: LaunchClaudeInput,
) -> Result<DashboardSnapshot, String> {
    session
        .launch(input)
        .await
        .map_err(|error| error.to_string())
}
