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

#[cfg(windows)]
mod console {
    //! Console attachment for the CLI entry path.

    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn AttachConsole(process_id: u32) -> i32;
    }

    /// Gives the release build a console when a terminal ran the CLI subcommand.
    ///
    /// The release binary is `windows_subsystem = "windows"`, so Windows starts
    /// it with no console: stdin is not a terminal and stderr is discarded,
    /// which turns the model prompt into a silent exit 1. Attaching to the
    /// launching terminal is enough because Rust resolves the standard handles
    /// through `GetStdHandle` on every read and write, and a child process
    /// inherits the handles this call installs. A GUI launch has no parent
    /// console; the failure leaves the handles as they were.
    pub(crate) fn attach_parent_console() {
        // Rule Ignore: RUST011
        // Reason: one kernel32 call whose only failure mode is the GUI launch, where the process keeps the handles it already had.
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }
}

fn main() -> Result<(), tauri::Error> {
    // Dock/Finder launches get launchd's minimal PATH. Rebuild PATH from the
    // login shell so Claude Code and terminal binaries remain resolvable.
    fix_path_env::fix().ok();

    let cli = DesktopCli::parse();
    if let Some(DesktopCommand::Launch { model }) = cli.command {
        #[cfg(windows)]
        console::attach_parent_console();
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
