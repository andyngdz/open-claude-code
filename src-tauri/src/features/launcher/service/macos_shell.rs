use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use open_claude_code_backend::open_code_go_public_model_id;
use uuid::Uuid;

use super::proxy::ClaudeProxyEnv;
use super::terminal_args::{sh_quote, MODEL_ARGUMENT};
use crate::features::errors::LauncherError;

/// Builds an export-and-exec shell line for AppleScript `do script` / `write text`.
pub(super) fn shell_launch_command(
    workspace: &Path,
    claude_path: &Path,
    model_id: &str,
    proxy_env: &ClaudeProxyEnv,
) -> String {
    let public_model = open_code_go_public_model_id(model_id);
    let mut parts = proxy_exports(proxy_env);
    parts.push(format!("cd {}", sh_quote(&workspace.to_string_lossy())));
    parts.push(format!(
        "exec {} {} {}",
        sh_quote(&claude_path.to_string_lossy()),
        MODEL_ARGUMENT,
        sh_quote(&public_model)
    ));
    parts.join(" && ")
}

/// Writes an executable temp script that exports proxy env and runs the CLI.
pub(super) fn write_launch_script(
    workspace: &Path,
    claude_path: &Path,
    public_model: &str,
    proxy_env: &ClaudeProxyEnv,
) -> Result<PathBuf, LauncherError> {
    let mut parts = vec!["#!/bin/zsh".to_owned()];
    parts.extend(proxy_exports(proxy_env));
    parts.push(format!("cd {}", sh_quote(&workspace.to_string_lossy())));
    parts.push(format!(
        "exec {} {} {}",
        sh_quote(&claude_path.to_string_lossy()),
        MODEL_ARGUMENT,
        sh_quote(public_model)
    ));
    let path = std::env::temp_dir().join(format!("open-claude-launch-{}.command", Uuid::new_v4()));
    fs::write(&path, parts.join("\n") + "\n").map_err(LauncherError::Spawn)?;
    let mut permissions = fs::metadata(&path)
        .map_err(LauncherError::Spawn)?
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).map_err(LauncherError::Spawn)?;
    Ok(path)
}

/// Escapes a value for embedding inside an AppleScript double-quoted string.
pub(super) fn applescript_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn proxy_exports(proxy_env: &ClaudeProxyEnv) -> Vec<String> {
    vec![
        format!(
            "export ANTHROPIC_BASE_URL={}",
            sh_quote(&proxy_env.base_url)
        ),
        format!("export ANTHROPIC_AUTH_TOKEN={}", sh_quote(&proxy_env.token)),
        "export CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1".to_owned(),
        format!(
            "export ANTHROPIC_DEFAULT_FABLE_MODEL={}",
            sh_quote(&proxy_env.fable)
        ),
        format!(
            "export ANTHROPIC_DEFAULT_OPUS_MODEL={}",
            sh_quote(&proxy_env.opus)
        ),
        format!(
            "export ANTHROPIC_DEFAULT_SONNET_MODEL={}",
            sh_quote(&proxy_env.sonnet)
        ),
        format!(
            "export ANTHROPIC_DEFAULT_HAIKU_MODEL={}",
            sh_quote(&proxy_env.haiku)
        ),
    ]
}

#[cfg(test)]
#[path = "macos_shell_test.rs"]
mod macos_shell_test;
