# Testing the Agentic Loop System

## Summary of Changes

✅ **Model Updated**: Changed from `qwen2.5:3b` to `qwen3:8b` in `src-tauri/src/ollama.rs`
✅ **Ollama Connection Verified**: Server running at `192.168.0.100:11434` with qwen3:8b available

## Implementation Complete

All files have been created and configured:

### Rust Backend (3 files)
1. ✅ `src-tauri/src/ollama.rs` - Ollama client with streaming
2. ✅ `src-tauri/src/agent.rs` - Agentic loop system  
3. ✅ `src-tauri/src/main.rs` - Updated with streaming commands

### Frontend (2 files)
4. ✅ `src/services/backend.ts` - Stream message types
5. ✅ `src/pages/GamePage.tsx` - Streaming UI

### Documentation (4 files)
6. ✅ `AGENTIC_SYSTEM.md` - Architecture documentation
7. ✅ `QUICKSTART.md` - Setup guide
8. ✅ `IMPLEMENTATION_SUMMARY.md` - Technical details
9. ✅ `README.md` - Updated features

## To Test

### Step 1: Build and Run

Open a terminal in the project directory and run:

```bash
npm run tauri:dev
```

**Expected**: 
- First build takes 5-10 minutes (Rust compilation)
- Window opens with the game interface

### Step 2: Start New Game

1. Click "Start New Adventure"
2. Verify initial state:
   - Location: Mysterious Room
   - Time: Morning
   - Outfit: Traveler's Cloak

### Step 3: Test Streaming

Submit an action, for example:
- "I open the blue door and step through"
- "I examine the room carefully looking for clues"
- "I change into more suitable attire"

**Expected Behavior**:

1. **Streaming Indicator** appears (blue pulsing dot)
2. **Text streams in** word by word
3. **Tool calls** may appear:
   - "Updating time..." (if time changes)
   - "Updating location..." (if location changes)
   - "Updating outfit..." (if outfit changes)
4. **Game state badges** update in real-time
5. **Three new choices** appear when complete
6. **Input is enabled** again

### Step 4: Verify Tool Execution

Try actions that should trigger tools:

**For time change**:
- "I rest for several hours"
- "I wait until evening"

**For location change**:  
- "I leave the room and explore the hallway"
- "I descend the stairs into the cellar"

**For outfit change**:
- "I put on the armor from the chest"
- "I change into the robes I found"

Watch the game state badges (top of story card) update in real-time.

### Step 5: Check Reasoning (Optional)

If the model outputs reasoning:
- Look for the purple "Model Reasoning" panel
- Click to expand and see the model's thought process

## Troubleshooting

### Build Fails

If you get compilation errors:

1. **Check Cargo.toml dependencies** are correct
2. **Clean build**:
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   npm run tauri:dev
   ```

### "Failed to connect to the storyteller"

If this error appears:

1. **Verify Ollama is running**:
   ```bash
   curl http://192.168.0.100:11434/api/tags
   ```

2. **Check model is available**:
   ```bash
   ollama list | grep qwen3
   ```

3. **Check firewall** if Ollama is on different machine

### No Response from Model

If submission hangs:

1. Check terminal output for Rust errors
2. Open DevTools (F12) and check Console for JS errors
3. Verify Ollama server isn't overloaded

### Tool Calls Not Working

If state doesn't update:

1. Check if model supports function calling (qwen3:8b does)
2. Look at terminal for tool execution logs
3. Try more explicit prompts like "Change time to evening"

## Expected Console Output

### Backend (Terminal)

You should see:
```
Starting development server...
Compiling tauri-adventure...
Running `target/debug/tauri-adventure`
```

When you submit an action, you might see debug output if added.

### Frontend (DevTools Console)

Should be minimal. If you see errors about:
- `agent-stream` events - check event listener setup
- Type errors - TypeScript build issue
- Network errors - Ollama connection problem

## Success Criteria

✅ App compiles and runs
✅ New game starts successfully  
✅ Streaming indicator appears on submit
✅ Text appears progressively
✅ Tool calls show progress indicators
✅ Game state updates in real-time
✅ Turn completes with new choices
✅ No errors in console

## What to Look For

### Visual Indicators

1. **Blue pulsing dot** = Streaming in progress
2. **Spinning circle** = Tool executing
3. **Purple panel** = Model reasoning available
4. **Red alert** = Error occurred
5. **State badges** change = Tool executed successfully

### Performance

- **First token**: 500ms-2s after submit
- **Streaming rate**: Smooth, progressive
- **UI responsiveness**: No freezing
- **Tool execution**: Near-instant state updates

## Next Steps After Testing

Once everything works:

1. **Experiment with prompts** - See what triggers tool calls
2. **Customize system prompt** - Edit `src-tauri/src/agent.rs`
3. **Add more tools** - Inventory, quests, etc.
4. **Improve choices** - Better extraction from model output
5. **Add persistence** - Save/load game state

## Files to Check if Issues

### Compilation errors:
- `src-tauri/Cargo.toml` - Dependencies
- `src-tauri/src/*.rs` - Syntax errors

### Runtime errors:
- `src-tauri/src/ollama.rs` - Connection/streaming
- `src-tauri/src/agent.rs` - Tool execution
- `src/pages/GamePage.tsx` - Event handling

### Connection errors:
- `src-tauri/src/ollama.rs:77` - Base URL
- `src-tauri/src/ollama.rs:78` - Model name

## Testing Checklist

- [ ] App compiles without errors
- [ ] App launches and shows home page
- [ ] Can start new game
- [ ] Can submit an action
- [ ] Streaming indicator appears
- [ ] Text streams progressively
- [ ] Tool calls display
- [ ] State updates correctly
- [ ] Turn completes successfully
- [ ] Can submit multiple turns
- [ ] Can navigate turn history
- [ ] No console errors

Good luck testing! The system should work seamlessly with your qwen3:8b model. 🎮

