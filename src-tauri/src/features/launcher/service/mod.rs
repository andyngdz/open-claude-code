//! Terminal discovery and process spawning services.

mod appimage_env;
mod command;
mod discovery;
#[cfg(any(test, target_os = "macos"))]
mod macos_launch;
#[cfg(any(test, target_os = "macos"))]
mod macos_shell;
mod proxy;
mod terminal_args;

pub(crate) use appimage_env::apply_appimage_host_env;
pub(crate) use command::{launch_claude, new_launch_session_id};
pub(crate) use discovery::list_available_terminals;
pub(crate) use proxy::{apply_proxy_env, claude_executable, claude_proxy_env};
