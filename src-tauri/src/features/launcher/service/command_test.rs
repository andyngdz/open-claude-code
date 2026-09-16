use std::path::{Path, PathBuf};

use open_claude_code_backend::open_code_go_public_model_id;

use super::{command_spec_for_terminal, MODEL_ARGUMENT};
use crate::features::launcher::TerminalKind;

#[test]
fn gnome_terminal_arguments_keep_paths_as_separate_values() {
    let command = command_spec_for_terminal(
        TerminalKind::GnomeTerminal,
        PathBuf::from("/usr/bin/gnome-terminal"),
        Path::new("/tmp/a workspace"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
    );

    assert_eq!(command.program, PathBuf::from("/usr/bin/gnome-terminal"));
    assert_eq!(command.arguments[1], "/tmp/a workspace");
    assert_eq!(command.arguments[4], MODEL_ARGUMENT);
    assert_eq!(
        command.arguments[5],
        open_code_go_public_model_id("qwen3.8-max")
    );
}

#[test]
fn kitty_uses_the_selected_workspace_and_model() {
    let command = command_spec_for_terminal(
        TerminalKind::Kitty,
        PathBuf::from("/usr/bin/kitty"),
        Path::new("/tmp/project"),
        Path::new("/opt/bin/claude"),
        "minimax-m3",
    );

    assert_eq!(command.arguments[0], "--directory");
    assert_eq!(command.arguments[1], "/tmp/project");
    assert_eq!(
        command.arguments[4],
        open_code_go_public_model_id("minimax-m3")
    );
}
