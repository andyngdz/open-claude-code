use serde_json::{Map, Value};

use super::history::{chat_history, responses_history};
use super::tools::copy_tools;

/// Names fields shared by Anthropic and OpenAI request payloads.
#[derive(Clone, Copy)]
pub(super) enum RequestField {
    Content,
    Input,
    Instructions,
    MaxTokens,
    MaxOutputTokens,
    Messages,
    Model,
    Role,
    Stream,
    System,
    Text,
    Type,
    Tools,
    Name,
    Description,
    Parameters,
    Function,
    InputSchema,
}

impl RequestField {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Content => "content",
            Self::Input => "input",
            Self::Instructions => "instructions",
            Self::MaxTokens => "max_tokens",
            Self::MaxOutputTokens => "max_output_tokens",
            Self::Messages => "messages",
            Self::Model => "model",
            Self::Role => "role",
            Self::Stream => "stream",
            Self::System => "system",
            Self::Text => "text",
            Self::Type => "type",
            Self::Tools => "tools",
            Self::Name => "name",
            Self::Description => "description",
            Self::Parameters => "parameters",
            Self::Function => "function",
            Self::InputSchema => "input_schema",
        }
    }
}

/// Identifies the OpenCode Go endpoint that receives a translated request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UpstreamProtocol {
    /// Anthropic Messages requires no translation.
    Messages,
    /// OpenAI-compatible Chat Completions.
    ChatCompletions,
    /// OpenAI Responses.
    Responses,
}

/// Translates an Anthropic Messages payload into the selected upstream protocol.
pub(crate) fn translate_request(
    protocol: UpstreamProtocol,
    body: &[u8],
) -> Result<Vec<u8>, serde_json::Error> {
    let request: Value = serde_json::from_slice(body)?;
    let translated = match protocol {
        UpstreamProtocol::Messages => request,
        UpstreamProtocol::ChatCompletions => chat_completions_request(&request),
        UpstreamProtocol::Responses => responses_request(&request),
    };
    serde_json::to_vec(&translated)
}

fn chat_completions_request(request: &Value) -> Value {
    let mut payload = Map::new();
    copy_field(request, &mut payload, RequestField::Model);
    copy_field(request, &mut payload, RequestField::Stream);
    copy_field(request, &mut payload, RequestField::MaxTokens);
    payload.insert(
        RequestField::Messages.as_str().to_owned(),
        chat_messages(request),
    );
    copy_tools(request, &mut payload, UpstreamProtocol::ChatCompletions);
    Value::Object(payload)
}

fn responses_request(request: &Value) -> Value {
    let mut payload = Map::new();
    copy_field(request, &mut payload, RequestField::Model);
    copy_field(request, &mut payload, RequestField::Stream);
    copy_renamed_field(
        request,
        &mut payload,
        RequestField::MaxTokens,
        RequestField::MaxOutputTokens,
    );
    if let Some(system) = request.get(RequestField::System.as_str()) {
        payload.insert(
            RequestField::Instructions.as_str().to_owned(),
            Value::String(text_content(system)),
        );
    }
    payload.insert(
        RequestField::Input.as_str().to_owned(),
        responses_input(request),
    );
    copy_tools(request, &mut payload, UpstreamProtocol::Responses);
    Value::Object(payload)
}

fn chat_messages(request: &Value) -> Value {
    let mut messages = Vec::new();
    if let Some(system) = request.get(RequestField::System.as_str()) {
        messages.push(message_value(
            RequestField::System.as_str(),
            text_content(system),
        ));
    }
    if let Some(request_messages) = request
        .get(RequestField::Messages.as_str())
        .and_then(Value::as_array)
    {
        messages.extend(chat_history(request_messages));
    }
    Value::Array(messages)
}

fn message_value(role: &str, content: String) -> Value {
    let mut message = Map::new();
    message.insert(
        RequestField::Role.as_str().to_owned(),
        Value::String(role.to_owned()),
    );
    message.insert(
        RequestField::Content.as_str().to_owned(),
        Value::String(content),
    );
    Value::Object(message)
}

fn responses_input(request: &Value) -> Value {
    let messages = request
        .get(RequestField::Messages.as_str())
        .and_then(Value::as_array)
        .map_or(&[] as &[Value], Vec::as_slice);
    Value::Array(responses_history(messages))
}

fn text_content(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|part| {
                part.get(RequestField::Text.as_str())
                    .and_then(Value::as_str)
            })
            .collect(),
        _ => String::new(),
    }
}

fn copy_field(request: &Value, payload: &mut Map<String, Value>, field: RequestField) {
    if let Some(value) = request.get(field.as_str()) {
        payload.insert(field.as_str().to_owned(), value.clone());
    }
}

fn copy_renamed_field(
    request: &Value,
    payload: &mut Map<String, Value>,
    source: RequestField,
    target: RequestField,
) {
    if let Some(value) = request.get(source.as_str()) {
        payload.insert(target.as_str().to_owned(), value.clone());
    }
}

#[cfg(test)]
#[path = "request_test.rs"]
mod request_test;
