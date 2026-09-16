//! Terminal discovery and process spawning services.

mod command;
mod discovery;
mod proxy;

pub(crate) use command::{launch_claude, new_launch_session_id};
pub(crate) use discovery::list_available_terminals;
pub(crate) use proxy::{apply_proxy_env, claude_executable, claude_proxy_env};
