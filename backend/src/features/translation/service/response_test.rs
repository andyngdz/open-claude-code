use bytes::Bytes;
use futures_util::{stream, TryStreamExt};
use indoc::indoc;

use super::{translate_response_stream, UpstreamProtocol};

#[tokio::test]
async fn translates_chat_completion_text_stream_to_anthropic_events() {
    let upstream = stream::iter([Ok(Bytes::from(indoc! {r#"
        data: {"choices":[{"delta":{"content":"Hi"}}]}

        data: {"choices":[{"delta":{},"finish_reason":"stop"}]}

        data: [DONE]

        "#}))]);
    let bytes = translate_response_stream(
        UpstreamProtocol::ChatCompletions,
        "kimi-k3".to_owned(),
        upstream,
    )
    .try_collect::<Vec<_>>()
    .await
    .expect("stream should translate");
    let output = String::from_utf8(bytes.concat().to_vec()).expect("events should be UTF-8");
    assert!(output.contains("event: message_start"));
    assert!(output.contains("text_delta"));
    assert!(output.contains("event: message_stop"));
}

#[tokio::test]
async fn translates_responses_text_stream_split_across_chunks() {
    let upstream = stream::iter([
        Ok(Bytes::from(indoc! {r#"
            event: response.output_text.delta
            data: {"delta":"Hel"#})),
        Ok(Bytes::from(indoc! {r#"
            lo"}

            event: response.completed
            data: {}

            "#})),
    ]);
    let bytes = translate_response_stream(UpstreamProtocol::Responses, "grok".to_owned(), upstream)
        .try_collect::<Vec<_>>()
        .await
        .expect("stream should translate");
    let output = String::from_utf8(bytes.concat().to_vec()).expect("events should be UTF-8");
    assert!(output.contains("Hello"));
    assert!(output.contains("event: message_stop"));
}

#[tokio::test]
async fn preserves_utf8_text_split_across_stream_chunks() {
    let upstream = stream::iter([
        Ok(Bytes::from(
            "data: {\"choices\":[{\"delta\":{\"content\":\"",
        )),
        Ok(Bytes::from(vec![0xE2, 0x82])),
        Ok(Bytes::from(vec![0xAC])),
        Ok(Bytes::from("\"}}]}\n\ndata: [DONE]\n\n")),
    ]);
    let bytes = translate_response_stream(
        UpstreamProtocol::ChatCompletions,
        "kimi-k3".to_owned(),
        upstream,
    )
    .try_collect::<Vec<_>>()
    .await
    .expect("stream should translate");
    let output = String::from_utf8(bytes.concat().to_vec()).expect("events should be UTF-8");

    assert!(output.contains("€"));
}

#[tokio::test]
async fn translates_chat_completion_tool_call_stream_to_anthropic_tool_use() {
    let upstream = stream::iter([Ok(Bytes::from(indoc! {r#"
        data: {"choices":[{"delta":{"tool_calls":[{"index":0,"id":"call_1","function":{"name":"read_file","arguments":"{\"path\":\"README.md\"}"}}]}}]}

        data: {"choices":[{"delta":{},"finish_reason":"tool_calls"}]}

        "#}))]);
    let bytes = translate_response_stream(
        UpstreamProtocol::ChatCompletions,
        "kimi-k3".to_owned(),
        upstream,
    )
    .try_collect::<Vec<_>>()
    .await
    .expect("stream should translate");
    let output = String::from_utf8(bytes.concat().to_vec()).expect("events should be UTF-8");

    assert!(output.contains("tool_use"));
    assert!(output.contains("read_file"));
    assert!(output.contains("input_json_delta"));
}
