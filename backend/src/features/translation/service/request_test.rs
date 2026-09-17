use super::{translate_request, UpstreamProtocol};

#[test]
fn translates_text_request_for_chat_completions() {
    let input = br#"{"model":"kimi-k3","system":"Be brief.","messages":[{"role":"user","content":[{"type":"text","text":"Hello"}]}],"max_tokens":20,"stream":true}"#;
    let output = translate_request(UpstreamProtocol::ChatCompletions, input)
        .expect("fixture should translate");
    let payload: serde_json::Value = serde_json::from_slice(&output).expect("output should parse");
    assert_eq!(payload["messages"][0]["role"], "system");
    assert_eq!(payload["messages"][1]["content"], "Hello");
}

#[test]
fn translates_system_and_text_request_for_responses() {
    let input = br#"{"model":"grok-4.6","system":"Be brief.","messages":[{"role":"user","content":"Hello"}],"max_tokens":20,"stream":true}"#;
    let output =
        translate_request(UpstreamProtocol::Responses, input).expect("fixture should translate");
    let payload: serde_json::Value = serde_json::from_slice(&output).expect("output should parse");
    assert_eq!(payload["instructions"], "Be brief.");
    assert_eq!(payload["input"][0]["content"][0]["text"], "Hello");
    assert_eq!(payload["max_output_tokens"], 20);
}
