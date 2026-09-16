#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};

/// Desktop entrypoint for Open Claude Code.
#[derive(Parser)]
#[command(name = open_claude_code_lib::APP_NAME)]
struct DesktopCli {
    #[command(subcommand)]
    command: Option<DesktopCommand>,
}

#[derive(Subcommand)]
enum DesktopCommand {
    /// Launch Claude Code in this terminal.
    Launch {
        /// Model id. Skips the terminal prompt.
        #[arg(long)]
        model: Option<String>,
    },
}

fn main() -> Result<(), tauri::Error> {
    let cli = DesktopCli::parse();
    if let Some(DesktopCommand::Launch { model }) = cli.command {
        if let Err(error) = open_claude_code_lib::run_launch(model) {
            eprintln!("{error}");
            std::process::exit(1);
        }
        return Ok(());
    }
    // NVIDIA Wayland + WebKitGTK exits with Gdk protocol error 71 unless this is set
    // before the webview opens. https://v2.tauri.app/develop/debug/linux-graphics/
    #[cfg(target_os = "linux")]
    std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
    open_claude_code_lib::run()
}
