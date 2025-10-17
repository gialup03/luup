# Backend Integration Guide

## Current Stub Implementation
The current implementation uses hardcoded responses in Rust commands located in `src-tauri/src/main.rs`.

## Architecture Overview

### Message Protocol
The backend uses a structured type system defined in Rust:

```rust
struct TurnData {
    turn_number: u32,
    story_text: String,
    choices: Vec<String>,
    game_state: GameState,
}

struct GameState {
    time: String,
    location: String,
    outfit: String,
    // Extensible: add more fields as needed
}
```

### Current Commands
- `start_new_game()` → Returns session ID, initializes game history
- `get_turn(session_id, turn_number)` → Returns specific turn data
- `submit_action(session_id, action)` → Processes action, generates new turn
- `list_saves()` → Returns mock save games (stubbed)
- `get_ollama_config()` / `set_ollama_config(ip)` → Settings management

## Future: Streaming + Tool Calls

### 1. Streaming Text Implementation

Replace the synchronous `submit_action` with a streaming approach:

**Backend (Rust)**:
```rust
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct StreamMessage {
    message_type: String,  // "text_chunk", "tool_call", "turn_complete"
    payload: serde_json::Value,
}

#[tauri::command]
async fn submit_action_stream(
    window: tauri::Window,
    session_id: String,
    action: String,
) -> Result<(), String> {
    // Connect to Ollama
    let response = ollama_client
        .post(format!("http://{}/api/generate", ollama_ip))
        .json(&request)
        .send()
        .await?;
    
    // Stream chunks
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        window.emit("turn-stream", StreamMessage {
            message_type: "text_chunk".to_string(),
            payload: serde_json::json!({ "content": String::from_utf8_lossy(&chunk) }),
        })?;
    }
    
    window.emit("turn-stream", StreamMessage {
        message_type: "turn_complete".to_string(),
        payload: serde_json::json!({}),
    })?;
    
    Ok(())
}
```

**Frontend (TypeScript)**:
```typescript
import { listen } from '@tauri-apps/api/event';

// In GamePage component
useEffect(() => {
  const unlisten = listen('turn-stream', (event) => {
    const message = event.payload as StreamMessage;
    
    switch (message.message_type) {
      case 'text_chunk':
        setStoryText(prev => prev + message.payload.content);
        break;
      case 'tool_call':
        handleToolCall(message.payload);
        break;
      case 'turn_complete':
        setIsStreaming(false);
        break;
    }
  });
  
  return () => {
    unlisten.then(f => f());
  };
}, []);
```

### 2. Tool Call Protocol

Extend the message system to support tool calls for game state mutations:

**Message Types**:
```typescript
type BackendMessage = 
  | { type: 'text_chunk', content: string }
  | { type: 'tool_call', name: string, args: Record<string, any> }
  | { type: 'choices', choices: string[] }
  | { type: 'turn_complete' };
```

**Example Tool Calls**:
```typescript
// Time progression
{ type: 'tool_call', name: 'set_time', args: { time: 'Evening' } }

// Location change
{ type: 'tool_call', name: 'set_location', args: { location: 'Dark Forest' } }

// Outfit change
{ type: 'tool_call', name: 'set_outfit', args: { outfit: 'Battle Armor' } }

// Inventory management
{ type: 'tool_call', name: 'add_item', args: { item: 'Magic Sword' } }
{ type: 'tool_call', name: 'remove_item', args: { item: 'Rusty Dagger' } }

// Custom game state
{ type: 'tool_call', name: 'set_state', args: { key: 'health', value: 85 } }
```

**Frontend Tool Handler**:
```typescript
const handleToolCall = (payload: { name: string, args: any }) => {
  switch (payload.name) {
    case 'set_time':
      setGameState(prev => ({ ...prev, time: payload.args.time }));
      break;
    case 'set_location':
      setGameState(prev => ({ ...prev, location: payload.args.location }));
      // Optional: Trigger location transition animation
      break;
    case 'set_outfit':
      setGameState(prev => ({ ...prev, outfit: payload.args.outfit }));
      break;
    case 'add_item':
      setInventory(prev => [...prev, payload.args.item]);
      // Optional: Show notification
      toast.success(`Acquired: ${payload.args.item}`);
      break;
    default:
      console.warn('Unknown tool call:', payload.name);
  }
};
```

### 3. Ollama Integration

**Add HTTP Client Dependency**:
```toml
# In Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
```

**Ollama Client Implementation**:
```rust
use reqwest::Client;

async fn call_ollama(
    ollama_ip: &str,
    prompt: String,
    system_prompt: String,
) -> Result<impl Stream<Item = String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let request_body = serde_json::json!({
        "model": "llama2",  // or user-configured model
        "prompt": prompt,
        "system": system_prompt,
        "stream": true,
        "format": "json",  // For structured outputs
    });
    
    let response = client
        .post(format!("http://{}/api/generate", ollama_ip))
        .json(&request_body)
        .send()
        .await?;
    
    // Return byte stream
    Ok(response.bytes_stream())
}
```

**System Prompt Template**:
```rust
const GAME_MASTER_PROMPT: &str = r#"
You are a creative game master for an interactive text adventure game.

RULES:
1. Generate immersive, descriptive story text
2. Always provide exactly 3 choices for the player
3. Use tool calls to update game state when appropriate
4. Be creative but consistent with the game world

AVAILABLE TOOLS:
- set_time(time: string) - Update time of day (Morning, Afternoon, Evening, Night)
- set_location(location: string) - Change player location
- set_outfit(outfit: string) - Change player's outfit/equipment
- add_item(item: string) - Add item to inventory
- remove_item(item: string) - Remove item from inventory

RESPONSE FORMAT:
{
  "story": "Your narrative text here...",
  "choices": ["Choice 1", "Choice 2", "Choice 3"],
  "tool_calls": [
    { "name": "set_time", "args": { "time": "Evening" } }
  ]
}

Current Game State:
- Time: {current_time}
- Location: {current_location}
- Outfit: {current_outfit}

Player Action: {player_action}

Generate the next turn:
"#;
```

### 4. UI Enhancements for Streaming

**Loading States**:
```typescript
// Add to GamePage component
const [isStreaming, setIsStreaming] = useState(false);
const [streamedText, setStreamedText] = useState('');

// Show streaming indicator
{isStreaming && (
  <div className="glass-card p-4 mb-4">
    <div className="flex items-center gap-3">
      <div className="animate-pulse w-2 h-2 bg-blue-500 rounded-full"></div>
      <span className="text-slate-400 text-sm">The tale unfolds...</span>
    </div>
  </div>
)}
```

**Animated Text Reveal**:
```typescript
// Typewriter effect for streamed text
useEffect(() => {
  if (streamedText) {
    // Optional: Add character-by-character reveal animation
    const words = streamedText.split(' ');
    let currentIndex = 0;
    
    const interval = setInterval(() => {
      if (currentIndex < words.length) {
        setDisplayText(words.slice(0, currentIndex + 1).join(' '));
        currentIndex++;
      } else {
        clearInterval(interval);
      }
    }, 50);
    
    return () => clearInterval(interval);
  }
}, [streamedText]);
```

**Tool Call Notifications**:
```typescript
// Visual feedback for tool calls
const showToolCallEffect = (toolName: string, args: any) => {
  switch (toolName) {
    case 'set_location':
      // Fade out/in transition
      setLocationTransition(true);
      setTimeout(() => setLocationTransition(false), 500);
      break;
    case 'add_item':
      // Toast notification
      toast.success(`Acquired: ${args.item}`, {
        icon: '✨',
        position: 'bottom-right',
      });
      break;
  }
};
```

## Testing the Integration

### Local Ollama Setup
```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull llama2

# Run server
ollama serve
```

### Test Prompts
```typescript
// In SettingsPage, add a "Test Connection" button
const testOllamaConnection = async () => {
  try {
    const response = await fetch(`http://${ollamaIp}/api/tags`);
    const data = await response.json();
    console.log('Available models:', data.models);
    alert('Connection successful!');
  } catch (error) {
    alert('Failed to connect to Ollama');
  }
};
```

## Performance Considerations

1. **Debounce rapid tool calls** to avoid UI jank
2. **Queue text chunks** for smooth streaming
3. **Cancel in-flight requests** when user navigates away
4. **Cache turn history** to avoid re-fetching
5. **Limit history size** to prevent memory bloat (keep last 50 turns)

## Security Notes

1. **Validate all tool calls** before applying to game state
2. **Sanitize AI-generated text** to prevent XSS
3. **Rate limit** Ollama requests to prevent abuse
4. **Store Ollama IP securely** (use Tauri's store plugin)

## Next Steps

1. Replace stubbed commands with Ollama HTTP client
2. Implement streaming event system
3. Add tool call parsing and validation
4. Create UI animations for state changes
5. Add error handling and retry logic
6. Implement save/load with serialized game state
