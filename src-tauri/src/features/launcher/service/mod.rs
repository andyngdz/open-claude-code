//! Terminal discovery and process spawning services.

mod command;
mod discovery;

pub(crate) use command::{launch_claude, new_launch_session_id};
pub(crate) use discovery::list_available_terminals;
