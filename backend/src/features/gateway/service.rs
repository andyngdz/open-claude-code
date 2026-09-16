use super::{
    GatewayConfiguration, GatewayDiscoveryModel, GatewayRequestError, ResolvedProviderRequest,
};
pub(super) use crate::constants::MODEL_FIELD;
use crate::features::providers::ProviderId;

const PUBLIC_MODEL_PREFIX: &str = "open-claude-code/claude/";
const CUSTOM_MODEL_DESCRIPTION: &str =
    "Custom provider model, attempted through Anthropic Messages";
const VERIFIED_MODEL_DESCRIPTION: &str = "Provider model compatible with Anthropic Messages";

/// Builds a discovery-safe model ID that Claude Code accepts in `/model`.
pub fn public_model_id(provider_id: &ProviderId, model_id: &str) -> String {
    format!("{PUBLIC_MODEL_PREFIX}{provider_id}/{model_id}")
}

/// Builds the models published by Claude Code gateway discovery.
pub(crate) fn discovery_models(configuration: &GatewayConfiguration) -> Vec<GatewayDiscoveryModel> {
    let mut models = configuration
        .catalog
        .iter()
        .map(|model| GatewayDiscoveryModel {
            id: public_model_id(&configuration.provider_id, &model.id),
            display_name: model.display_name.clone(),
            description: VERIFIED_MODEL_DESCRIPTION,
        })
        .collect::<Vec<_>>();
    models.extend(
        configuration
            .custom_models
            .iter()
            .map(|model_id| GatewayDiscoveryModel {
                id: public_model_id(&configuration.provider_id, model_id),
                display_name: model_id.clone(),
                description: CUSTOM_MODEL_DESCRIPTION,
            }),
    );
    models
}

/// Validates the public model target and rewrites it to the upstream model ID.
pub(crate) fn resolve_provider_request(
    configuration: &GatewayConfiguration,
    request_body: &[u8],
) -> Result<ResolvedProviderRequest, GatewayRequestError> {
    let mut payload: serde_json::Value =
        serde_json::from_slice(request_body).map_err(|_source| GatewayRequestError::InvalidJson)?;
    let public_id = payload
        .get(MODEL_FIELD)
        .and_then(serde_json::Value::as_str)
        .ok_or(GatewayRequestError::MissingModel)?;
    let (provider_id, model_id) =
        parse_public_model_id(public_id).ok_or(GatewayRequestError::InvalidPublicModel)?;
    if !is_configured_model(configuration, &provider_id, &model_id) {
        return Err(GatewayRequestError::ModelNotConfigured);
    }
    payload[MODEL_FIELD] = serde_json::Value::String(model_id);
    let body =
        serde_json::to_vec(&payload).map_err(|_source| GatewayRequestError::Serialization)?;
    Ok(ResolvedProviderRequest { provider_id, body })
}

fn parse_public_model_id(value: &str) -> Option<(ProviderId, String)> {
    let remainder = value.strip_prefix(PUBLIC_MODEL_PREFIX)?;
    let (provider_id, model_id) = remainder.split_once('/')?;
    if provider_id.is_empty() || model_id.is_empty() {
        return None;
    }
    Some((ProviderId::new(provider_id), model_id.to_owned()))
}

fn is_configured_model(
    configuration: &GatewayConfiguration,
    provider_id: &ProviderId,
    model_id: &str,
) -> bool {
    configuration.provider_id == *provider_id
        && (configuration
            .catalog
            .iter()
            .any(|model| model.id == model_id)
            || configuration
                .custom_models
                .iter()
                .any(|custom| custom == model_id))
}

#[cfg(test)]
#[path = "service_test.rs"]
mod service_test;
