//! OpenCode Go provider and Claude-compatible local gateway.

mod adapters;
mod constants;
mod errors;
mod features;
mod interface;
mod runtime;

pub use errors::OpenCodeGoBackendError;
pub use features::providers::{ModelCatalogEntry, ProviderConnectionState};
pub use interface::{ControlClient, ControlError, ControlState};
pub use runtime::{
    open_code_go_public_model_id, split_one_million_suffix, with_one_million_suffix, ContextWindow,
    OpenCodeGoBackend,
};

/// Provider model used for Claude Code's primary model aliases by default.
pub const DEFAULT_PRIMARY_MODEL_ID: &str = "qwen3.8-max";
/// Provider model used for Claude Code's fast model alias by default.
pub const DEFAULT_FAST_MODEL_ID: &str = "qwen3.8-flash";
/// Route a caller probes to learn whether a published gateway still answers.
///
/// Shared with the CLI, which must tell a live gateway from a handshake file the
/// previous app left behind, and which cannot import the private constants tree.
pub const GATEWAY_HEALTH_PATH: &str = constants::HEALTH_PATH;
