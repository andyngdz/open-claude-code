use std::net::Ipv4Addr;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json, Router,
};
use secrecy::SecretString;
use serde_json::{json, Value};
use tokio::{net::TcpListener, task::JoinHandle};

use super::ControlClient;
use crate::{
    constants::{
        AUTHORIZATION_HEADER, BEARER_PREFIX, CONTROL_CATALOG_REFRESH_PATH, CONTROL_CREDENTIAL_PATH,
        CONTROL_GATEWAY_PATH, CONTROL_STATE_PATH,
    },
    features::providers::{ModelCatalogEntry, ProviderConnectionState},
    interface::ControlError,
};

const CONTROL_TOKEN: &str = "control-token";
const SAVED_API_KEY: &str = "sk-saved";

#[tokio::test]
async fn every_control_route_round_trips_the_client_contract() {
    let harness = ControlHarness::start(CannedResponse::default()).await;
    let client = harness.client(CONTROL_TOKEN);

    let state = client.state().await.expect("state should be readable");
    assert!(matches!(
        state.connection,
        ProviderConnectionState::Connected
    ));
    assert_eq!(state.catalog, house_models());

    assert_eq!(
        client
            .saved_api_key()
            .await
            .expect("key should be readable"),
        SAVED_API_KEY
    );
    assert_eq!(
        client
            .save_api_key("sk-submitted")
            .await
            .expect("key should save"),
        house_models()
    );
    client
        .remove_credential()
        .await
        .expect("credential should be removed");
    assert_eq!(
        client
            .refresh_catalog()
            .await
            .expect("catalog should refresh"),
        house_models()
    );
    client
        .publish_gateway(&house_models(), &["local-model".to_owned()], "session-1")
        .await
        .expect("gateway should accept the published models");
}

#[tokio::test]
async fn a_rejection_carries_the_message_the_route_prepared() {
    let harness = ControlHarness::start(CannedResponse {
        rejection: Some("OpenCode Go rejected that API key.".to_owned()),
        ..CannedResponse::default()
    })
    .await;

    let error = harness
        .client(CONTROL_TOKEN)
        .save_api_key("sk-wrong")
        .await
        .expect_err("a rejected key should fail");

    assert_eq!(
        error.to_string(),
        "OpenCode Go rejected that API key.",
        "the dashboard shows this text unchanged"
    );
}

#[tokio::test]
async fn an_unexpected_credential_is_reported_as_a_rejection() {
    let harness = ControlHarness::start(CannedResponse::default()).await;

    let error = harness
        .client("wrong-token")
        .state()
        .await
        .expect_err("the route should refuse the wrong token");

    assert!(matches!(error, ControlError::Rejected(_)));
}

#[tokio::test]
async fn a_gateway_that_is_not_listening_is_reported_as_unreachable() {
    let client = ControlClient::new(
        "http://127.0.0.1:1".to_owned(),
        SecretString::from(CONTROL_TOKEN.to_owned()),
    )
    .expect("the client should build");

    let error = client
        .state()
        .await
        .expect_err("nothing listens on that port");

    assert!(matches!(error, ControlError::Unreachable(_)));
}

/// Canned answers, and the request bodies the client is expected to send.
#[derive(Clone)]
struct CannedResponse {
    /// When set, the credential route rejects with this message instead.
    rejection: Option<String>,
    /// Body the gateway route must receive.
    expected_publish: Value,
    /// Body the credential route must receive.
    expected_credential: Value,
}

impl Default for CannedResponse {
    fn default() -> Self {
        Self {
            rejection: None,
            expected_publish: json!({
                "catalog": house_catalog(),
                "customModels": ["local-model"],
                "sessionId": "session-1"
            }),
            expected_credential: json!({ "apiKey": "sk-submitted" }),
        }
    }
}

async fn canned(
    State(canned): State<CannedResponse>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !holds_control_token(&headers) {
        return rejection("Control token is invalid");
    }
    answer(&canned, &method, uri.path(), &body)
}

/// Answers one control route, refusing a body the client should not have sent.
fn answer(canned: &CannedResponse, method: &Method, path: &str, body: &Bytes) -> Response {
    match (method.as_str(), path) {
        ("GET", CONTROL_STATE_PATH) => Json(json!({
            "connection": { "status": "connected" },
            "catalog": house_catalog()
        }))
        .into_response(),
        ("GET", CONTROL_CREDENTIAL_PATH) => {
            Json(json!({ "apiKey": SAVED_API_KEY })).into_response()
        }
        ("PUT", CONTROL_CREDENTIAL_PATH) => match &canned.rejection {
            Some(message) => rejection(message),
            None => match body_matches(body, &canned.expected_credential) {
                true => Json(house_catalog()).into_response(),
                false => rejection("unexpected credential payload"),
            },
        },
        ("DELETE", CONTROL_CREDENTIAL_PATH) => StatusCode::NO_CONTENT.into_response(),
        ("POST", CONTROL_CATALOG_REFRESH_PATH) => Json(house_catalog()).into_response(),
        ("PUT", CONTROL_GATEWAY_PATH) => match body_matches(body, &canned.expected_publish) {
            true => StatusCode::NO_CONTENT.into_response(),
            false => rejection("unexpected published models"),
        },
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

fn holds_control_token(headers: &HeaderMap) -> bool {
    let expected = format!("{BEARER_PREFIX}{CONTROL_TOKEN}");
    headers
        .get(AUTHORIZATION_HEADER)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

fn body_matches(body: &Bytes, expected: &Value) -> bool {
    serde_json::from_slice::<Value>(body).is_ok_and(|received| received == *expected)
}

fn rejection(message: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(json!({ "error": message }))).into_response()
}

fn house_catalog() -> Value {
    json!([{ "id": "qwen3.8-max", "displayName": "Qwen 3.8 Max", "isCustom": false }])
}

fn house_models() -> Vec<ModelCatalogEntry> {
    vec![ModelCatalogEntry {
        id: "qwen3.8-max".to_owned(),
        display_name: "Qwen 3.8 Max".to_owned(),
        is_custom: false,
    }]
}

/// Serves canned control answers on a loopback port for one test.
struct ControlHarness {
    base_url: String,
    server: JoinHandle<()>,
}

impl ControlHarness {
    async fn start(answers: CannedResponse) -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("loopback port should bind");
        let address = listener
            .local_addr()
            .expect("a bound listener should report its address");
        let server = tokio::spawn(async move {
            axum::serve(listener, Router::new().fallback(canned).with_state(answers))
                .await
                .expect("the canned server should serve until the test ends");
        });

        Self {
            base_url: format!("http://{address}"),
            server,
        }
    }

    fn client(&self, token: &str) -> ControlClient {
        ControlClient::new(self.base_url.clone(), SecretString::from(token.to_owned()))
            .expect("the client should build")
    }
}

impl Drop for ControlHarness {
    fn drop(&mut self) {
        self.server.abort();
    }
}
