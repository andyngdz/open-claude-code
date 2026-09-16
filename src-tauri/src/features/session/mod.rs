//! Desktop session that owns settings, the OpenCode Go backend, and launched processes.

mod domain;
mod errors;
mod service;

pub(crate) use service::window::{
    read_tray_action, show_settings, TrayAction, MAIN_WINDOW, QUIT_ID, SETTINGS_ID,
};

pub(crate) use domain::DashboardSnapshot;
pub(crate) use errors::SessionError;
pub(crate) use service::AppSession;
