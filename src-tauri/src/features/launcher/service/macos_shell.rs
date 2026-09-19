use std::{
    fs,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

use uuid::Uuid;

use super::proxy::{proxy_variables, ClaudeProxyEnv};
use super::terminal_args::{sh_quote, MODEL_ARGUMENT};
use crate::features::errors::LauncherError;

/// Writes a private, one-time launch script that exports proxy env and runs the CLI.
pub(super) fn write_launch_script(
    workspace: &Path,
    claude_path: &Path,
    public_model: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<PathBuf, LauncherError> {
    let mut parts = vec![
        "#!/bin/zsh".to_owned(),
        "rm -f -- \"$0\" || exit 1".to_owned(),
    ];
    parts.extend(proxy_exports(proxy_env));
    let mut workspace_command = format!("cd {}", sh_quote(&workspace.to_string_lossy()));
    workspace_command.push_str(" || exit 1");
    parts.push(workspace_command);
    parts.push(format!(
        "exec {} {} {}",
        sh_quote(&claude_path.to_string_lossy()),
        MODEL_ARGUMENT,
        sh_quote(public_model)
    ));
    let path = std::env::temp_dir().join(format!("open-claude-launch-{}.command", Uuid::new_v4()));
    let mut script = fs::OpenOptions::new()
        .create_new(true)
        .mode(0o700)
        .write(true)
        .open(&path)
        .map_err(LauncherError::Spawn)?;
    if let Err(source) = script
        .write_all(parts.join("\n").as_bytes())
        .and_then(|()| script.write_all(b"\n"))
    {
        drop(script);
        fs::remove_file(&path).ok();
        return Err(LauncherError::Spawn(source));
    }
    Ok(path)
}

/// Escapes a value for embedding inside an AppleScript double-quoted string.
pub(super) fn applescript_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn proxy_exports(proxy_env: &ClaudeProxyEnv) -> Vec<String> {
    proxy_variables(proxy_env)
        .into_iter()
        .map(|variable| format!("export {}={}", variable.name, sh_quote(variable.value)))
        .collect()
}

#[cfg(test)]
#[path = "macos_shell_test.rs"]
mod macos_shell_test;
