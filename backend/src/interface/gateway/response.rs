use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::constants::MODEL_FIELD;
use crate::features::{
    gateway::GatewayDiscoveryModel,
    providers::{ProviderError, ProviderHeader, ProviderResponse},
};

const INVALID_REQUEST_ERROR: &str = "invalid_request_error";
const AUTHENTICATION_ERROR: &str = "authentication_error";
const API_ERROR: &str = "api_error";

/// Converts gateway and provider results into Anthropic-compatible HTTP responses.
pub(super) struct ResponsePresenter;

impl ResponsePresenter {
    /// Returns an Anthropic authentication error.
    pub(super) fn authentication_error(message: &str) -> Response {
        Self::error(StatusCode::UNAUTHORIZED, AUTHENTICATION_ERROR, message)
    }

    /// Returns an Anthropic invalid-request error.
    pub(super) fn bad_request(message: &str) -> Response {
        Self::error(StatusCode::BAD_REQUEST, INVALID_REQUEST_ERROR, message)
    }

    /// Serializes models using Claude Code's gateway discovery shape.
    pub(super) fn models(models: Vec<GatewayDiscoveryModel>) -> Response {
        let models = models.into_iter().map(DiscoveryModel::from).collect();
        Json(DiscoveryResponse {
            object: "list",
            data: models,
        })
        .into_response()
    }

    /// Preserves an upstream status, headers, and streaming body.
    pub(super) fn upstream(provider_response: ProviderResponse) -> Response {
        let status =
            StatusCode::from_u16(provider_response.status_code).unwrap_or(StatusCode::BAD_GATEWAY);
        let mut response = Response::new(Body::from_stream(provider_response.body));
        *response.status_mut() = status;
        Self::append_headers(&mut response, provider_response.headers);
        response
    }

    /// Maps provider boundary failures without exposing sensitive details.
    pub(super) fn provider_error(source: ProviderError) -> Response {
        match source {
            ProviderError::CredentialNotFound | ProviderError::InvalidApiKey => {
                Self::authentication_error(&source.to_string())
            }
            ProviderError::NotRegistered(_) | ProviderError::EmptyApiKey => {
                Self::bad_request(&source.to_string())
            }
            ProviderError::KeyringUnavailable(_)
            | ProviderError::Request(_)
            | ProviderError::UnexpectedResponse
            | ProviderError::KeyringTask(_)
            | ProviderError::InvalidHeader => Self::error(
                StatusCode::BAD_GATEWAY,
                API_ERROR,
                "Provider request failed",
            ),
        }
    }

    fn error(status: StatusCode, error_type: &str, message: &str) -> Response {
        (
            status,
            Json(ErrorEnvelope {
                response_type: "error",
                error: ErrorDetail {
                    error_type,
                    message,
                },
            }),
        )
            .into_response()
    }

    fn append_headers(response: &mut Response, headers: Vec<ProviderHeader>) {
        for header in headers {
            let Ok(name) = HeaderName::from_bytes(header.name.as_bytes()) else {
                continue;
            };
            let Ok(value) = HeaderValue::from_bytes(&header.value) else {
                continue;
            };
            response.headers_mut().append(name, value);
        }
    }
}

#[derive(Serialize)]
struct DiscoveryResponse {
    object: &'static str,
    data: Vec<DiscoveryModel>,
}

#[derive(Serialize)]
struct DiscoveryModel {
    id: String,
    object: &'static str,
    created: u8,
    display_name: String,
    description: &'static str,
}

impl From<GatewayDiscoveryModel> for DiscoveryModel {
    fn from(model: GatewayDiscoveryModel) -> Self {
        Self {
            id: model.id,
            object: MODEL_FIELD,
            created: 0,
            display_name: model.display_name,
            description: model.description,
        }
    }
}

#[derive(Serialize)]
struct ErrorEnvelope<'a> {
    #[serde(rename = "type")]
    response_type: &'static str,
    error: ErrorDetail<'a>,
}

#[derive(Serialize)]
struct ErrorDetail<'a> {
    #[serde(rename = "type")]
    error_type: &'a str,
    message: &'a str,
}

#[cfg(test)]
#[path = "response_test.rs"]
mod response_test;
