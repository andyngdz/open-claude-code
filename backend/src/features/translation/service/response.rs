use std::{
    collections::{HashMap, HashSet},
    io,
};

use bytes::Bytes;
use futures_util::{future, Stream, StreamExt};
use serde_json::{json, Value};

use super::framing::sse_frame_end;
use super::UpstreamProtocol;
use crate::features::providers::ProviderBodyStream;

const SSE_FRAME_SEPARATOR: &str = "\x0A\x0A";
const SSE_DATA_PREFIX: &str = "data: ";
const SSE_EVENT_PREFIX: &str = "event: ";

/// Converts OpenAI-compatible server-sent events to Anthropic Messages events.
pub(crate) fn translate_response_stream<S>(
    protocol: UpstreamProtocol,
    model_id: String,
    upstream: S,
) -> ProviderBodyStream
where
    S: Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
{
    let state = ResponseTranslator::new(protocol, model_id);
    Box::pin(upstream.scan(state, |state, chunk| {
        let output = chunk.map(|chunk| state.translate_chunk(&chunk));
        future::ready(Some(output))
    }))
}

struct ResponseTranslator {
    protocol: UpstreamProtocol,
    model_id: String,
    buffer: Vec<u8>,
    message_started: bool,
    text_started: bool,
    active_tools: HashSet<usize>,
    tool_indices: HashMap<String, usize>,
    completed: bool,
}

impl ResponseTranslator {
    fn new(protocol: UpstreamProtocol, model_id: String) -> Self {
        Self {
            protocol,
            model_id,
            buffer: Vec::new(),
            message_started: false,
            text_started: false,
            active_tools: HashSet::new(),
            tool_indices: HashMap::new(),
            completed: false,
        }
    }

    fn translate_chunk(&mut self, chunk: &[u8]) -> Bytes {
        self.buffer.extend_from_slice(chunk);
        let mut output = String::new();
        while let Some((frame_end, separator_len)) = sse_frame_end(&self.buffer) {
            let frame = self.buffer[..frame_end].to_vec();
            self.buffer.drain(..frame_end + separator_len);
            if let Ok(frame) = std::str::from_utf8(&frame) {
                self.translate_frame(frame, &mut output);
            }
        }
        Bytes::from(output)
    }

    fn translate_frame(&mut self, frame: &str, output: &mut String) {
        let event = frame
            .lines()
            .find_map(|line| line.strip_prefix(SSE_EVENT_PREFIX));
        let data = frame
            .lines()
            .find_map(|line| line.strip_prefix(SSE_DATA_PREFIX));
        let Some(data) = data else {
            return;
        };
        if data == "[DONE]" {
            self.finish(output, "end_turn");
            return;
        }
        let Ok(payload) = serde_json::from_str::<Value>(data) else {
            return;
        };
        match self.protocol {
            UpstreamProtocol::ChatCompletions => self.translate_chat(payload, output),
            UpstreamProtocol::Responses => self.translate_responses(event, payload, output),
            UpstreamProtocol::Messages => {}
        }
    }

    fn translate_chat(&mut self, payload: Value, output: &mut String) {
        let Some(choice) = payload
            .get("choices")
            .and_then(Value::as_array)
            .and_then(|items| items.first())
        else {
            return;
        };
        let delta = choice.get("delta").unwrap_or(&Value::Null);
        if let Some(text) = delta.get("content").and_then(Value::as_str) {
            self.text_delta(output, text);
        }
        if let Some(tool_calls) = delta.get("tool_calls").and_then(Value::as_array) {
            for call in tool_calls {
                let index = call.get("index").and_then(Value::as_u64).unwrap_or(0) as usize
                    + usize::from(self.text_started);
                let name = call.pointer("/function/name").and_then(Value::as_str);
                let id = call.get("id").and_then(Value::as_str);
                self.tool_start(output, index, id, name);
                if let Some(arguments) = call.pointer("/function/arguments").and_then(Value::as_str)
                {
                    self.event(output, "content_block_delta", json!({"type":"content_block_delta","index":index,"delta":{"type":"input_json_delta","partial_json":arguments}}));
                }
            }
        }
        if let Some(reason) = choice.get("finish_reason").and_then(Value::as_str) {
            self.finish(
                output,
                if reason == "tool_calls" {
                    "tool_use"
                } else {
                    "end_turn"
                },
            );
        }
    }

    fn translate_responses(&mut self, event: Option<&str>, payload: Value, output: &mut String) {
        match event.or_else(|| payload.get("type").and_then(Value::as_str)) {
            Some("response.output_text.delta") => {
                if let Some(text) = payload.get("delta").and_then(Value::as_str) {
                    self.text_delta(output, text);
                }
            }
            Some("response.output_item.added") => {
                let item = payload.get("item").unwrap_or(&Value::Null);
                if item.get("type").and_then(Value::as_str) == Some("function_call") {
                    let index = self.next_tool_index();
                    self.tool_start(
                        output,
                        index,
                        item.get("call_id").and_then(Value::as_str),
                        item.get("name").and_then(Value::as_str),
                    );
                }
            }
            Some("response.function_call_arguments.delta") => {
                let call_id = payload.get("call_id").and_then(Value::as_str);
                let index = self.tool_index(call_id);
                self.tool_start(output, index, call_id, None);
                if let Some(arguments) = payload.get("delta").and_then(Value::as_str) {
                    self.event(output, "content_block_delta", json!({"type":"content_block_delta","index":index,"delta":{"type":"input_json_delta","partial_json":arguments}}));
                }
            }
            Some("response.completed") => self.finish(output, "end_turn"),
            _ => {}
        }
    }

    fn text_delta(&mut self, output: &mut String, text: &str) {
        self.start_message(output);
        if !self.text_started {
            self.text_started = true;
            self.event(output, "content_block_start", json!({"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}));
        }
        self.event(output, "content_block_delta", json!({"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":text}}));
    }

    fn tool_start(
        &mut self,
        output: &mut String,
        index: usize,
        id: Option<&str>,
        name: Option<&str>,
    ) {
        self.start_message(output);
        if let Some(id) = id {
            self.tool_indices.insert(id.to_owned(), index);
        }
        if self.active_tools.insert(index) {
            self.event(output, "content_block_start", json!({"type":"content_block_start","index":index,"content_block":{"type":"tool_use","id":id.unwrap_or("toolu_open_claude_code"),"name":name.unwrap_or("unknown"),"input":{}}}));
        }
    }

    fn next_tool_index(&self) -> usize {
        self.active_tools.len() + usize::from(self.text_started)
    }

    fn tool_index(&self, id: Option<&str>) -> usize {
        id.and_then(|id| self.tool_indices.get(id).copied())
            .unwrap_or_else(|| self.next_tool_index())
    }

    fn start_message(&mut self, output: &mut String) {
        if !self.message_started {
            self.message_started = true;
            self.event(output, "message_start", json!({"type":"message_start","message":{"id":"msg_open_claude_code","type":"message","role":"assistant","content":[],"model":self.model_id,"stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":0,"output_tokens":0}}}));
        }
    }

    fn finish(&mut self, output: &mut String, reason: &str) {
        if self.completed {
            return;
        }
        self.start_message(output);
        if self.text_started {
            self.event(
                output,
                "content_block_stop",
                json!({"type":"content_block_stop","index":0}),
            );
        }
        for index in self.active_tools.clone() {
            self.event(
                output,
                "content_block_stop",
                json!({"type":"content_block_stop","index":index}),
            );
        }
        self.event(output, "message_delta", json!({"type":"message_delta","delta":{"stop_reason":reason,"stop_sequence":null},"usage":{"output_tokens":0}}));
        self.event(output, "message_stop", json!({"type":"message_stop"}));
        self.completed = true;
    }

    fn event(&self, output: &mut String, name: &str, payload: Value) {
        output.push_str("event: ");
        output.push_str(name);
        output.push('\n');
        output.push_str("data: ");
        output.push_str(&payload.to_string());
        output.push_str(SSE_FRAME_SEPARATOR);
    }
}

#[cfg(test)]
#[path = "response_test.rs"]
mod response_test;
