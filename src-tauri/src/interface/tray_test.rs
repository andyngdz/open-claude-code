use crate::features::session::{read_tray_action, TrayAction, SETTINGS_ID};

#[test]
fn installer_menu_ids_match_the_parser() {
    assert!(matches!(
        read_tray_action(SETTINGS_ID),
        Some(TrayAction::Settings)
    ));
}
