use std::{path::PathBuf, process::Command};

use open_claude_code_backend::open_code_go_public_model_id;

use super::discovery::find_executable;
use crate::features::{errors::LauncherError, settings::ModelAliasMapping};

const CLAUDE_EXECUTABLE: &str = "claude";

/// Proxy variables given only to a Claude Code process.
pub(crate) struct ClaudeProxyEnv {
    /// Loopback gateway URL.
    pub(crate) base_url: String,
    /// In-memory gateway credential.
    pub(crate) token: String,
    /// Public id for the Fable alias.
    pub(crate) fable: String,
    /// Public id for the Opus alias.
    pub(crate) opus: String,
    /// Public id for the Sonnet alias.
    pub(crate) sonnet: String,
    /// Public id for the Haiku alias.
    pub(crate) haiku: String,
}

/// Builds the proxy environment from the saved alias mapping.
pub(crate) fn claude_proxy_env(
    proxy_base_url: &str,
    proxy_token: &str,
    aliases: &ModelAliasMapping,
) -> ClaudeProxyEnv {
    ClaudeProxyEnv {
        base_url: proxy_base_url.to_owned(),
        token: proxy_token.to_owned(),
        fable: open_code_go_public_model_id(&aliases.fable),
        opus: open_code_go_public_model_id(&aliases.opus),
        sonnet: open_code_go_public_model_id(&aliases.sonnet),
        haiku: open_code_go_public_model_id(&aliases.haiku),
    }
}

/// Resolves the Claude Code executable on PATH.
pub(crate) fn claude_executable() -> Result<PathBuf, LauncherError> {
    find_executable(CLAUDE_EXECUTABLE).ok_or(LauncherError::ClaudeNotFound)
}

/// Applies proxy variables to a process that will run Claude Code.
pub(crate) fn apply_proxy_env(command: &mut Command, proxy_env: &ClaudeProxyEnv) {
    command
        .env("ANTHROPIC_BASE_URL", &proxy_env.base_url)
        .env("ANTHROPIC_AUTH_TOKEN", &proxy_env.token)
        .env("CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY", "1")
        .env("ANTHROPIC_DEFAULT_FABLE_MODEL", &proxy_env.fable)
        .env("ANTHROPIC_DEFAULT_OPUS_MODEL", &proxy_env.opus)
        .env("ANTHROPIC_DEFAULT_SONNET_MODEL", &proxy_env.sonnet)
        .env("ANTHROPIC_DEFAULT_HAIKU_MODEL", &proxy_env.haiku);
}
