use std::{net::Ipv4Addr, sync::Arc};

use async_trait::async_trait;
use reqwest::{Client, Method, StatusCode};
use secrecy::SecretString;
use serde_json::{json, Value};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::interface::gateway::{
    routes::{gateway_router, GatewayHttpState},
    runtime::GatewayPublisher,
};
use crate::{
    constants::{
        AUTHORIZATION_HEADER, CONTROL_CATALOG_REFRESH_PATH, CONTROL_CREDENTIAL_PATH,
        CONTROL_GATEWAY_PATH, CONTROL_STATE_PATH, HEALTH_PATH,
    },
    features::{
        gateway::GatewayConfiguration,
        providers::{
            AuthMethod, ModelCatalogEntry, Provider, ProviderConnectionState, ProviderDescriptor,
            ProviderError, ProviderId, ProviderProtocol, ProviderRegistry, ProviderRequest,
            ProviderResponse,
        },
    },
};

const PROVIDER_ID: &str = "test-provider";
const LOCAL_TOKEN: &str = "local-token";
const CONTROL_TOKEN: &str = "control-token";
const HARNESS_API_KEY: &str = "sk-saved";

#[tokio::test]
async fn every_control_route_rejects_a_credential_that_is_not_the_control_token() {
    let harness = GatewayHarness::start(FakeProvider::connected(HARNESS_API_KEY)).await;

    for (method, path, body) in control_requests() {
        let anonymous = harness.send(method.clone(), path, None, body.clone()).await;
        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED, "{path}");

        let data_plane = harness.send(method, path, Some(LOCAL_TOKEN), body).await;
        assert_eq!(data_plane.status(), StatusCode::UNAUTHORIZED, "{path}");
    }
}

#[tokio::test]
async fn the_published_gateway_is_reported_back_by_the_state_route() {
    let harness = GatewayHarness::start(FakeProvider::connected(HARNESS_API_KEY)).await;

    let published = harness
        .send(
            Method::PUT,
            CONTROL_GATEWAY_PATH,
            Some(CONTROL_TOKEN),
            Some(json!({
                "catalog": house_catalog(),
                "customModels": ["local-model"],
                "sessionId": "session-1"
            })),
        )
        .await;
    assert_eq!(published.status(), StatusCode::NO_CONTENT);

    let state = harness
        .send(Method::GET, CONTROL_STATE_PATH, Some(CONTROL_TOKEN), None)
        .await;
    assert_eq!(state.status(), StatusCode::OK);
    let body: Value = state.json().await.expect("state should carry JSON");
    assert_eq!(body["catalog"], json!(house_catalog()));
    assert_eq!(body["connection"]["status"], "connected");
}

#[tokio::test]
async fn the_credential_routes_round_trip_the_saved_key() {
    let harness = GatewayHarness::start(FakeProvider::connected(HARNESS_API_KEY)).await;

    let saved = harness
        .send(
            Method::GET,
            CONTROL_CREDENTIAL_PATH,
            Some(CONTROL_TOKEN),
            None,
        )
        .await;
    assert_eq!(saved.status(), StatusCode::OK);
    let body: Value = saved.json().await.expect("credential should carry JSON");
    assert_eq!(body["apiKey"], HARNESS_API_KEY);

    let removed = harness
        .send(
            Method::DELETE,
            CONTROL_CREDENTIAL_PATH,
            Some(CONTROL_TOKEN),
            None,
        )
        .await;
    assert_eq!(removed.status(), StatusCode::NO_CONTENT);

    let refreshed = harness
        .send(
            Method::POST,
            CONTROL_CATALOG_REFRESH_PATH,
            Some(CONTROL_TOKEN),
            None,
        )
        .await;
    assert_eq!(refreshed.status(), StatusCode::OK);
    let catalog: Value = refreshed.json().await.expect("catalog should carry JSON");
    assert_eq!(catalog, json!(house_catalog()));
}

#[tokio::test]
async fn an_absent_credential_reads_back_as_an_empty_key() {
    let harness = GatewayHarness::start(FakeProvider::disconnected()).await;

    let response = harness
        .send(
            Method::GET,
            CONTROL_CREDENTIAL_PATH,
            Some(CONTROL_TOKEN),
            None,
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = response.json().await.expect("credential should carry JSON");
    assert_eq!(body["apiKey"], "");
}

#[tokio::test]
async fn a_provider_failure_comes_back_as_a_ready_to_show_message() {
    let harness = GatewayHarness::start(FakeProvider::failing()).await;

    let response = harness
        .send(
            Method::POST,
            CONTROL_CATALOG_REFRESH_PATH,
            Some(CONTROL_TOKEN),
            None,
        )
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body: Value = response.json().await.expect("failure should carry JSON");
    let message = body["error"].as_str().unwrap_or_default();
    assert!(!message.is_empty(), "the dashboard needs a message to show");
}

#[tokio::test]
async fn the_health_probe_answers_without_a_token_and_the_data_plane_needs_one() {
    let harness = GatewayHarness::start(FakeProvider::connected(HARNESS_API_KEY)).await;

    let probe = harness.send(Method::HEAD, HEALTH_PATH, None, None).await;
    assert_eq!(probe.status(), StatusCode::NO_CONTENT);

    let rejected = harness
        .send(Method::GET, "/v1/models", Some(CONTROL_TOKEN), None)
        .await;
    assert_eq!(rejected.status(), StatusCode::UNAUTHORIZED);

    let discovered = harness
        .send(Method::GET, "/v1/models", Some(LOCAL_TOKEN), None)
        .await;
    assert_eq!(discovered.status(), StatusCode::OK);
}

/// Returns every control request as method, path, and the body its route reads.
fn control_requests() -> Vec<(Method, &'static str, Option<Value>)> {
    vec![
        (Method::GET, CONTROL_STATE_PATH, None),
        (Method::GET, CONTROL_CREDENTIAL_PATH, None),
        (
            Method::PUT,
            CONTROL_CREDENTIAL_PATH,
            Some(json!({ "apiKey": "sk-submitted" })),
        ),
        (Method::DELETE, CONTROL_CREDENTIAL_PATH, None),
        (Method::POST, CONTROL_CATALOG_REFRESH_PATH, None),
        (
            Method::PUT,
            CONTROL_GATEWAY_PATH,
            Some(json!({
                "catalog": house_catalog(),
                "customModels": [],
                "sessionId": "session-1"
            })),
        ),
    ]
}

fn house_catalog() -> Value {
    json!([{ "id": "qwen3.8-max", "displayName": "Qwen 3.8 Max", "isCustom": false }])
}

/// Serves the gateway routes on a loopback port for the lifetime of the test.
struct GatewayHarness {
    base_url: String,
    server: JoinHandle<()>,
}

impl GatewayHarness {
    async fn start(provider: FakeProvider) -> Self {
        let publisher = GatewayPublisher::new(
            GatewayConfiguration {
                provider_id: ProviderId::new(PROVIDER_ID),
                catalog: Vec::new(),
                custom_models: Vec::new(),
            },
            "fallback-session".to_owned(),
        );
        let state = GatewayHttpState {
            registry: ProviderRegistry::new(vec![Arc::new(provider)]),
            publisher,
            local_token: SecretString::from(LOCAL_TOKEN.to_owned()),
            control_token: SecretString::from(CONTROL_TOKEN.to_owned()),
        };
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("loopback port should bind");
        let address = listener
            .local_addr()
            .expect("a bound listener should report its address");
        let server = tokio::spawn(async move {
            axum::serve(listener, gateway_router(state))
                .await
                .expect("the test server should serve until the test ends");
        });

        Self {
            base_url: format!("http://{address}"),
            server,
        }
    }

    async fn send(
        &self,
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<Value>,
    ) -> reqwest::Response {
        let mut request = Client::new().request(method, format!("{}{path}", self.base_url));
        if let Some(token) = token {
            request = request.header(AUTHORIZATION_HEADER, format!("Bearer {token}"));
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        request
            .send()
            .await
            .expect("the test server should answer every request")
    }
}

impl Drop for GatewayHarness {
    fn drop(&mut self) {
        self.server.abort();
    }
}

/// Answers provider calls with the credential and catalog one test hands it.
struct FakeProvider {
    descriptor: ProviderDescriptor,
    api_key: Option<&'static str>,
    fails: bool,
}

impl FakeProvider {
    fn connected(api_key: &'static str) -> Self {
        Self::new(Some(api_key), false)
    }

    fn disconnected() -> Self {
        Self::new(None, false)
    }

    fn failing() -> Self {
        Self::new(None, true)
    }

    fn new(api_key: Option<&'static str>, fails: bool) -> Self {
        Self {
            descriptor: ProviderDescriptor {
                id: ProviderId::new(PROVIDER_ID),
                display_name: "Test".to_owned(),
                auth_methods: vec![AuthMethod::ApiKey],
                protocols: vec![ProviderProtocol::AnthropicMessages],
            },
            api_key,
            fails,
        }
    }
}

#[async_trait]
impl Provider for FakeProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    async fn connection_state(&self) -> ProviderConnectionState {
        if self.fails {
            return ProviderConnectionState::Failed {
                message: "keyring is unavailable".to_owned(),
            };
        }
        match self.api_key {
            Some(_) => ProviderConnectionState::Connected,
            None => ProviderConnectionState::Disconnected,
        }
    }

    async fn load_saved_api_key(&self) -> Result<SecretString, ProviderError> {
        if self.fails {
            return Err(ProviderError::UnexpectedResponse);
        }
        self.api_key
            .map(|api_key| SecretString::from(api_key.to_owned()))
            .ok_or(ProviderError::CredentialNotFound)
    }

    async fn save_and_test_api_key(
        &self,
        _api_key: &str,
    ) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        if self.fails {
            return Err(ProviderError::UnexpectedResponse);
        }
        Ok(house_models())
    }

    async fn remove_credential(&self) -> Result<(), ProviderError> {
        match self.fails {
            true => Err(ProviderError::UnexpectedResponse),
            false => Ok(()),
        }
    }

    async fn refresh_catalog(&self) -> Result<Vec<ModelCatalogEntry>, ProviderError> {
        if self.fails {
            return Err(ProviderError::UnexpectedResponse);
        }
        Ok(house_models())
    }

    async fn forward_messages(
        &self,
        _request: ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        Err(ProviderError::UnexpectedResponse)
    }
}

fn house_models() -> Vec<ModelCatalogEntry> {
    vec![ModelCatalogEntry {
        id: "qwen3.8-max".to_owned(),
        display_name: "Qwen 3.8 Max".to_owned(),
        is_custom: false,
    }]
}
