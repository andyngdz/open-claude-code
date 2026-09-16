use super::{read_tray_action, TrayAction, QUIT_ID, SETTINGS_ID};

#[test]
fn tray_ids_map_to_settings_and_quit() {
    assert!(matches!(
        read_tray_action(SETTINGS_ID),
        Some(TrayAction::Settings)
    ));
    assert!(matches!(read_tray_action(QUIT_ID), Some(TrayAction::Quit)));
    assert!(read_tray_action("other").is_none());
}
