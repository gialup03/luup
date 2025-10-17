use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::pin::Pin;
use tokio_stream::Stream;
use futures::stream::StreamExt;

/// Ollama client for communicating with the local LLM
#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
    model: String,
    http_client: reqwest::Client,
}

/// Tool definition matching Ollama's native format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: Vec<String>,
    pub properties: serde_json::Map<String, Value>,
}

/// Chat message for conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request to Ollama chat endpoint
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    tools: Option<Vec<Tool>>,
}

/// Streamed response chunk from Ollama
#[derive(Debug, Deserialize)]
pub struct OllamaStreamChunk {
    pub model: String,
    pub created_at: String,
    pub message: Option<ChatMessage>,
    pub done: bool,
    #[serde(default)]
    pub done_reason: Option<String>,
}

/// Types of chunks we can receive from the stream
#[derive(Debug, Clone)]
pub enum StreamChunk {
    TextChunk(String),
    ReasoningChunk(String),
    ToolCall { name: String, arguments: Value },
    Done,
}

impl OllamaClient {
    /// Create a new Ollama client with hardcoded endpoint
    pub fn new() -> Self {
        Self {
            base_url: "http://192.168.0.100:11434".to_string(),
            model: "qwen3:8b".to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new Ollama client with custom base URL
    pub fn with_url(base_url: String) -> Self {
        Self {
            base_url: format!("http://{}", base_url),
            model: "qwen3:8b".to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Send a chat request with tools and return a stream of chunks
    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<Tool>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, Box<dyn Error + Send + Sync>>> + Send>>, Box<dyn Error + Send + Sync>> {
        let request = OllamaRequest {
            model: self.model.clone(),
            messages,
            stream: true,
            tools: if tools.is_empty() { None } else { Some(tools) },
        };

        let url = format!("{}/api/chat", self.base_url);
        println!("🌐 Sending request to Ollama at: {}", url);
        println!("📦 Model: {}", self.model);
        println!("💬 Message count: {}", request.messages.len());
        
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Ollama request failed: {}", response.status()).into());
        }

        let stream = response.bytes_stream();
        Ok(Box::pin(Self::parse_stream(stream)))
    }

    /// Parse NDJSON stream into typed chunks
    fn parse_stream<S, E>(stream: S) -> impl Stream<Item = Result<StreamChunk, Box<dyn Error + Send + Sync>>>
    where
        S: Stream<Item = Result<bytes::Bytes, E>> + Unpin,
        E: Error + Send + Sync + 'static,
    {
        futures::stream::unfold(
            (stream, Vec::new()),
            |(mut stream, mut buffer)| async move {
                loop {
                    match stream.next().await {
                        Some(Ok(ref bytes)) => {
                            buffer.extend_from_slice(bytes);

                            // Try to find a complete line (NDJSON)
                            if let Some(newline_pos) = buffer.iter().position(|&b| b == b'\n') {
                                let line_bytes = buffer.drain(..=newline_pos).collect::<Vec<_>>();
                                let line = String::from_utf8_lossy(&line_bytes);

                                // Parse the JSON line
                                match serde_json::from_str::<OllamaStreamChunk>(&line) {
                                    Ok(chunk) => {
                                        println!("🔍 Raw Ollama chunk: done={}, message={:?}", chunk.done, chunk.message);
                                        if chunk.done {
                                            println!("✅ Ollama stream marked as done");
                                            return Some((Ok(StreamChunk::Done), (stream, buffer)));
                                        }

                                        if let Some(message) = chunk.message {
                                            // Check if this is a tool call
                                            if let Ok(content_json) = serde_json::from_str::<Value>(&message.content) {
                                                if let Some(tool_calls) = content_json.get("tool_calls") {
                                                    if let Some(tool_call_array) = tool_calls.as_array() {
                                                        if let Some(first_call) = tool_call_array.first() {
                                                            if let (Some(name), Some(args)) = (
                                                                first_call.get("function").and_then(|f| f.get("name")).and_then(|n| n.as_str()),
                                                                first_call.get("function").and_then(|f| f.get("arguments"))
                                                            ) {
                                                                return Some((
                                                                    Ok(StreamChunk::ToolCall {
                                                                        name: name.to_string(),
                                                                        arguments: args.clone(),
                                                                    }),
                                                                    (stream, buffer),
                                                                ));
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // Regular text content
                                            if !message.content.is_empty() {
                                                // Check if it looks like reasoning (starts with "thinking:" or similar)
                                                if message.content.starts_with("<think>") || message.content.contains("reasoning:") {
                                                    return Some((
                                                        Ok(StreamChunk::ReasoningChunk(message.content)),
                                                        (stream, buffer),
                                                    ));
                                                } else {
                                                    return Some((
                                                        Ok(StreamChunk::TextChunk(message.content)),
                                                        (stream, buffer),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return Some((
                                            Err(format!("Failed to parse JSON: {}", e).into()),
                                            (stream, buffer),
                                        ));
                                    }
                                }
                            } else {
                                // No complete line yet, continue accumulating
                                continue;
                            }
                        }
                        Some(Err(ref e)) => {
                            return Some((
                                Err(format!("Stream error: {}", e).into()),
                                (stream, buffer),
                            ));
                        }
                        None => {
                            return None; // Stream ended
                        }
                    }
                }
            },
        )
    }
}

/// Create the standard tool set for the game
pub fn create_game_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "set_time".to_string(),
                description: "Update the time of day in the game world".to_string(),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    required: vec!["time".to_string()],
                    properties: {
                        let mut props = serde_json::Map::new();
                        props.insert(
                            "time".to_string(),
                            serde_json::json!({
                                "type": "string",
                                "description": "The time of day",
                                "enum": ["Morning", "Afternoon", "Evening", "Night"]
                            }),
                        );
                        props
                    },
                },
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "set_location".to_string(),
                description: "Change the player's current location".to_string(),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    required: vec!["location".to_string()],
                    properties: {
                        let mut props = serde_json::Map::new();
                        props.insert(
                            "location".to_string(),
                            serde_json::json!({
                                "type": "string",
                                "description": "The name of the new location"
                            }),
                        );
                        props
                    },
                },
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "set_outfit".to_string(),
                description: "Change the player's outfit or equipment".to_string(),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    required: vec!["outfit".to_string()],
                    properties: {
                        let mut props = serde_json::Map::new();
                        props.insert(
                            "outfit".to_string(),
                            serde_json::json!({
                                "type": "string",
                                "description": "Description of the outfit or equipment"
                            }),
                        );
                        props
                    },
                },
            },
        },
    ]
}

