use std::{net::Ipv4Addr, sync::Arc, time::Duration};

use secrecy::SecretString;
use tokio::{
    sync::{oneshot, Mutex, RwLock},
    task::JoinHandle,
};
use uuid::Uuid;

use super::routes::{gateway_router, GatewayHttpState};
pub(crate) use crate::features::gateway::GatewayConfiguration;
use crate::{
    features::providers::{ModelCatalogEntry, ProviderRegistry},
    interface::GatewayError,
};

/// Owns the bound gateway address, local credential, and server task.
pub struct GatewayRuntime {
    base_url: String,
    local_token: SecretString,
    configuration: Arc<RwLock<GatewayConfiguration>>,
    session_id: Arc<RwLock<String>>,
    shutdown_sender: std::sync::Mutex<Option<oneshot::Sender<()>>>,
    server_task: Mutex<Option<JoinHandle<Result<(), std::io::Error>>>>,
}

impl GatewayRuntime {
    /// Returns the loopback base URL passed to Claude Code.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the in-memory token passed only to the launched Claude process.
    pub fn local_token(&self) -> &SecretString {
        &self.local_token
    }

    /// Replaces published models and the fallback session for later requests.
    pub(crate) async fn configure(
        &self,
        catalog: Vec<ModelCatalogEntry>,
        custom_models: Vec<String>,
        session_id: String,
    ) {
        let mut configuration = self.configuration.write().await;
        configuration.catalog = catalog;
        configuration.custom_models = custom_models;
        drop(configuration);
        *self.session_id.write().await = session_id;
    }

    /// Gracefully stops the Axum server and joins its owned task.
    pub async fn stop(&self) -> Result<(), GatewayError> {
        let shutdown_sender = self
            .shutdown_sender
            .lock()
            .map_err(|_poisoned| GatewayError::LifecycleUnavailable)?
            .take();
        if let Some(sender) = shutdown_sender {
            sender.send(()).ok();
        }

        let server_task = self.server_task.lock().await.take();
        let Some(server_task) = server_task else {
            return Ok(());
        };
        let joined = tokio::time::timeout(Duration::from_secs(5), server_task)
            .await
            .map_err(|_elapsed| GatewayError::StopTimeout)?
            .map_err(GatewayError::Join)?;
        joined.map_err(GatewayError::Serve)
    }
}

/// Binds the gateway to an ephemeral loopback port and starts serving routes.
pub(crate) async fn start_gateway(
    registry: ProviderRegistry,
    configuration: Arc<RwLock<GatewayConfiguration>>,
    session_id: Arc<RwLock<String>>,
) -> Result<GatewayRuntime, GatewayError> {
    let local_token = SecretString::from(Uuid::new_v4().to_string());
    let state_configuration = Arc::clone(&configuration);
    let state_session_id = Arc::clone(&session_id);
    let state = GatewayHttpState {
        registry,
        configuration,
        session_id,
        local_token: local_token.clone(),
    };
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .map_err(GatewayError::Bind)?;
    let local_address = listener.local_addr().map_err(GatewayError::LocalAddress)?;
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let server_task = tokio::spawn(async move {
        axum::serve(listener, gateway_router(state))
            .with_graceful_shutdown(async move {
                shutdown_receiver.await.ok();
            })
            .await
    });

    Ok(GatewayRuntime {
        base_url: format!("http://{local_address}"),
        local_token,
        configuration: state_configuration,
        session_id: state_session_id,
        shutdown_sender: std::sync::Mutex::new(Some(shutdown_sender)),
        server_task: Mutex::new(Some(server_task)),
    })
}

#[cfg(test)]
#[path = "runtime_test.rs"]
mod runtime_test;
