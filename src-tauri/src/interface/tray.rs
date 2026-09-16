use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

use crate::features::session::{
    read_tray_action, show_settings, TrayAction, MAIN_WINDOW, QUIT_ID, SETTINGS_ID,
};

/// Adds the tray icon and keeps the gateway alive when the window closes.
pub(crate) fn install(app: &tauri::App) -> tauri::Result<()> {
    let settings = MenuItem::with_id(app, SETTINGS_ID, "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT_ID, "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings, &quit])?;
    let Some(icon) = app.default_window_icon().cloned() else {
        return Err(std::io::Error::other("tray icon is missing").into());
    };

    TrayIconBuilder::with_id(crate::constants::APP_NAME)
        .tooltip("Open Claude Code")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_settings(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match read_tray_action(event.id.as_ref()) {
            Some(TrayAction::Settings) => show_settings(app),
            Some(TrayAction::Quit) => app.exit(0),
            None => {}
        })
        .build(app)?;
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let handle = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                handle.hide().ok();
            }
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "tray_test.rs"]
mod tray_test;
