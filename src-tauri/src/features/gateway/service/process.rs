use open_claude_code_backend::OpenCodeGoBackend;
use secrecy::ExposeSecret;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

use super::catalog::published_catalog;
use crate::features::gateway::{GatewayChildError, GatewayHandshake};
use crate::features::settings::SettingsStore;

/// The gateway running as its own process, owned by the app that spawned it.
pub(crate) struct GatewayProcess {
    backend: OpenCodeGoBackend,
}

impl GatewayProcess {
    /// Starts the backend from the settings the app already persisted.
    pub(crate) async fn start() -> Result<Self, GatewayChildError> {
        let settings = SettingsStore::for_application()
            .map_err(|_source| GatewayChildError::Settings)?
            .load()
            .map_err(|_source| GatewayChildError::Settings)?;
        let custom_models = settings.custom_models.clone();
        let backend = OpenCodeGoBackend::start(published_catalog(&settings), custom_models.clone())
            .await
            .map_err(|error| GatewayChildError::Backend(error.to_string()))?;
        publish_saved_credential(&backend, &custom_models).await;

        Ok(Self { backend })
    }

    /// Returns the one line the app reads to reach this gateway.
    ///
    /// The line is the only thing this process ever writes to stdout, so the
    /// pipe the app reads stays empty of anything else.
    pub(crate) fn handshake_line(&self) -> Result<String, GatewayChildError> {
        GatewayHandshake {
            base_url: self.backend.gateway_base_url().to_owned(),
            token: self.backend.gateway_token().expose_secret().to_owned(),
            control_token: self.backend.control_token().expose_secret().to_owned(),
        }
        .to_line()
    }

    /// Serves until the app that spawned this process exits, then stops.
    ///
    /// The app holds the write end of this process's stdin, so the app leaving,
    /// killed or not, ends the read below and takes the gateway with it.
    pub(crate) async fn serve_until_parent_exits(self) -> Result<(), GatewayChildError> {
        await_stdin_end().await?;
        self.backend
            .stop()
            .await
            .map_err(|error| GatewayChildError::Backend(error.to_string()))
    }
}

/// Warms the catalog with the stored credential before the app asks for it.
///
/// The catalog only reaches the app through the gateway configuration, so the
/// refresh has to be published here for a launch made right after startup to
/// see every model the credential unlocks. A credential that is missing or
/// refused leaves the catalog the settings declared.
async fn publish_saved_credential(backend: &OpenCodeGoBackend, custom_models: &[String]) {
    let Ok(api_key) = backend.load_saved_api_key().await else {
        return;
    };
    let Ok(catalog) = backend.save_api_key(api_key.expose_secret()).await else {
        return;
    };
    backend
        .configure_gateway(catalog, custom_models.to_vec(), Uuid::new_v4().to_string())
        .await;
}

/// Blocks until the app closes the read end of this process's stdin.
async fn await_stdin_end() -> Result<(), GatewayChildError> {
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    loop {
        match stdin.read_line(&mut line).await {
            Ok(0) => return Ok(()),
            Ok(_read) => line.clear(),
            Err(source) => return Err(GatewayChildError::ParentPipe(source)),
        }
    }
}
