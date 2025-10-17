# Agentic Loop System Documentation

## Overview

This document describes the agentic loop system implemented for the text adventure game. The system provides a flexible abstraction layer between the Ollama LLM interface and the game logic, with real-time streaming of text, reasoning, and tool calls.

## Architecture

### Layer 1: Model Interface (`ollama.rs`)

The Ollama client provides a clean interface to communicate with the Ollama API:

- **Endpoint**: Hardcoded to `192.168.0.100:11434`
- **Model**: `qwen2.5:3b` (can be changed in the code)
- **Streaming**: Native NDJSON streaming support
- **Tool Calling**: Uses Ollama's native function calling API

#### Key Components

1. **OllamaClient**: Manages HTTP connections and streaming
2. **Tool Definition**: Structures that match Ollama's expected format
3. **Stream Parser**: Parses NDJSON responses into typed chunks

#### Tool Format

Tools are defined using Ollama's native format:

```rust
Tool {
    tool_type: "function",
    function: ToolFunction {
        name: "set_time",
        description: "Update the time of day in the game world",
        parameters: ToolParameters {
            param_type: "object",
            required: ["time"],
            properties: { /* JSON schema */ }
        }
    }
}
```

### Layer 2: Agentic Loop (`agent.rs`)

The agent manages the conversation flow and game state:

- **Conversation History**: Maintains full chat context
- **System Prompt**: Dungeon master persona with clear instructions
- **Tool Registry**: Defines and executes available tools
- **State Management**: Updates game state based on tool calls

#### Agent Flow

1. User submits an action
2. Agent formats prompt with current game state
3. Calls Ollama with tool definitions
4. Streams responses chunk by chunk
5. Executes tool calls when received
6. Emits progress to frontend
7. Completes turn when done

#### Available Tools

1. **set_time**: Changes time of day (Morning/Afternoon/Evening/Night)
2. **set_location**: Updates player's current location
3. **set_outfit**: Changes player's outfit or equipment

#### System Prompt

The system prompt instructs the model to:
- Generate immersive narrative text
- Always provide exactly 3 choices
- Use tools naturally to update state
- Maintain consistency with game world

### Layer 3: Backend Integration (`main.rs`)

The Tauri backend exposes commands and manages state:

- **AppState**: Global state with mutexes for thread safety
- **Commands**: Tauri commands callable from frontend
- **Event Emission**: Streams messages to frontend in real-time

#### Stream Event Flow

```
Backend (Rust)              Frontend (TypeScript)
     |                              |
     |-- text_chunk ------------->  | Append to displayed text
     |-- reasoning_chunk --------->  | Show in reasoning panel
     |-- tool_call -------------->   | Display tool progress
     |-- tool_result ------------>   | Update game state UI
     |-- turn_complete ---------->   | Finalize turn, add to history
```

### Layer 4: Frontend (`GamePage.tsx`)

The frontend handles streaming UI updates:

- **Event Listener**: Subscribes to `agent-stream` events
- **Real-time Updates**: Displays text as it streams
- **Tool Visualization**: Shows tool calls in progress
- **Reasoning Panel**: Expandable section for model reasoning

## Message Types

### AgentMessage (Rust → Frontend)

```typescript
type AgentStreamMessage =
  | { type: 'text_chunk'; content: string }
  | { type: 'reasoning_chunk'; content: string }
  | { type: 'tool_call'; name: string; args: any }
  | { type: 'tool_result'; name: string; result: GameState }
  | { type: 'turn_complete'; turn_number: number; ... }
  | { type: 'error'; message: string };
```

## Usage

### Starting a New Game

```typescript
const sessionId = await backend.startNewGame();
// Initializes agent, clears history, returns session ID
```

### Submitting an Action (Streaming)

```typescript
await backend.submitActionStream(sessionId, "I open the blue door");
// Triggers agentic loop, streams responses via events
```

### Listening to Stream Events

```typescript
const unlisten = await listen('agent-stream', (event) => {
  const message = event.payload;
  // Handle different message types
});
```

## Extending the System

### Adding New Tools

1. **Define the tool in `ollama.rs`**:

```rust
Tool {
    tool_type: "function".to_string(),
    function: ToolFunction {
        name: "add_item".to_string(),
        description: "Add an item to inventory".to_string(),
        parameters: // ... parameter schema
    }
}
```

2. **Implement execution in `agent.rs`**:

```rust
fn execute_tool(&self, tool_name: &str, arguments: &Value, state: &mut GameState) {
    match tool_name {
        "add_item" => {
            // Extract argument and update state
        },
        // ... other tools
    }
}
```

3. **Update GameState type if needed**:

```rust
pub struct GameState {
    pub time: String,
    pub location: String,
    pub outfit: String,
    pub inventory: Vec<String>, // New field
}
```

### Customizing the System Prompt

Edit `Agent::create_system_prompt()` in `agent.rs`:

```rust
fn create_system_prompt() -> String {
    r#"You are a [custom persona]...
    
    Your role is to:
    - [Custom instruction 1]
    - [Custom instruction 2]
    
    Available tools:
    - [tool list]
    "#.to_string()
}
```

### Switching Models

Change the model in `OllamaClient::new()` in `ollama.rs`:

```rust
Self {
    base_url: "http://192.168.0.100:11434".to_string(),
    model: "llama3:8b".to_string(), // Change here
    http_client: reqwest::Client::new(),
}
```

## Testing

### Prerequisites

1. Ollama must be running at `192.168.0.100:11434`
2. Model `qwen2.5:3b` must be pulled: `ollama pull qwen2.5:3b`

### Testing Connection

From the Settings page, you can verify the Ollama configuration. The client will attempt to connect when you submit an action.

### Expected Behavior

1. Submit an action
2. See streaming indicator appear
3. Text streams in word by word
4. Tool calls show as "Updating [tool]..."
5. Game state updates in real-time
6. Turn completes, choices appear

## Error Handling

- **Connection Errors**: Displayed as red alert in UI
- **Tool Execution Errors**: Logged but don't crash the loop
- **Stream Parse Errors**: Emitted as error messages to frontend

## Performance Notes

- **Latency**: First token typically arrives within 500ms-2s
- **Streaming**: Provides responsive feel even for long responses
- **State Locking**: Mutexes ensure thread safety but may add minimal overhead
- **Memory**: Conversation history grows with turns (consider limiting in production)

## Future Improvements

1. **Multi-turn Tool Execution**: Allow model to call tools, see results, and continue
2. **Tool Call Confirmation**: Ask user before executing certain tools
3. **Reasoning Display**: Better formatting for chain-of-thought output
4. **Save/Load State**: Persist conversation history with game saves
5. **Dynamic Tool Registration**: Load tools from config file
6. **Llama.cpp Integration**: Local inference without Ollama dependency
7. **Streaming Cancellation**: Allow user to stop generation mid-stream

## Troubleshooting

### "Failed to connect to the storyteller"

- Check Ollama is running: `curl http://192.168.0.100:11434/api/tags`
- Verify model is available: `ollama list`
- Check firewall settings if on different machine

### Model not using tools

- Verify model supports function calling (qwen2.5 does)
- Check system prompt has clear tool instructions
- Try more explicit user prompts

### Streaming appears slow

- Check network latency to Ollama server
- Try a smaller model
- Increase Ollama's concurrent request limit

### No reasoning chunks appearing

- Reasoning detection looks for `<think>` tags or "reasoning:" prefix
- Modify system prompt to encourage reasoning output
- Or adjust detection logic in `ollama.rs`

## Credits

Built using:
- **Ollama** for LLM inference
- **Qwen 2.5** for the language model
- **Tauri** for the desktop framework
- **Rust** for high-performance backend
- **React** for the frontend UI

