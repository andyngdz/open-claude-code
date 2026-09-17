use serde_json::json;

use super::{chat_history, responses_history};

#[test]
fn maps_anthropic_tool_use_to_chat_completion_tool_call() {
    let messages = vec![
        json!({"role":"assistant","content":[{"type":"tool_use","id":"toolu_1","name":"read_file","input":{"path":"README.md"}}]}),
    ];

    let output = chat_history(&messages);

    assert_eq!(output[0]["tool_calls"][0]["id"], "toolu_1");
    assert_eq!(output[0]["tool_calls"][0]["function"]["name"], "read_file");
}

#[test]
fn maps_anthropic_tool_result_to_responses_function_output() {
    let messages = vec![
        json!({"role":"user","content":[{"type":"tool_result","tool_use_id":"toolu_1","content":"file contents"}]}),
    ];

    let output = responses_history(&messages);

    assert_eq!(output[0]["type"], "function_call_output");
    assert_eq!(output[0]["call_id"], "toolu_1");
    assert_eq!(output[0]["output"], "file contents");
}
