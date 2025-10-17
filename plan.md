# Spec Provenance

Created: 2025-10-17
Request: Build a cross-platform turn-based text adventure game with Tauri, stubbed agentic backend (local LLM via Ollama), and Apple-like UI. Features: home page (new game/load saves), settings (Ollama IP config), game page (turn navigation, AI-generated choices, custom action input).

# Spec Header

**Name**: Luup  
**Smallest Scope**: Complete desktop app with three screens (home, settings, game) and extensible backend interface  
**Non-Goals**: 
- Actual LLM integration in this phase (stubbed responses only)
- Save game persistence logic (UI only)
- Game state persistence between sessions
- Multi-user support
- Cloud sync

# Paths to Supplementary Guidelines

**Design System**: https://raw.githubusercontent.com/memextech/templates/refs/heads/main/design/glassmorphism-enterprise.md
- Premium translucent glass effects
- Bricolage Grotesque + Inter typography
- Dark gradient backgrounds with glass overlays
- Perfect for Apple-like aesthetic

# Decision Snapshot

## Tech Stack
- **Framework**: Tauri 1.5+ (cross-platform desktop)
- **Frontend**: React 18 + TypeScript + Vite
- **Styling**: Tailwind CSS with custom glassmorphism utilities
- **State Management**: React Context (lightweight, sufficient for game state)
- **Backend Interface**: Rust commands via Tauri IPC
- **Future-Ready**: Message protocol designed for streaming + tool calls

## Key Architectural Decisions

**1. Extensible Backend Protocol**  
Design a message-based protocol that can handle:
- Text chunks (for streaming)
- Tool calls (game state mutations: time, location, outfit, etc.)
- Turn metadata (choices, story text)

```typescript
interface BackendMessage {
  type: 'text_chunk' | 'choices' | 'tool_call' | 'turn_complete';
  payload: any;
}
```

**2. Tauri Command Structure**  
- `start_new_game()` → Returns game session ID
- `get_turn(session_id, turn_number)` → Returns turn data
- `submit_action(session_id, action)` → Processes action, returns new turn
- `get_ollama_config()` / `set_ollama_config(ip)` → Settings management
- `list_saves()` → Returns save game list (stubbed)

**3. Component Architecture**  
```
src/
├── App.tsx                 # Main router
├── contexts/
│   ├── GameContext.tsx     # Game state (current turn, session, etc.)
│   └── SettingsContext.tsx # Ollama config
├── pages/
│   ├── HomePage.tsx        # New game / load saves
│   ├── SettingsPage.tsx    # Ollama IP configuration
│   └── GamePage.tsx        # Main game interface
├── components/
│   ├── TurnDisplay.tsx     # Story text + choices
│   ├── ChoiceButton.tsx    # Glass-styled choice buttons
│   ├── ActionInput.tsx     # Custom action textbox
│   └── TurnNavigation.tsx  # Previous/Next turn buttons
└── services/
    └── backend.ts          # Tauri command wrappers
```

**4. Glass Design Implementation**  
- Background: Dark gradient (slate-900 to slate-800)
- Cards: `rgba(255, 255, 255, 0.08)` with `backdrop-blur-2xl`
- Buttons: Glass effect with gradient accents
- Typography: Bricolage Grotesque for headings, Inter for body

## Why This Stack?

✅ **Tauri over Electron**: 10x smaller bundle, Rust performance, native system integration  
✅ **React + Vite**: Fast dev experience, familiar patterns  
✅ **Tailwind**: Rapid glassmorphism styling without heavy CSS  
✅ **Context over Redux**: Game state is simple, no over-engineering  
✅ **Message protocol**: Decouples UI from backend implementation, enables streaming later

# Architecture at a Glance

```
┌─────────────────────────────────────────┐
│         Tauri Window (React)            │
│  ┌───────────────────────────────────┐  │
│  │  HomePage                         │  │
│  │  - New Game button                │  │
│  │  - Save slots (stubbed list)      │  │
│  │  - Settings button                │  │
│  └───────────────────────────────────┘  │
│                  ↓                       │
│  ┌───────────────────────────────────┐  │
│  │  SettingsPage                     │  │
│  │  - Ollama IP input                │  │
│  │  - Save/Back buttons              │  │
│  └───────────────────────────────────┘  │
│                  ↓                       │
│  ┌───────────────────────────────────┐  │
│  │  GamePage                         │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │ Story text (generated)      │  │  │
│  │  └─────────────────────────────┘  │  │
│  │  [Choice 1] [Choice 2] [Choice 3]│  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │ Custom action input         │  │  │
│  │  └─────────────────────────────┘  │  │
│  │  [← Prev] [Submit] [Next →]     │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
                  ↕ Tauri IPC
┌─────────────────────────────────────────┐
│       Rust Backend (Tauri Core)         │
│  ┌───────────────────────────────────┐  │
│  │ Commands (stubbed):               │  │
│  │ - start_new_game()                │  │
│  │ - get_turn(session, turn_num)     │  │
│  │ - submit_action(session, action)  │  │
│  │ - get/set_ollama_config()         │  │
│  │ - list_saves()                    │  │
│  └───────────────────────────────────┘  │
│                                          │
│  Stubbed responses return:               │
│  - Hardcoded story text                  │
│  - 3 preset choices                      │
│  - Mock game state                       │
└─────────────────────────────────────────┘

Future: Replace stubs with Ollama HTTP client,
        stream responses, process tool calls
```

# Implementation Plan

## Phase 1: Project Scaffolding

**1.1 Initialize Tauri Project**
```bash
npm create tauri-app@latest
# Choose: React + TypeScript + Vite
cd [project-name]
npm install
```

**1.2 Install Dependencies**
```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
npm install react-router-dom
```

**1.3 Configure Tailwind with Glassmorphism**

`tailwind.config.js`:
```js
export default {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        heading: ['Bricolage Grotesque', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
      },
      colors: {
        glass: {
          base: 'rgba(255, 255, 255, 0.05)',
          elevated: 'rgba(255, 255, 255, 0.08)',
          hover: 'rgba(255, 255, 255, 0.12)',
        },
      },
      backdropBlur: {
        glass: '20px',
        'glass-xl': '24px',
      },
    },
  },
  plugins: [],
}
```

`src/index.css`:
```css
@import url('https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:wght@500;600;700&family=Inter:wght@400;500;600&display=swap');

@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-gradient-to-br from-slate-900 to-slate-800 text-slate-50 font-body;
  }
}

@layer components {
  .glass-card {
    @apply bg-glass-elevated backdrop-blur-glass-xl rounded-2xl border border-white/10;
  }
  
  .glass-button {
    @apply bg-glass-base backdrop-blur-glass hover:bg-glass-hover rounded-xl border border-white/10 transition-all duration-200;
  }
}
```

## Phase 2: Backend Interface (Rust)

**2.1 Define Message Protocol**

`src-tauri/src/types.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnData {
    pub turn_number: u32,
    pub story_text: String,
    pub choices: Vec<String>,
    pub game_state: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub time: String,
    pub location: String,
    pub outfit: String,
    // Extensible: add more fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub id: String,
    pub name: String,
    pub last_played: String,
    pub turn_count: u32,
}
```

**2.2 Implement Stubbed Commands**

`src-tauri/src/main.rs`:
```rust
mod types;
use types::{TurnData, GameState, OllamaConfig, SaveGame};
use tauri::State;
use std::sync::Mutex;

struct AppState {
    ollama_config: Mutex<OllamaConfig>,
    game_history: Mutex<Vec<TurnData>>,
}

#[tauri::command]
fn start_new_game(state: State<AppState>) -> Result<String, String> {
    let mut history = state.game_history.lock().unwrap();
    history.clear();
    
    // Add initial turn
    history.push(TurnData {
        turn_number: 0,
        story_text: "You wake up in a dimly lit room. The air smells of old parchment and something... magical. Three doors stand before you, each humming with a different energy.".to_string(),
        choices: vec![
            "Open the door radiating blue light".to_string(),
            "Open the door with ancient runes carved into it".to_string(),
            "Open the plain wooden door".to_string(),
        ],
        game_state: GameState {
            time: "Morning".to_string(),
            location: "Mysterious Room".to_string(),
            outfit: "Traveler's Cloak".to_string(),
        },
    });
    
    Ok("session_stub_001".to_string())
}

#[tauri::command]
fn get_turn(session_id: String, turn_number: u32, state: State<AppState>) -> Result<TurnData, String> {
    let history = state.game_history.lock().unwrap();
    
    if let Some(turn) = history.get(turn_number as usize) {
        Ok(turn.clone())
    } else {
        Err("Turn not found".to_string())
    }
}

#[tauri::command]
fn submit_action(
    session_id: String,
    action: String,
    state: State<AppState>,
) -> Result<TurnData, String> {
    let mut history = state.game_history.lock().unwrap();
    let current_turn = history.len() as u32;
    
    // Stub: Generate next turn based on action
    let new_turn = TurnData {
        turn_number: current_turn,
        story_text: format!("You chose: '{}'. The path unfolds before you, revealing new mysteries and dangers. What will you do next?", action),
        choices: vec![
            "Investigate the strange sound".to_string(),
            "Continue forward cautiously".to_string(),
            "Rest and assess your surroundings".to_string(),
        ],
        game_state: GameState {
            time: "Afternoon".to_string(),
            location: "Enchanted Corridor".to_string(),
            outfit: "Traveler's Cloak".to_string(),
        },
    };
    
    history.push(new_turn.clone());
    Ok(new_turn)
}

#[tauri::command]
fn list_saves() -> Result<Vec<SaveGame>, String> {
    // Stub: Return mock save games
    Ok(vec![
        SaveGame {
            id: "save_001".to_string(),
            name: "The Forest Adventure".to_string(),
            last_played: "2025-10-15".to_string(),
            turn_count: 23,
        },
        SaveGame {
            id: "save_002".to_string(),
            name: "Castle Siege".to_string(),
            last_played: "2025-10-14".to_string(),
            turn_count: 15,
        },
    ])
}

#[tauri::command]
fn get_ollama_config(state: State<AppState>) -> Result<OllamaConfig, String> {
    let config = state.ollama_config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
fn set_ollama_config(ip_address: String, state: State<AppState>) -> Result<(), String> {
    let mut config = state.ollama_config.lock().unwrap();
    config.ip_address = ip_address;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            ollama_config: Mutex::new(OllamaConfig {
                ip_address: "localhost:11434".to_string(),
            }),
            game_history: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_new_game,
            get_turn,
            submit_action,
            list_saves,
            get_ollama_config,
            set_ollama_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Phase 3: Frontend Implementation

**3.1 Service Layer**

`src/services/backend.ts`:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

export interface TurnData {
  turn_number: number;
  story_text: string;
  choices: string[];
  game_state: GameState;
}

export interface GameState {
  time: string;
  location: string;
  outfit: string;
}

export interface SaveGame {
  id: string;
  name: string;
  last_played: string;
  turn_count: number;
}

export interface OllamaConfig {
  ip_address: string;
}

export const backend = {
  async startNewGame(): Promise<string> {
    return await invoke('start_new_game');
  },

  async getTurn(sessionId: string, turnNumber: number): Promise<TurnData> {
    return await invoke('get_turn', { sessionId, turnNumber });
  },

  async submitAction(sessionId: string, action: string): Promise<TurnData> {
    return await invoke('submit_action', { sessionId, action });
  },

  async listSaves(): Promise<SaveGame[]> {
    return await invoke('list_saves');
  },

  async getOllamaConfig(): Promise<OllamaConfig> {
    return await invoke('get_ollama_config');
  },

  async setOllamaConfig(ipAddress: string): Promise<void> {
    return await invoke('set_ollama_config', { ipAddress });
  },
};
```

**3.2 Context Providers**

`src/contexts/GameContext.tsx`:
```typescript
import React, { createContext, useContext, useState, ReactNode } from 'react';
import { TurnData } from '../services/backend';

interface GameContextType {
  sessionId: string | null;
  currentTurn: TurnData | null;
  turnHistory: TurnData[];
  currentTurnIndex: number;
  setSessionId: (id: string) => void;
  addTurn: (turn: TurnData) => void;
  navigateToTurn: (index: number) => void;
}

const GameContext = createContext<GameContextType | undefined>(undefined);

export const GameProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [turnHistory, setTurnHistory] = useState<TurnData[]>([]);
  const [currentTurnIndex, setCurrentTurnIndex] = useState<number>(0);

  const addTurn = (turn: TurnData) => {
    setTurnHistory((prev) => [...prev, turn]);
    setCurrentTurnIndex(turnHistory.length);
  };

  const navigateToTurn = (index: number) => {
    if (index >= 0 && index < turnHistory.length) {
      setCurrentTurnIndex(index);
    }
  };

  const currentTurn = turnHistory[currentTurnIndex] || null;

  return (
    <GameContext.Provider
      value={{
        sessionId,
        currentTurn,
        turnHistory,
        currentTurnIndex,
        setSessionId,
        addTurn,
        navigateToTurn,
      }}
    >
      {children}
    </GameContext.Provider>
  );
};

export const useGame = () => {
  const context = useContext(GameContext);
  if (!context) throw new Error('useGame must be used within GameProvider');
  return context;
};
```

`src/contexts/SettingsContext.tsx`:
```typescript
import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { backend } from '../services/backend';

interface SettingsContextType {
  ollamaIp: string;
  setOllamaIp: (ip: string) => void;
  saveSettings: () => Promise<void>;
}

const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export const SettingsProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [ollamaIp, setOllamaIp] = useState<string>('localhost:11434');

  useEffect(() => {
    backend.getOllamaConfig().then((config) => setOllamaIp(config.ip_address));
  }, []);

  const saveSettings = async () => {
    await backend.setOllamaConfig(ollamaIp);
  };

  return (
    <SettingsContext.Provider value={{ ollamaIp, setOllamaIp, saveSettings }}>
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = () => {
  const context = useContext(SettingsContext);
  if (!context) throw new Error('useSettings must be used within SettingsProvider');
  return context;
};
```

**3.3 Components**

`src/components/TurnDisplay.tsx`:
```typescript
import React from 'react';
import { GameState } from '../services/backend';

interface TurnDisplayProps {
  storyText: string;
  gameState: GameState;
}

export const TurnDisplay: React.FC<TurnDisplayProps> = ({ storyText, gameState }) => {
  return (
    <div className="glass-card p-8 mb-6">
      {/* Game State Badge */}
      <div className="flex gap-3 mb-6">
        <span className="px-4 py-2 bg-glass-base backdrop-blur-glass rounded-full text-sm text-slate-300 border border-white/10">
          📍 {gameState.location}
        </span>
        <span className="px-4 py-2 bg-glass-base backdrop-blur-glass rounded-full text-sm text-slate-300 border border-white/10">
          🕐 {gameState.time}
        </span>
        <span className="px-4 py-2 bg-glass-base backdrop-blur-glass rounded-full text-sm text-slate-300 border border-white/10">
          👔 {gameState.outfit}
        </span>
      </div>

      {/* Story Text */}
      <p className="text-lg leading-relaxed text-slate-50 font-body">{storyText}</p>
    </div>
  );
};
```

`src/components/ChoiceButton.tsx`:
```typescript
import React from 'react';

interface ChoiceButtonProps {
  choice: string;
  onClick: () => void;
}

export const ChoiceButton: React.FC<ChoiceButtonProps> = ({ choice, onClick }) => {
  return (
    <button
      onClick={onClick}
      className="glass-button px-6 py-4 text-left w-full hover:scale-[1.02] active:scale-[0.98]"
    >
      <span className="text-slate-50 font-medium">{choice}</span>
    </button>
  );
};
```

`src/components/ActionInput.tsx`:
```typescript
import React from 'react';

interface ActionInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  disabled?: boolean;
}

export const ActionInput: React.FC<ActionInputProps> = ({
  value,
  onChange,
  onSubmit,
  disabled,
}) => {
  return (
    <div className="glass-card p-6">
      <label className="block text-sm font-medium text-slate-300 mb-3">
        Your Action
      </label>
      <textarea
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' && e.metaKey) {
            onSubmit();
          }
        }}
        disabled={disabled}
        placeholder="Type your custom action here, or click a choice above..."
        className="w-full bg-glass-base backdrop-blur-glass rounded-xl border border-white/10 p-4 text-slate-50 placeholder-slate-500 focus:outline-none focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20 resize-none"
        rows={3}
      />
      <p className="text-xs text-slate-400 mt-2">⌘ + Enter to submit</p>
    </div>
  );
};
```

`src/components/TurnNavigation.tsx`:
```typescript
import React from 'react';

interface TurnNavigationProps {
  currentIndex: number;
  totalTurns: number;
  onPrevious: () => void;
  onNext: () => void;
  onSubmit: () => void;
  isCurrentTurn: boolean;
}

export const TurnNavigation: React.FC<TurnNavigationProps> = ({
  currentIndex,
  totalTurns,
  onPrevious,
  onNext,
  onSubmit,
  isCurrentTurn,
}) => {
  return (
    <div className="flex justify-between items-center gap-4">
      <button
        onClick={onPrevious}
        disabled={currentIndex === 0}
        className="glass-button px-6 py-3 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        <span>←</span>
        <span>Previous</span>
      </button>

      {isCurrentTurn ? (
        <button
          onClick={onSubmit}
          className="px-8 py-3 bg-gradient-to-r from-blue-500 to-indigo-500 hover:from-blue-600 hover:to-indigo-600 rounded-xl font-semibold text-white transition-all shadow-lg shadow-blue-500/20"
        >
          Submit Choice
        </button>
      ) : (
        <span className="text-slate-400 text-sm">
          Turn {currentIndex + 1} of {totalTurns}
        </span>
      )}

      <button
        onClick={onNext}
        disabled={currentIndex >= totalTurns - 1}
        className="glass-button px-6 py-3 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        <span>Next</span>
        <span>→</span>
      </button>
    </div>
  );
};
```

**3.4 Pages**

`src/pages/HomePage.tsx`:
```typescript
import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { backend, SaveGame } from '../services/backend';
import { useGame } from '../contexts/GameContext';

export const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { setSessionId, addTurn } = useGame();
  const [saves, setSaves] = useState<SaveGame[]>([]);

  useEffect(() => {
    backend.listSaves().then(setSaves);
  }, []);

  const handleNewGame = async () => {
    try {
      const sessionId = await backend.startNewGame();
      setSessionId(sessionId);
      const initialTurn = await backend.getTurn(sessionId, 0);
      addTurn(initialTurn);
      navigate('/game');
    } catch (error) {
      console.error('Failed to start new game:', error);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-8">
      <div className="max-w-2xl w-full">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-6xl font-heading font-bold mb-4 bg-gradient-to-r from-blue-400 to-indigo-400 bg-clip-text text-transparent">
            Luup
          </h1>
          <p className="text-slate-300 text-lg">Your story begins here</p>
        </div>

        {/* New Game Button */}
        <button
          onClick={handleNewGame}
          className="w-full glass-card p-8 mb-6 hover:bg-glass-hover transition-all group"
        >
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-heading font-semibold mb-2 text-slate-50">
                Start New Adventure
              </h2>
              <p className="text-slate-400">Begin a fresh journey</p>
            </div>
            <span className="text-4xl group-hover:translate-x-2 transition-transform">
              ✨
            </span>
          </div>
        </button>

        {/* Saved Games */}
        <div className="glass-card p-6 mb-6">
          <h3 className="text-xl font-heading font-semibold mb-4 text-slate-50">
            Continue Your Journey
          </h3>
          <div className="space-y-3">
            {saves.map((save) => (
              <button
                key={save.id}
                onClick={() => {
                  // Stub: Just navigate to game page
                  navigate('/game');
                }}
                className="w-full glass-button p-4 text-left hover:bg-glass-hover transition-all"
              >
                <div className="flex justify-between items-start">
                  <div>
                    <h4 className="font-semibold text-slate-50">{save.name}</h4>
                    <p className="text-sm text-slate-400">
                      {save.turn_count} turns • Last played {save.last_played}
                    </p>
                  </div>
                  <span className="text-slate-400">→</span>
                </div>
              </button>
            ))}
          </div>
        </div>

        {/* Settings Button */}
        <button
          onClick={() => navigate('/settings')}
          className="w-full glass-button p-4 flex items-center justify-center gap-2 text-slate-300 hover:text-slate-50"
        >
          <span>⚙️</span>
          <span>Settings</span>
        </button>
      </div>
    </div>
  );
};
```

`src/pages/SettingsPage.tsx`:
```typescript
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useSettings } from '../contexts/SettingsContext';

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const { ollamaIp, setOllamaIp, saveSettings } = useSettings();
  const [localIp, setLocalIp] = useState(ollamaIp);
  const [saved, setSaved] = useState(false);

  const handleSave = async () => {
    setOllamaIp(localIp);
    await saveSettings();
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-8">
      <div className="max-w-xl w-full">
        <button
          onClick={() => navigate(-1)}
          className="glass-button px-4 py-2 mb-8 flex items-center gap-2 text-slate-300 hover:text-slate-50"
        >
          <span>←</span>
          <span>Back</span>
        </button>

        <div className="glass-card p-8">
          <h1 className="text-4xl font-heading font-bold mb-2 text-slate-50">
            Settings
          </h1>
          <p className="text-slate-400 mb-8">Configure your AI backend</p>

          {/* Ollama IP Config */}
          <div className="mb-6">
            <label className="block text-sm font-medium text-slate-300 mb-3">
              Ollama Server Address
            </label>
            <input
              type="text"
              value={localIp}
              onChange={(e) => setLocalIp(e.target.value)}
              placeholder="localhost:11434"
              className="w-full bg-glass-base backdrop-blur-glass rounded-xl border border-white/10 p-4 text-slate-50 placeholder-slate-500 focus:outline-none focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20"
            />
            <p className="text-xs text-slate-400 mt-2">
              Enter the IP address and port of your local Ollama server
            </p>
          </div>

          {/* Save Button */}
          <button
            onClick={handleSave}
            className="w-full px-6 py-3 bg-gradient-to-r from-blue-500 to-indigo-500 hover:from-blue-600 hover:to-indigo-600 rounded-xl font-semibold text-white transition-all shadow-lg shadow-blue-500/20"
          >
            {saved ? '✓ Saved!' : 'Save Settings'}
          </button>
        </div>
      </div>
    </div>
  );
};
```

`src/pages/GamePage.tsx`:
```typescript
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useGame } from '../contexts/GameContext';
import { backend } from '../services/backend';
import { TurnDisplay } from '../components/TurnDisplay';
import { ChoiceButton } from '../components/ChoiceButton';
import { ActionInput } from '../components/ActionInput';
import { TurnNavigation } from '../components/TurnNavigation';

export const GamePage: React.FC = () => {
  const navigate = useNavigate();
  const { sessionId, currentTurn, turnHistory, currentTurnIndex, addTurn, navigateToTurn } =
    useGame();
  const [customAction, setCustomAction] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const isCurrentTurn = currentTurnIndex === turnHistory.length - 1;

  const handleChoiceClick = (choice: string) => {
    setCustomAction(choice);
  };

  const handleSubmit = async () => {
    if (!sessionId || !customAction.trim() || !isCurrentTurn) return;

    setIsSubmitting(true);
    try {
      const newTurn = await backend.submitAction(sessionId, customAction);
      addTurn(newTurn);
      setCustomAction('');
    } catch (error) {
      console.error('Failed to submit action:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!currentTurn) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <p className="text-slate-400">No game in progress. Start a new game!</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <button
            onClick={() => navigate('/')}
            className="glass-button px-4 py-2 flex items-center gap-2 text-slate-300 hover:text-slate-50"
          >
            <span>←</span>
            <span>Home</span>
          </button>
          <h1 className="text-2xl font-heading font-semibold text-slate-50">
            Your Adventure
          </h1>
          <div className="w-24"></div> {/* Spacer for centering */}
        </div>

        {/* Story Display */}
        <TurnDisplay storyText={currentTurn.story_text} gameState={currentTurn.game_state} />

        {/* Choices */}
        <div className="mb-6">
          <h3 className="text-lg font-heading font-semibold text-slate-50 mb-4">
            Choose Your Path
          </h3>
          <div className="grid gap-3">
            {currentTurn.choices.map((choice, index) => (
              <ChoiceButton
                key={index}
                choice={choice}
                onClick={() => handleChoiceClick(choice)}
              />
            ))}
          </div>
        </div>

        {/* Custom Action Input */}
        <div className="mb-6">
          <ActionInput
            value={customAction}
            onChange={setCustomAction}
            onSubmit={handleSubmit}
            disabled={!isCurrentTurn || isSubmitting}
          />
        </div>

        {/* Turn Navigation */}
        <TurnNavigation
          currentIndex={currentTurnIndex}
          totalTurns={turnHistory.length}
          onPrevious={() => navigateToTurn(currentTurnIndex - 1)}
          onNext={() => navigateToTurn(currentTurnIndex + 1)}
          onSubmit={handleSubmit}
          isCurrentTurn={isCurrentTurn}
        />
      </div>
    </div>
  );
};
```

**3.5 App Router**

`src/App.tsx`:
```typescript
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { GameProvider } from './contexts/GameContext';
import { SettingsProvider } from './contexts/SettingsContext';
import { HomePage } from './pages/HomePage';
import { SettingsPage } from './pages/SettingsPage';
import { GamePage } from './pages/GamePage';

function App() {
  return (
    <Router>
      <SettingsProvider>
        <GameProvider>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="/game" element={<GamePage />} />
          </Routes>
        </GameProvider>
      </SettingsProvider>
    </Router>
  );
}

export default App;
```

## Phase 4: Future Extensibility Notes

**Document for Backend Integration**

Create `BACKEND_INTEGRATION.md`:
```markdown
# Backend Integration Guide

## Current Stub Implementation
The current implementation uses hardcoded responses in Rust commands.

## Future: Streaming + Tool Calls

### 1. Streaming Text
Replace `submit_action` with a streaming approach:
- Use Tauri events to emit text chunks
- Frontend listens via `listen()` API
- Append chunks to story text in real-time

### 2. Tool Call Protocol
Extend `BackendMessage` to support tool calls:

\`\`\`rust
enum BackendMessage {
    TextChunk { content: String },
    ToolCall { name: String, args: serde_json::Value },
    TurnComplete,
}
\`\`\`

Tool examples:
- `set_time(time: String)` → Updates game_state.time
- `set_location(location: String)` → Updates game_state.location
- `set_outfit(outfit: String)` → Updates game_state.outfit
- `add_inventory_item(item: String)` → Future: inventory system

### 3. Ollama Integration
Replace stubs with:
- HTTP client to Ollama API
- System prompt for game master behavior
- JSON mode for structured outputs (choices + tool calls)

### 4. UI Updates
- Add loading spinner during streaming
- Animate text chunks appearing
- Visual feedback for tool calls (e.g., location badge changes)
```

# Verification & Demo Script

## Build and Run

```bash
# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

## Testing Checklist

**Home Page**:
- [ ] "Start New Adventure" button navigates to game page
- [ ] Mock save games render correctly
- [ ] "Continue Your Journey" buttons navigate to game (stubbed)
- [ ] Settings button navigates to settings page
- [ ] Glassmorphism styling renders correctly

**Settings Page**:
- [ ] Ollama IP input field displays current value
- [ ] Can edit IP address
- [ ] Save button updates config and shows confirmation
- [ ] Back button returns to previous page

**Game Page**:
- [ ] Initial story text and choices render
- [ ] Game state badges (location, time, outfit) display correctly
- [ ] Clicking a choice populates the custom action textbox
- [ ] Can manually edit or type custom action
- [ ] Submit button sends action and displays new turn
- [ ] Previous/Next buttons navigate through turn history
- [ ] Submit button only appears on the current turn
- [ ] Home button returns to home page

**Extensibility**:
- [ ] Review `BACKEND_INTEGRATION.md` for future streaming approach
- [ ] Verify Rust types support additional game state fields
- [ ] Confirm frontend can handle dynamic game state updates

## Demo Flow

1. Launch app → Home page appears
2. Click "Start New Adventure" → Game page with initial story
3. Click one of the three choices → Action input populates
4. Optionally edit the action text
5. Click "Submit Choice" → New turn appears
6. Navigate back with "Previous" → See previous turn (read-only)
7. Navigate forward with "Next" → Return to current turn
8. Click Home → Return to home page
9. Click Settings → Open settings page
10. Change Ollama IP → Save → Confirmation appears

# Deploy

**Desktop Distribution**:
- **macOS**: `.dmg` in `src-tauri/target/release/bundle/dmg/`
- **Windows**: `.msi` in `src-tauri/target/release/bundle/msi/`
- **Linux**: `.AppImage` or `.deb` in `src-tauri/target/release/bundle/`

**Code Signing** (future):
- macOS: Apple Developer certificate
- Windows: Code signing certificate
- Document in `DISTRIBUTION.md` when ready to ship

**Auto-Updates** (future):
- Integrate Tauri updater plugin
- Host update manifests on static server
- Version bump workflow in CI/CD
