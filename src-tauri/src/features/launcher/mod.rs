//! Linux terminal discovery and Claude Code process launching.

mod domain;
mod service;

pub(crate) use domain::{
    LaunchClaudeInput, LaunchReceipt, ProcessRegistry, TerminalKind, TerminalOption,
};
pub(crate) use service::{launch_claude, list_available_terminals, new_launch_session_id};
