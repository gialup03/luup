# Quick Start Guide - Agentic System

This guide will help you get the agentic loop system up and running.

## Prerequisites

1. **Ollama Server Running**
   - Must be accessible at `192.168.0.100:11434`
   - Or modify the hardcoded IP in `src-tauri/src/ollama.rs`

2. **Model Downloaded**
   ```bash
   ollama pull qwen2.5:3b
   ```

3. **Development Environment**
   - Node.js 20.19+ or 22.12+
   - Rust 1.70+
   - npm or yarn

## Installation

1. **Install Dependencies**
   ```bash
   npm install
   ```

2. **Build and Run**
   ```bash
   npm run tauri:dev
   ```

   Or use the startup scripts:
   - Windows: `start.bat`
   - PowerShell: `.\start.ps1`

## Testing the Agentic System

### 1. Start a New Game

1. Click "Start New Adventure" from the home page
2. You'll see the initial game state:
   - Location: Mysterious Room
   - Time: Morning
   - Outfit: Traveler's Cloak

### 2. Submit an Action

1. Either click one of the suggested choices, OR
2. Type your own custom action
3. Click "Submit Choice"

### 3. Watch the Stream

You should see:

- **Streaming Indicator**: Blue pulsing dot with "The tale unfolds..."
- **Text Streaming**: Story text appears word by word
- **Tool Calls**: If the model updates state, you'll see:
  - Spinning indicator: "Updating time..."
  - Game state badges update in real-time
- **Reasoning** (if available): Purple panel with model's thought process
- **Choices**: Three new choices appear when complete

### 4. Verify Tool Execution

Try actions that should trigger state changes:

- **Time progression**: "I rest for several hours"
  - Should call `set_time` to update time of day
  
- **Location change**: "I leave the room and enter the hallway"
  - Should call `set_location` to update location
  
- **Outfit change**: "I put on the armor hanging on the wall"
  - Should call `set_outfit` to update outfit

## Troubleshooting

### "Failed to connect to the storyteller"

**Problem**: Cannot reach Ollama server

**Solutions**:
1. Verify Ollama is running:
   ```bash
   curl http://192.168.0.100:11434/api/tags
   ```

2. Check if model is available:
   ```bash
   ollama list
   ```

3. If Ollama is on `localhost`, update `src-tauri/src/ollama.rs`:
   ```rust
   base_url: "http://localhost:11434".to_string(),
   ```

### No Tool Calls Happening

**Problem**: Model generates text but doesn't use tools

**Possible Causes**:
1. Model doesn't support function calling (qwen2.5 does)
2. Prompts aren't triggering tool use
3. System prompt needs adjustment

**Solutions**:
1. Try more explicit actions: "Change the time to evening"
2. Check system prompt in `src-tauri/src/agent.rs`
3. Look at Rust console output for tool call detection

### Streaming is Very Slow

**Problem**: Text appears slowly or with long delays

**Solutions**:
1. Check network latency: `ping 192.168.0.100`
2. Try a smaller/faster model:
   ```rust
   model: "qwen2.5:1.5b".to_string(),
   ```
3. Check Ollama server resources (CPU/GPU usage)

### Reasoning Not Appearing

**Problem**: Purple reasoning panel never shows

**Explanation**: 
- Reasoning detection looks for `<think>` tags or "reasoning:" prefix
- Not all models output reasoning by default
- Qwen may not always use these formats

**Solutions**:
1. Modify system prompt to request explicit reasoning
2. Adjust detection logic in `ollama.rs` to match your model's format
3. Or simply note that this feature is optional

## Configuration

### Change Ollama Endpoint

Edit `src-tauri/src/ollama.rs`:
```rust
pub fn new() -> Self {
    Self {
        base_url: "http://YOUR_IP:11434".to_string(),
        // ...
    }
}
```

### Change Model

Edit `src-tauri/src/ollama.rs`:
```rust
pub fn new() -> Self {
    Self {
        // ...
        model: "llama3:8b".to_string(),  // Your model
        // ...
    }
}
```

### Customize System Prompt

Edit `src-tauri/src/agent.rs`:
```rust
fn create_system_prompt() -> String {
    r#"Your custom dungeon master prompt here..."#.to_string()
}
```

## Development

### Rebuilding After Changes

Rust changes require a rebuild:
```bash
# Stop the dev server (Ctrl+C)
# Restart it
npm run tauri:dev
```

Frontend (TypeScript/React) changes hot-reload automatically.

### Viewing Logs

**Rust Backend**:
- Logs appear in the terminal where you ran `npm run tauri:dev`

**Frontend**:
- Open DevTools in the app window (F12 or Cmd+Option+I)
- Check Console tab

### Adding Debug Output

In Rust:
```rust
println!("Debug: {:?}", some_value);
eprintln!("Error: {:?}", error);
```

In TypeScript:
```typescript
console.log('Debug:', value);
console.error('Error:', error);
```

## Example Session

Here's what a successful session looks like:

1. **Start**: "You wake up in a dimly lit room..."
2. **Action**: "I examine the blue door more closely"
3. **Stream Start**: Blue indicator appears
4. **Text Streams**: "As you approach the door, you notice..."
5. **Tool Call**: "Updating time..." (if evening falls)
6. **State Updates**: Time badge changes to "Evening"
7. **Stream Complete**: Three new choices appear
8. **Ready**: Input is enabled again

## Next Steps

Once the basic system is working:

1. **Experiment with prompts** - Try different actions to see tool usage
2. **Customize the system prompt** - Make it fit your game world
3. **Add new tools** - Extend the game mechanics (see `AGENTIC_SYSTEM.md`)
4. **Improve UI** - Enhance the streaming visualizations
5. **Implement save/load** - Persist game state between sessions

## Getting Help

If you encounter issues:

1. Check `AGENTIC_SYSTEM.md` for architecture details
2. Review `BACKEND_INTEGRATION.md` for integration patterns
3. Check Rust compiler errors in the terminal
4. Look for JavaScript errors in DevTools console
5. Verify Ollama server is responding correctly

## Performance Tips

- **First build**: Takes 5-10 minutes (Rust compilation)
- **Subsequent builds**: ~30 seconds
- **Hot reload**: Frontend changes are instant
- **First request**: May be slower as model loads into memory
- **Streaming**: Should feel responsive after first token

Happy adventuring! 🎮

