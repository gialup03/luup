# Text Adventure Game

A cross-platform turn-based text adventure game built with Tauri, React, and TypeScript. Features an extensible backend designed for integration with local LLMs via Ollama.

## Features

- ✨ **Glassmorphism UI** - Apple-like design with translucent glass effects
- 🎮 **Turn-based gameplay** - Navigate through story turns with choices
- 🤖 **Agentic backend** - Full streaming LLM integration with Ollama
- 🔧 **Native tool calling** - Model can modify game state in real-time
- 💭 **Reasoning display** - See the model's thought process
- ⚙️ **Configurable** - Settings page for Ollama server configuration
- 💾 **Save system UI** - Interface for loading previous game saves
- 📱 **Cross-platform** - Runs on Windows, macOS, and Linux

## Project Structure

```
tauri-adventure/
├── src/                          # Frontend React app
│   ├── components/               # Reusable UI components
│   │   ├── TurnDisplay.tsx      # Story text and game state badges
│   │   ├── ChoiceButton.tsx     # Glass-styled choice buttons
│   │   ├── ActionInput.tsx      # Custom action input field
│   │   └── TurnNavigation.tsx   # Turn navigation controls
│   ├── contexts/                 # React Context providers
│   │   ├── GameContext.tsx      # Game state management
│   │   └── SettingsContext.tsx  # Settings management
│   ├── pages/                    # Main application pages
│   │   ├── HomePage.tsx         # New game / load saves
│   │   ├── SettingsPage.tsx     # Ollama configuration
│   │   └── GamePage.tsx         # Main game interface
│   ├── services/                 # Backend communication
│   │   └── backend.ts           # Tauri command wrappers
│   ├── App.tsx                  # Router configuration
│   ├── main.tsx                 # React entry point
│   └── index.css                # Tailwind + custom styles
│
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   └── main.rs              # Tauri commands (stubbed)
│   ├── icons/                   # Application icons
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
│
├── BACKEND_INTEGRATION.md        # Guide for LLM integration
├── start.bat                     # Windows startup script
├── start.ps1                     # PowerShell startup script
└── README.md                     # This file
```

## Getting Started

### Prerequisites

- **Node.js** 20.19+ or 22.12+
- **Rust** 1.70+ (install from [rustup.rs](https://rustup.rs))
- **npm** or **yarn**

### Installation

```bash
# Clone or navigate to the project directory
cd tauri-adventure

# Install dependencies
npm install

# Start development server
npm run tauri:dev
```

Or use the provided startup scripts:

**Windows:**
```cmd
start.bat
```

**PowerShell:**
```powershell
.\start.ps1
```

### Building for Production

```bash
# Build for your current platform
npm run tauri:build
```

Installers will be created in `src-tauri/target/release/bundle/`

## Usage

### Home Page

- Click **"Start New Adventure"** to begin a new game
- View and select from **mock saved games** (click to navigate, persistence not implemented)
- Access **Settings** to configure Ollama

### Game Page

1. Read the story text and view current game state (location, time, outfit)
2. Choose from three AI-generated options (currently stubbed)
3. Or type your own custom action
4. Click **"Submit Choice"** to progress to the next turn
5. Use **Previous/Next** buttons to review turn history

### Settings Page

- Configure your local Ollama server IP address
- Default: `localhost:11434`
- Changes are saved to application state

## Current Implementation Status

### ✅ Completed

- Full UI with glassmorphism design
- Routing between Home, Settings, and Game pages
- Turn navigation system (previous/next)
- Choice selection populates action input
- Game state display (time, location, outfit)
- Settings persistence
- **Agentic Loop System** - Full Ollama integration with streaming
- **Real-time streaming** - Text and reasoning stream as generated
- **Tool execution** - Model can update time, location, outfit
- **Event-driven architecture** - Frontend updates in real-time

### 🚧 In Progress

- **Save/Load** - UI exists, persistence needs implementation
- **Extended tools** - Inventory, quests, etc.
- **Model reasoning** - Better visualization of thought process

## Agentic System

The game now features a complete agentic loop system with:

- **Model Interface Layer** (`ollama.rs`) - Communicates with Ollama API
- **Agentic Loop** (`agent.rs`) - Manages conversation and tool execution  
- **Streaming Events** - Real-time updates to frontend
- **Native Tool Calling** - Ollama's function calling API

See `AGENTIC_SYSTEM.md` for detailed documentation on:
- Architecture and data flow
- Adding new tools
- Customizing the system prompt
- Testing and troubleshooting

Also see `BACKEND_INTEGRATION.md` for integration patterns and advanced usage.

### Quick Example: Adding a Tool Call

**Backend (Rust)**:
```rust
// In src-tauri/src/main.rs
#[derive(Serialize)]
struct ToolCall {
    name: String,
    args: serde_json::Value,
}

// Emit tool call
window.emit("tool-call", ToolCall {
    name: "set_time".to_string(),
    args: json!({ "time": "Evening" }),
})?;
```

**Frontend (TypeScript)**:
```typescript
// In GamePage.tsx
useEffect(() => {
  const unlisten = listen('tool-call', (event) => {
    const tool = event.payload;
    if (tool.name === 'set_time') {
      setGameState(prev => ({ ...prev, time: tool.args.time }));
    }
  });
  return () => { unlisten.then(f => f()); };
}, []);
```

## Tech Stack

- **Frontend**: React 18 + TypeScript + Vite
- **Styling**: Tailwind CSS with custom glassmorphism utilities
- **Desktop**: Tauri 2.0 (Rust + WebView)
- **Routing**: React Router v6
- **State**: React Context API

## Design System

- **Fonts**: Bricolage Grotesque (headings) + Inter (body)
- **Colors**: Slate palette with blue/indigo accents
- **Effects**: Backdrop blur, glassmorphism, gradient overlays
- **Inspired by**: macOS Big Sur, iOS design language

## Performance Notes

- First build takes 5-10 minutes (Rust compilation)
- Subsequent builds are much faster (~30 seconds)
- Production builds are highly optimized
- Tauri apps are 10x smaller than Electron (typically <5MB)

## Troubleshooting

### Vite Node Version Warning

If you see a warning about Node.js version, you can:
1. Upgrade Node to 20.19+ or 22.12+
2. Or ignore it - the app will still work

### Rust Compilation Errors

```bash
# Clean and rebuild
cd src-tauri
cargo clean
cd ..
npm run tauri:dev
```

### Icon Errors

If you get icon-related errors, ensure:
- `src-tauri/icons/icon.ico` exists and is valid
- `src-tauri/icons/icon.icns` exists (for macOS)
- Icon files are proper format (not just renamed PNGs)

## License

MIT

## Future Enhancements

- [x] Ollama HTTP client integration
- [x] Streaming text responses
- [x] Tool call system for game state
- [ ] Save/load persistence with local storage
- [ ] Multi-turn tool execution (tool → result → continue)
- [ ] Character inventory system (extend tools)
- [ ] Health/stats tracking
- [ ] Achievement system
- [ ] Custom themes
- [ ] Sound effects and music
- [ ] Image generation for scenes
- [ ] Llama.cpp integration for local inference
