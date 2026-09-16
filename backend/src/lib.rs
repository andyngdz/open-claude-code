//! OpenCode Go provider and Claude-compatible local gateway.

mod adapters;
mod constants;
mod errors;
mod features;
mod interface;
mod runtime;

pub use errors::OpenCodeGoBackendError;
pub use features::providers::{ModelCatalogEntry, ProviderConnectionState};
pub use runtime::{open_code_go_public_model_id, OpenCodeGoBackend};

/// Provider model used for Claude Code's primary model aliases by default.
pub const DEFAULT_PRIMARY_MODEL_ID: &str = "qwen3.8-max";
/// Provider model used for Claude Code's fast model alias by default.
pub const DEFAULT_FAST_MODEL_ID: &str = "qwen3.8-flash";
