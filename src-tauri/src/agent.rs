use crate::ollama::{ChatMessage, OllamaClient, StreamChunk, create_game_tools};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use tokio_stream::StreamExt;

/// Game state that can be modified by tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub time: String,
    pub location: String,
    pub outfit: String,
}

/// Messages that can be streamed to the frontend
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentMessage {
    TextChunk { content: String },
    ReasoningChunk { content: String },
    ToolCall { name: String, args: Value },
    ToolResult { name: String, result: GameState },
    Choices { choices: Vec<String> },
    TurnComplete { 
        turn_number: u32,
        story_text: String,
        choices: Vec<String>,
        game_state: GameState,
    },
    Error { message: String },
}

/// The agentic system that manages the game loop
#[derive(Clone)]
pub struct Agent {
    client: OllamaClient,
    conversation_history: Vec<ChatMessage>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            client: OllamaClient::new(),
            conversation_history: Vec::new(),
        }
    }

    pub fn with_ollama_url(base_url: String) -> Self {
        Self {
            client: OllamaClient::with_url(base_url),
            conversation_history: Vec::new(),
        }
    }

    /// Initialize a new game session
    pub fn start_new_game(&mut self) -> GameState {
        self.conversation_history.clear();
        
        // Add system prompt
        self.conversation_history.push(ChatMessage {
            role: "system".to_string(),
            content: Self::create_system_prompt(),
        });

        GameState {
            time: "Morning".to_string(),
            location: "Mysterious Room".to_string(),
            outfit: "Traveler's Cloak".to_string(),
        }
    }

    /// Main agentic loop - processes an action and streams responses
    pub async fn process_action<F>(
        &mut self,
        action: String,
        current_state: &mut GameState,
        turn_number: u32,
        mut emit: F,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        F: FnMut(AgentMessage) + Send,
    {
        // Add user action to conversation
        let user_message = self.format_user_message(&action, current_state);
        self.conversation_history.push(ChatMessage {
            role: "user".to_string(),
            content: user_message,
        });

        // Get tools
        let tools = create_game_tools();

        // Call Ollama with streaming
        let mut stream = self
            .client
            .chat_stream(self.conversation_history.clone(), tools)
            .await?;

        let mut accumulated_text = String::new();
        let mut accumulated_reasoning = String::new();
        
        // Process stream
        println!("📡 Starting to process Ollama stream...");
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => match chunk {
                    StreamChunk::TextChunk(content) => {
                        println!("💬 Text chunk received: {}", content);
                        accumulated_text.push_str(&content);
                        emit(AgentMessage::TextChunk { 
                            content: content.clone() 
                        });
                    }
                    StreamChunk::ReasoningChunk(content) => {
                        println!("🤔 Reasoning chunk received: {}", content);
                        accumulated_reasoning.push_str(&content);
                        emit(AgentMessage::ReasoningChunk { 
                            content: content.clone() 
                        });
                    }
                    StreamChunk::ToolCall { name, arguments } => {
                        println!("🔧 Tool call received: {} with args: {:?}", name, arguments);
                        // Emit tool call notification
                        emit(AgentMessage::ToolCall {
                            name: name.clone(),
                            args: arguments.clone(),
                        });

                        // Execute tool and update state
                        if let Err(e) = self.execute_tool(&name, &arguments, current_state) {
                            println!("❌ Tool execution failed: {}", e);
                            emit(AgentMessage::Error {
                                message: format!("Tool execution failed: {}", e),
                            });
                        } else {
                            println!("✅ Tool executed successfully, new state: {:?}", current_state);
                            // Emit updated state
                            emit(AgentMessage::ToolResult {
                                name: name.clone(),
                                result: current_state.clone(),
                            });
                        }
                    }
                    StreamChunk::Done => {
                        println!("🏁 Stream done signal received");
                        break;
                    }
                },
                Err(e) => {
                    println!("❌ Stream error: {}", e);
                    emit(AgentMessage::Error {
                        message: format!("Stream error: {}", e),
                    });
                    break;
                }
            }
        }
        println!("📝 Accumulated text length: {} chars", accumulated_text.len());

        // Add assistant response to history
        if !accumulated_text.is_empty() {
            self.conversation_history.push(ChatMessage {
                role: "assistant".to_string(),
                content: accumulated_text.clone(),
            });
        }

        // Generate choices (for now, use defaults - could be extracted from model response)
        let choices = self.extract_choices(&accumulated_text);
        println!("🎲 Extracted {} choices from text", choices.len());

        // Emit turn complete
        println!("🎯 Emitting TurnComplete with {} chars of story text", accumulated_text.len());
        emit(AgentMessage::TurnComplete {
            turn_number,
            story_text: accumulated_text.clone(),
            choices: choices.clone(),
            game_state: current_state.clone(),
        });

        Ok(())
    }

    /// Execute a tool call and modify game state
    fn execute_tool(
        &self,
        tool_name: &str,
        arguments: &Value,
        state: &mut GameState,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match tool_name {
            "set_time" => {
                if let Some(time) = arguments.get("time").and_then(|v| v.as_str()) {
                    state.time = time.to_string();
                    Ok(())
                } else {
                    Err("Missing 'time' argument".into())
                }
            }
            "set_location" => {
                if let Some(location) = arguments.get("location").and_then(|v| v.as_str()) {
                    state.location = location.to_string();
                    Ok(())
                } else {
                    Err("Missing 'location' argument".into())
                }
            }
            "set_outfit" => {
                if let Some(outfit) = arguments.get("outfit").and_then(|v| v.as_str()) {
                    state.outfit = outfit.to_string();
                    Ok(())
                } else {
                    Err("Missing 'outfit' argument".into())
                }
            }
            _ => Err(format!("Unknown tool: {}", tool_name).into()),
        }
    }

    /// Create the system prompt for the dungeon master
    fn create_system_prompt() -> String {
        r#"You are a creative and immersive dungeon master for a text-based adventure game.

Your role is to:
1. Generate vivid, engaging narrative text that brings the story to life
2. Always provide exactly 3 distinct choices for the player at the end of your response
3. Use the available tools to naturally update game state (time, location, outfit) as the story progresses
4. Maintain consistency with the current game state and previous events
5. Be creative but responsive to player actions

Available tools:
- set_time: Update time of day (Morning, Afternoon, Evening, Night)
- set_location: Change the player's location
- set_outfit: Update the player's outfit or equipment

Format your responses as narrative text followed by three choices prefixed with numbers:
1. [First choice]
2. [Second choice]  
3. [Third choice]

Use tools when appropriate (e.g., call set_time when time passes, set_location when moving to a new place).

Remember: You are telling an interactive story. Make it memorable!"#.to_string()
    }

    /// Format user message with current state context
    fn format_user_message(&self, action: &str, state: &GameState) -> String {
        format!(
            r#"Current State:
- Time: {}
- Location: {}
- Outfit: {}

Player Action: {}

Continue the story based on this action. Remember to provide exactly 3 choices and use tools to update state if appropriate."#,
            state.time, state.location, state.outfit, action
        )
    }

    /// Extract choices from the model's response
    fn extract_choices(&self, text: &str) -> Vec<String> {
        let mut choices = Vec::new();
        
        // Try to extract numbered choices from the text
        for line in text.lines() {
            let trimmed = line.trim();
            // Match patterns like "1. ", "1) ", "1: "
            if let Some(rest) = trimmed.strip_prefix("1.").or_else(|| trimmed.strip_prefix("1)").or_else(|| trimmed.strip_prefix("1:"))) {
                choices.push(rest.trim().to_string());
            } else if let Some(rest) = trimmed.strip_prefix("2.").or_else(|| trimmed.strip_prefix("2)").or_else(|| trimmed.strip_prefix("2:"))) {
                choices.push(rest.trim().to_string());
            } else if let Some(rest) = trimmed.strip_prefix("3.").or_else(|| trimmed.strip_prefix("3)").or_else(|| trimmed.strip_prefix("3:"))) {
                choices.push(rest.trim().to_string());
            }
        }

        // If we couldn't extract choices, provide defaults
        if choices.len() < 3 {
            choices = vec![
                "Continue exploring".to_string(),
                "Examine your surroundings carefully".to_string(),
                "Take a different approach".to_string(),
            ];
        }

        choices.truncate(3); // Ensure exactly 3 choices
        choices
    }
}

