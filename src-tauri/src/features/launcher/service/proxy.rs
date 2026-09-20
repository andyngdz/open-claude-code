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

/// One proxy setting, named so the process env and the macOS wrapper cannot drift.
pub(super) struct ProxyVariable<'a> {
    pub(super) name: &'static str,
    pub(super) value: &'a str,
}

/// Lists every proxy variable Claude Code needs, in the order the launch script exports them.
pub(super) fn proxy_variables(proxy_env: &ClaudeProxyEnv) -> Vec<ProxyVariable<'_>> {
    vec![
        ProxyVariable {
            name: "ANTHROPIC_BASE_URL",
            value: &proxy_env.base_url,
        },
        ProxyVariable {
            name: "ANTHROPIC_AUTH_TOKEN",
            value: &proxy_env.token,
        },
        ProxyVariable {
            name: "CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY",
            value: "1",
        },
        ProxyVariable {
            name: "ANTHROPIC_DEFAULT_FABLE_MODEL",
            value: &proxy_env.fable,
        },
        ProxyVariable {
            name: "ANTHROPIC_DEFAULT_OPUS_MODEL",
            value: &proxy_env.opus,
        },
        ProxyVariable {
            name: "ANTHROPIC_DEFAULT_SONNET_MODEL",
            value: &proxy_env.sonnet,
        },
        ProxyVariable {
            name: "ANTHROPIC_DEFAULT_HAIKU_MODEL",
            value: &proxy_env.haiku,
        },
    ]
}

/// Applies proxy variables to a process that will run Claude Code.
pub(crate) fn apply_proxy_env(command: &mut Command, proxy_env: &ClaudeProxyEnv) {
    command.envs(
        proxy_variables(proxy_env)
            .into_iter()
            .map(|variable| (variable.name, variable.value)),
    );
}

#[cfg(test)]
#[path = "proxy_test.rs"]
mod proxy_test;
