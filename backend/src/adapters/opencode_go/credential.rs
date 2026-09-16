use keyring::Entry;
use secrecy::{ExposeSecret, SecretString};

use crate::features::providers::ProviderError;

const KEYRING_SERVICE: &str = "dev.andyng.openclaudecode";
const KEYRING_ACCOUNT: &str = "provider:opencode-go";

/// Stores OpenCode Go credentials in the operating system keyring.
#[derive(Clone, Debug, Default)]
pub(super) struct OpenCodeGoCredentialStore;

impl OpenCodeGoCredentialStore {
    /// Stores or replaces the credential without returning its value.
    pub(super) async fn save(&self, api_key: SecretString) -> Result<(), ProviderError> {
        tokio::task::spawn_blocking(move || {
            entry()?
                .set_password(api_key.expose_secret())
                .map_err(ProviderError::KeyringUnavailable)
        })
        .await
        .map_err(ProviderError::KeyringTask)?
    }

    /// Loads the credential for an upstream provider request.
    pub(super) async fn load(&self) -> Result<SecretString, ProviderError> {
        tokio::task::spawn_blocking(move || match entry()?.get_password() {
            Ok(api_key) => Ok(SecretString::from(api_key)),
            Err(keyring::Error::NoEntry) => Err(ProviderError::CredentialNotFound),
            Err(source) => Err(ProviderError::KeyringUnavailable(source)),
        })
        .await
        .map_err(ProviderError::KeyringTask)?
    }

    /// Removes the credential when present.
    pub(super) async fn remove(&self) -> Result<(), ProviderError> {
        tokio::task::spawn_blocking(move || match entry()?.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(source) => Err(ProviderError::KeyringUnavailable(source)),
        })
        .await
        .map_err(ProviderError::KeyringTask)?
    }
}

fn entry() -> Result<Entry, ProviderError> {
    Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).map_err(ProviderError::KeyringUnavailable)
}
