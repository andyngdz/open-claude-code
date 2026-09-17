use serde_json::{Map, Value};

use super::request::{RequestField, UpstreamProtocol};

/// Converts Anthropic tool declarations to the selected upstream schema.
pub(super) fn copy_tools(
    request: &Value,
    payload: &mut Map<String, Value>,
    protocol: UpstreamProtocol,
) {
    let Some(tools) = request
        .get(RequestField::Tools.as_str())
        .and_then(Value::as_array)
    else {
        return;
    };
    let translated = tools
        .iter()
        .map(|tool| translate_tool(tool, protocol))
        .collect();
    payload.insert(
        RequestField::Tools.as_str().to_owned(),
        Value::Array(translated),
    );
}

fn translate_tool(tool: &Value, protocol: UpstreamProtocol) -> Value {
    let name = tool
        .get(RequestField::Name.as_str())
        .cloned()
        .unwrap_or(Value::Null);
    let description = tool
        .get(RequestField::Description.as_str())
        .cloned()
        .unwrap_or(Value::Null);
    let parameters = tool
        .get(RequestField::InputSchema.as_str())
        .cloned()
        .unwrap_or(Value::Null);
    match protocol {
        UpstreamProtocol::ChatCompletions => chat_tool(name, description, parameters),
        UpstreamProtocol::Responses => responses_tool(name, description, parameters),
        UpstreamProtocol::Messages => tool.clone(),
    }
}

fn chat_tool(name: Value, description: Value, parameters: Value) -> Value {
    let mut function = Map::new();
    function.insert(RequestField::Name.as_str().to_owned(), name);
    function.insert(RequestField::Description.as_str().to_owned(), description);
    function.insert(RequestField::Parameters.as_str().to_owned(), parameters);
    let mut tool = Map::new();
    tool.insert(
        RequestField::Type.as_str().to_owned(),
        Value::String(RequestField::Function.as_str().to_owned()),
    );
    tool.insert(
        RequestField::Function.as_str().to_owned(),
        Value::Object(function),
    );
    Value::Object(tool)
}

fn responses_tool(name: Value, description: Value, parameters: Value) -> Value {
    let mut tool = Map::new();
    tool.insert(
        RequestField::Type.as_str().to_owned(),
        Value::String(RequestField::Function.as_str().to_owned()),
    );
    tool.insert(RequestField::Name.as_str().to_owned(), name);
    tool.insert(RequestField::Description.as_str().to_owned(), description);
    tool.insert(RequestField::Parameters.as_str().to_owned(), parameters);
    Value::Object(tool)
}

#[cfg(test)]
#[path = "tools_test.rs"]
mod tools_test;
