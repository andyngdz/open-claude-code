//! Terminal discovery and Claude Code process launching.

mod domain;
mod service;

pub(crate) use domain::{
    LaunchClaudeInput, LaunchReceipt, ProcessRegistry, TerminalKind, TerminalOption,
};
pub(crate) use service::{
    apply_appimage_host_env, apply_proxy_env, claude_executable, claude_proxy_env, launch_claude,
    list_available_terminals, new_launch_session_id,
};
