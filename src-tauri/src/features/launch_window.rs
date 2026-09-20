//! Resolves the context window a launch asks Claude Code to declare.
//!
//! A launch declares 1M when a ticked row points at the model it starts. The
//! rows are the four aliases plus the Default model row, which points at
//! `launch_model_id`, so it keeps its own declaration beside that id. The rule
//! takes the rows apart instead of whole settings because the dashboard holds
//! settings while the CLI holds the handshake, and both must answer the same.

use open_claude_code_backend::ContextWindow;

use crate::features::settings::ModelAliasMapping;

/// Returns the window the Default model row declares for a model.
///
/// The row points at the model the dashboard launches, so a launch of any
/// other model reads as `ContextWindow::Standard`.
pub(crate) fn default_row_window(
    launch_model_id: Option<&str>,
    declared: ContextWindow,
    model_id: &str,
) -> ContextWindow {
    if launch_model_id == Some(model_id) {
        return declared;
    }
    ContextWindow::Standard
}

/// Returns the window every ticked launch row declares for a model.
///
/// The wider declaration wins, so unticking one row cannot take 1M away from a
/// model another row asks for.
pub(crate) fn declared_window(
    aliases: &ModelAliasMapping,
    launch_model_id: Option<&str>,
    launch_extended_context: ContextWindow,
    model_id: &str,
) -> ContextWindow {
    let alias_rows = aliases.declared_window(model_id);
    alias_rows.widest(default_row_window(
        launch_model_id,
        launch_extended_context,
        model_id,
    ))
}

#[cfg(test)]
#[path = "launch_window_test.rs"]
mod launch_window_test;
