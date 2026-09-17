use serde_json::{json, Map, Value};

use super::{copy_tools, UpstreamProtocol};

#[test]
fn maps_anthropic_tool_schema_to_chat_completions() {
    let request = json!({"tools":[{"name":"read_file","description":"Reads a file","input_schema":{"type":"object"}}]});
    let mut output = Map::<String, Value>::new();

    copy_tools(&request, &mut output, UpstreamProtocol::ChatCompletions);

    assert_eq!(output["tools"][0]["type"], "function");
    assert_eq!(
        output["tools"][0]["function"]["parameters"]["type"],
        "object"
    );
}
