use crate::features::translation::UpstreamProtocol;

const MESSAGES_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/messages";
const CHAT_COMPLETIONS_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/chat/completions";
const RESPONSES_ENDPOINT: &str = "https://opencode.ai/zen/go/v1/responses";

/// Returns the upstream endpoint for a supported protocol.
pub(super) fn endpoint_for_protocol(protocol: UpstreamProtocol) -> &'static str {
    match protocol {
        UpstreamProtocol::Messages => MESSAGES_ENDPOINT,
        UpstreamProtocol::ChatCompletions => CHAT_COMPLETIONS_ENDPOINT,
        UpstreamProtocol::Responses => RESPONSES_ENDPOINT,
    }
}

/// Returns the protocol order used to negotiate a model without model-specific mappings.
pub(super) fn protocol_candidates() -> [UpstreamProtocol; 3] {
    [
        UpstreamProtocol::Messages,
        UpstreamProtocol::ChatCompletions,
        UpstreamProtocol::Responses,
    ]
}

#[cfg(test)]
#[path = "protocol_test.rs"]
mod protocol_test;
