//! Terminal discovery and process spawning services.

mod command;
mod discovery;
#[cfg(target_os = "macos")]
mod macos_launch;
#[cfg(target_os = "macos")]
mod macos_shell;
mod proxy;
mod terminal_args;

pub(crate) use command::{launch_claude, new_launch_session_id};
pub(crate) use discovery::list_available_terminals;
pub(crate) use proxy::{apply_proxy_env, claude_executable, claude_proxy_env};
