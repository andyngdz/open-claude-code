use thiserror::Error;

use crate::{features::providers::ProviderError, interface::GatewayError};

/// Reports a user-visible failure from the OpenCode Go backend.
#[derive(Debug, Error)]
pub enum OpenCodeGoBackendError {
    /// The provider operation failed.
    #[error("{0}")]
    Provider(String),
    /// The local gateway operation failed.
    #[error("{0}")]
    Gateway(String),
}

impl From<ProviderError> for OpenCodeGoBackendError {
    fn from(source: ProviderError) -> Self {
        let message = match source {
            ProviderError::EmptyApiKey => "Enter an API key.",
            ProviderError::InvalidApiKey => "OpenCode Go rejected that API key.",
            ProviderError::CredentialNotFound => "No OpenCode Go API key is saved.",
            ProviderError::KeyringUnavailable(_) | ProviderError::KeyringTask(_) => {
                "The system keyring is unavailable."
            }
            ProviderError::Request(_) => {
                "OpenCode Go could not be reached. Check the connection and try again."
            }
            ProviderError::UnexpectedResponse | ProviderError::InvalidHeader => {
                "OpenCode Go returned an unexpected response. Try again."
            }
            ProviderError::NotRegistered(_) => "OpenCode Go is not available.",
        };
        Self::Provider(message.to_owned())
    }
}

impl From<GatewayError> for OpenCodeGoBackendError {
    fn from(source: GatewayError) -> Self {
        Self::Gateway(source.to_string())
    }
}
