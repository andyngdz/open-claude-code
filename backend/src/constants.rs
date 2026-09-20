//! Shared protocol literals used by more than one backend module.

/// Header that carries the OpenCode Go API key.
pub(crate) const API_KEY_HEADER: &str = "x-api-key";
/// Header that carries the local gateway bearer token.
pub(crate) const AUTHORIZATION_HEADER: &str = "authorization";
/// Prefix a bearer credential carries in the authorization header.
pub(crate) const BEARER_PREFIX: &str = "Bearer ";
/// Unauthenticated route that answers whether a gateway is serving.
pub(crate) const HEALTH_PATH: &str = "/api/hello";
/// JSON field that names the requested model.
pub(crate) const MODEL_FIELD: &str = "model";
/// Control route that reports connection state and published models.
pub(crate) const CONTROL_STATE_PATH: &str = "/control/state";
/// Control route that reads, replaces, or removes the stored credential.
pub(crate) const CONTROL_CREDENTIAL_PATH: &str = "/control/credential";
/// Control route that refreshes the provider model catalog.
pub(crate) const CONTROL_CATALOG_REFRESH_PATH: &str = "/control/catalog/refresh";
/// Control route that publishes the models and session the gateway serves.
pub(crate) const CONTROL_GATEWAY_PATH: &str = "/control/gateway";
