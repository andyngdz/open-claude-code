use thiserror::Error;

/// Reports a request that cannot be routed to a configured provider model.
#[derive(Debug, Error)]
pub(crate) enum GatewayRequestError {
    #[error("Request body must be valid JSON")]
    InvalidJson,
    #[error("Request model is required")]
    MissingModel,
    #[error("Request model is not published by this gateway")]
    InvalidPublicModel,
    #[error("Request model is not configured")]
    ModelNotConfigured,
    #[error("Request body could not be normalized")]
    Serialization,
}
