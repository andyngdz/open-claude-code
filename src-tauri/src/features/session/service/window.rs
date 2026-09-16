use tauri::Manager;

pub(crate) const SETTINGS_ID: &str = "settings";
pub(crate) const QUIT_ID: &str = "quit";
pub(crate) const MAIN_WINDOW: &str = "main";

/// Tray menu choice.
pub(crate) enum TrayAction {
    Settings,
    Quit,
}

/// Shows the settings window after a tray click or menu choice.
pub(crate) fn show_settings(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return;
    };
    window.unminimize().ok();
    window.show().ok();
    window.set_focus().ok();
}

/// Parses a tray menu id into a closed action.
pub(crate) fn read_tray_action(id: &str) -> Option<TrayAction> {
    if id == SETTINGS_ID {
        return Some(TrayAction::Settings);
    }
    if id == QUIT_ID {
        return Some(TrayAction::Quit);
    }
    None
}

#[cfg(test)]
#[path = "window_test.rs"]
mod window_test;
