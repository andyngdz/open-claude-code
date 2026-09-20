use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

use super::dto::{ControlErrorBody, ControlState, CredentialPayload, GatewayPayload};
use crate::{
    constants::{
        AUTHORIZATION_HEADER, BEARER_PREFIX, CONTROL_CATALOG_REFRESH_PATH, CONTROL_CREDENTIAL_PATH,
        CONTROL_GATEWAY_PATH, CONTROL_STATE_PATH,
    },
    features::providers::ModelCatalogEntry,
    interface::ControlError,
};

/// How long one control call may take before the gateway counts as gone.
const CONTROL_TIMEOUT: Duration = Duration::from_secs(5);

/// Message shown when a rejected request carried nothing readable.
const UNREADABLE_REJECTION: &str = "The local gateway rejected the request.";

/// Drives the gateway process the desktop app spawned, over the control routes.
///
/// Only the process that spawned the gateway holds the control token, so these
/// routes stay closed to the Claude Code session launched with the gateway
/// token.
#[derive(Clone)]
pub struct ControlClient {
    http: reqwest::Client,
    base_url: String,
    token: SecretString,
}

impl ControlClient {
    /// Builds a client for a gateway that is already listening.
    pub fn new(base_url: String, token: SecretString) -> Result<Self, ControlError> {
        let http = reqwest::Client::builder()
            .timeout(CONTROL_TIMEOUT)
            .build()
            .map_err(ControlError::Unreachable)?;
        Ok(Self {
            http,
            base_url,
            token,
        })
    }

    /// Returns the provider connection state and the models the gateway serves.
    pub async fn state(&self) -> Result<ControlState, ControlError> {
        self.json(self.http.get(self.url(CONTROL_STATE_PATH))).await
    }

    /// Returns the stored API key, or an empty string when none is saved.
    ///
    /// The provider refuses to store an empty key, so an empty string here means
    /// exactly one thing: no credential is saved.
    pub async fn saved_api_key(&self) -> Result<String, ControlError> {
        let request = self.http.get(self.url(CONTROL_CREDENTIAL_PATH));
        let payload: CredentialPayload = self.json(request).await?;
        Ok(payload.api_key)
    }

    /// Validates and stores an API key, then returns the refreshed catalog.
    pub async fn save_api_key(
        &self,
        api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, ControlError> {
        let payload = CredentialPayload {
            api_key: api_key.to_owned(),
        };
        let request = self.http.put(self.url(CONTROL_CREDENTIAL_PATH));
        self.json(request.json(&payload)).await
    }

    /// Removes the stored provider credential.
    pub async fn remove_credential(&self) -> Result<(), ControlError> {
        let request = self.http.delete(self.url(CONTROL_CREDENTIAL_PATH));
        self.empty(request).await
    }

    /// Refreshes the provider model catalog with the stored credential.
    pub async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, ControlError> {
        let request = self.http.post(self.url(CONTROL_CATALOG_REFRESH_PATH));
        self.json(request).await
    }

    /// Publishes the models and fallback session the gateway serves next.
    pub async fn publish_gateway(
        &self,
        catalog: &[ModelCatalogEntry],
        custom_models: &[String],
        session_id: &str,
    ) -> Result<(), ControlError> {
        let payload = GatewayPayload {
            catalog: catalog.to_vec(),
            custom_models: custom_models.to_vec(),
            session_id: session_id.to_owned(),
        };
        let request = self.http.put(self.url(CONTROL_GATEWAY_PATH));
        self.empty(request.json(&payload)).await
    }

    /// Reads a JSON response body, turning a rejection into a typed error.
    async fn json<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, ControlError> {
        let response = self.send(request).await?;
        response.json().await.map_err(ControlError::Unexpected)
    }

    /// Sends a request whose success carries no body.
    async fn empty(&self, request: reqwest::RequestBuilder) -> Result<(), ControlError> {
        self.send(request).await.map(|_response| ())
    }

    /// Sends one authenticated control request.
    async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ControlError> {
        let response = self
            .authenticated(request)
            .send()
            .await
            .map_err(ControlError::Unreachable)?;
        match response.status().is_success() {
            true => Ok(response),
            false => Err(Self::rejection_of(response).await),
        }
    }

    /// Adds the control credential every control route requires.
    fn authenticated(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.header(AUTHORIZATION_HEADER, self.credential())
    }

    /// Returns the bearer credential in the shape the routes read.
    fn credential(&self) -> String {
        format!("{BEARER_PREFIX}{}", self.token.expose_secret())
    }

    /// Builds a control route URL on the loopback listener.
    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    /// Reads the message a rejecting route prepared for the dashboard.
    async fn rejection_of(response: reqwest::Response) -> ControlError {
        let body = response.json::<ControlErrorBody>().await;
        let message = match body {
            Ok(body) => body.error,
            Err(_source) => UNREADABLE_REJECTION.to_owned(),
        };
        ControlError::Rejected(message)
    }
}

#[cfg(test)]
#[path = "client_test.rs"]
mod client_test;
