use serde_json::Value;

use super::request::RequestField;

const ROLE_USER: &str = "user";
const ROLE_TOOL: &str = "tool";
const TYPE_TOOL_USE: &str = "tool_use";
const TYPE_TOOL_RESULT: &str = "tool_result";
const TYPE_FUNCTION_CALL: &str = "function_call";
const TYPE_FUNCTION_OUTPUT: &str = "function_call_output";
const FIELD_TOOL_CALLS: &str = "tool_calls";
const FIELD_TOOL_CALL_ID: &str = "tool_call_id";
const FIELD_TOOL_USE_ID: &str = "tool_use_id";
const FIELD_ARGUMENTS: &str = "arguments";
const FIELD_CALL_ID: &str = "call_id";
const FIELD_OUTPUT: &str = "output";

/// Converts Anthropic message history into OpenAI Chat Completion message items.
pub(super) fn chat_history(messages: &[Value]) -> Vec<Value> {
    messages.iter().flat_map(chat_message).collect()
}

/// Converts Anthropic message history into OpenAI Responses input items.
pub(super) fn responses_history(messages: &[Value]) -> Vec<Value> {
    messages.iter().flat_map(response_message).collect()
}

fn chat_message(message: &Value) -> Vec<Value> {
    let role = string_field(message, RequestField::Role).unwrap_or(ROLE_USER);
    let parts = content_parts(message);
    let results: Vec<_> = parts
        .iter()
        .filter(|part| kind(part) == TYPE_TOOL_RESULT)
        .collect();
    if !results.is_empty() {
        return results.into_iter().map(chat_tool_result).collect();
    }
    let mut output = message_value(role, message_text(message, &parts));
    let calls: Vec<_> = parts
        .iter()
        .filter(|part| kind(part) == TYPE_TOOL_USE)
        .map(chat_tool_call)
        .collect();
    if !calls.is_empty() {
        insert_field(&mut output, FIELD_TOOL_CALLS, Value::Array(calls));
    }
    vec![output]
}

fn response_message(message: &Value) -> Vec<Value> {
    let role = string_field(message, RequestField::Role).unwrap_or(ROLE_USER);
    let parts = content_parts(message);
    let results: Vec<_> = parts
        .iter()
        .filter(|part| kind(part) == TYPE_TOOL_RESULT)
        .map(response_tool_result)
        .collect();
    if !results.is_empty() {
        return results;
    }
    let calls: Vec<_> = parts
        .iter()
        .filter(|part| kind(part) == TYPE_TOOL_USE)
        .map(response_tool_call)
        .collect();
    if !calls.is_empty() {
        return calls;
    }
    vec![response_text(role, message_text(message, &parts))]
}

fn chat_tool_result(part: &Value) -> Value {
    let mut output = message_value(ROLE_TOOL, text_field(part, RequestField::Content));
    let tool_use_id = part
        .get(FIELD_TOOL_USE_ID)
        .and_then(Value::as_str)
        .unwrap_or_default();
    insert_field(
        &mut output,
        FIELD_TOOL_CALL_ID,
        Value::String(tool_use_id.to_owned()),
    );
    output
}

fn chat_tool_call(part: &Value) -> Value {
    serde_json::json!({"id": part.get("id").and_then(Value::as_str).unwrap_or_default(), "type": TYPE_FUNCTION_CALL, "function": {"name": string_field(part, RequestField::Name).unwrap_or_default(), FIELD_ARGUMENTS: part.get("input").unwrap_or(&Value::Null).to_string()}})
}

fn response_tool_result(part: &Value) -> Value {
    serde_json::json!({"type": TYPE_FUNCTION_OUTPUT, FIELD_CALL_ID: part.get(FIELD_TOOL_USE_ID).and_then(Value::as_str).unwrap_or_default(), FIELD_OUTPUT: text_field(part, RequestField::Content)})
}

fn response_tool_call(part: &Value) -> Value {
    serde_json::json!({"type": TYPE_FUNCTION_CALL, FIELD_CALL_ID: part.get("id").and_then(Value::as_str).unwrap_or_default(), "name": string_field(part, RequestField::Name).unwrap_or_default(), FIELD_ARGUMENTS: part.get("input").unwrap_or(&Value::Null).to_string()})
}

fn response_text(role: &str, text: String) -> Value {
    serde_json::json!({"role": role, "content": [{"type": "input_text", "text": text}]})
}

fn message_value(role: &str, content: String) -> Value {
    serde_json::json!({"role": role, "content": content})
}

fn insert_field(value: &mut Value, name: &str, field: Value) {
    if let Some(object) = value.as_object_mut() {
        object.insert(name.to_owned(), field);
    }
}
fn content_parts(message: &Value) -> Vec<Value> {
    message
        .get("content")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}
fn kind(part: &Value) -> &str {
    part.get("type").and_then(Value::as_str).unwrap_or_default()
}
fn string_field(value: &Value, field: RequestField) -> Option<&str> {
    value.get(field.as_str()).and_then(Value::as_str)
}
fn text_field(value: &Value, field: RequestField) -> String {
    value
        .get(field.as_str())
        .map(text_value)
        .unwrap_or_default()
}
fn text_parts(parts: &[Value]) -> String {
    parts
        .iter()
        .filter_map(|part| part.get("text").and_then(Value::as_str))
        .collect()
}

fn message_text(message: &Value, parts: &[Value]) -> String {
    message
        .get("content")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| text_parts(parts))
}
fn text_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Array(parts) => text_parts(parts),
        _ => String::new(),
    }
}

#[cfg(test)]
#[path = "history_test.rs"]
mod history_test;
