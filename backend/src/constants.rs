//! Shared protocol literals used by more than one backend module.

/// Header that carries the OpenCode Go API key.
pub(crate) const API_KEY_HEADER: &str = "x-api-key";
/// Header that carries the local gateway bearer token.
pub(crate) const AUTHORIZATION_HEADER: &str = "authorization";
/// JSON field that names the requested model.
pub(crate) const MODEL_FIELD: &str = "model";
