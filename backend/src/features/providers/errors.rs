use thiserror::Error;

use super::ProviderId;

/// Reports provider registry, credential, and upstream transport failures.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// The requested provider is not registered.
    #[error("provider is not registered: {0}")]
    NotRegistered(ProviderId),
    /// The submitted credential contains no characters.
    #[error("provider API key is required")]
    EmptyApiKey,
    /// The operating system credential manager failed.
    #[error("system keyring is unavailable")]
    KeyringUnavailable(#[source] keyring::Error),
    /// No credential exists for the provider.
    #[error("provider credential was not found")]
    CredentialNotFound,
    /// The upstream rejected the credential.
    #[error("provider rejected the API key")]
    InvalidApiKey,
    /// The upstream HTTP request failed.
    #[error("provider request failed")]
    Request(#[source] reqwest::Error),
    /// The upstream returned an unclassified status.
    #[error("provider returned an unexpected response")]
    UnexpectedResponse,
    /// A blocking credential operation could not be joined.
    #[error("system keyring task failed")]
    KeyringTask(#[source] tokio::task::JoinError),
    /// An HTTP header could not cross the provider boundary.
    #[error("provider returned an invalid HTTP header")]
    InvalidHeader,
}
