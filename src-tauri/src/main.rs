// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ollama;
mod agent;

use agent::{Agent, AgentMessage, GameState as AgentGameState};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{State, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TurnData {
    turn_number: u32,
    story_text: String,
    choices: Vec<String>,
    game_state: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    time: String,
    location: String,
    outfit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaConfig {
    ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveGame {
    id: String,
    name: String,
    last_played: String,
    turn_count: u32,
}

struct AppState {
    ollama_config: Mutex<OllamaConfig>,
    game_history: Mutex<Vec<TurnData>>,
    agent: Mutex<Agent>,
    current_game_state: Mutex<AgentGameState>,
}

#[tauri::command]
fn start_new_game(state: State<AppState>) -> Result<String, String> {
    let mut history = state.game_history.lock().unwrap();
    let mut agent = state.agent.lock().unwrap();
    let mut current_state = state.current_game_state.lock().unwrap();
    
    history.clear();
    
    // Initialize agent and get initial state
    *current_state = agent.start_new_game();
    
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
            time: current_state.time.clone(),
            location: current_state.location.clone(),
            outfit: current_state.outfit.clone(),
        },
    });
    
    Ok("session_stub_001".to_string())
}

#[tauri::command]
fn get_turn(_session_id: String, turn_number: u32, state: State<AppState>) -> Result<TurnData, String> {
    let history = state.game_history.lock().unwrap();
    
    if let Some(turn) = history.get(turn_number as usize) {
        Ok(turn.clone())
    } else {
        Err("Turn not found".to_string())
    }
}

#[tauri::command]
fn submit_action(
    _session_id: String,
    action: String,
    state: State<AppState>,
) -> Result<TurnData, String> {
    let mut history = state.game_history.lock().unwrap();
    let current_turn = history.len() as u32;
    
    // Legacy sync endpoint - just return a stub
    // Real streaming happens via submit_action_stream
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
async fn submit_action_stream(
    window: tauri::Window,
    _session_id: String,
    action: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Clone what we need from state
    let turn_number = {
        let history = state.game_history.lock().unwrap();
        history.len() as u32
    };

    // Clone agent and state to avoid holding locks across await
    let mut agent = {
        let agent_guard = state.agent.lock().map_err(|e| e.to_string())?;
        agent_guard.clone()
    };
    
    let mut current_state = {
        let state_guard = state.current_game_state.lock().map_err(|e| e.to_string())?;
        state_guard.clone()
    };

    // Process the action with streaming - no locks held here
    let result = agent.process_action(
        action,
        &mut current_state,
        turn_number,
        |message| {
            // Emit each message to the frontend
            let _ = window.emit("agent-stream", &message);
            
            // If it's a turn complete, also save to history
            if let AgentMessage::TurnComplete { turn_number, story_text, choices, game_state } = &message {
                if let Ok(mut history) = state.game_history.lock() {
                    history.push(TurnData {
                        turn_number: *turn_number,
                        story_text: story_text.clone(),
                        choices: choices.clone(),
                        game_state: GameState {
                            time: game_state.time.clone(),
                            location: game_state.location.clone(),
                            outfit: game_state.outfit.clone(),
                        },
                    });
                }
            }
        }
    ).await;

    // Update the state back after processing
    if result.is_ok() {
        if let Ok(mut agent_guard) = state.agent.lock() {
            *agent_guard = agent;
        }
        if let Ok(mut state_guard) = state.current_game_state.lock() {
            *state_guard = current_state;
        }
    }

    result.map_err(|e| e.to_string())
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
                ip_address: "192.168.0.100:11434".to_string(),
            }),
            game_history: Mutex::new(Vec::new()),
            agent: Mutex::new(Agent::new()),
            current_game_state: Mutex::new(AgentGameState {
                time: "Morning".to_string(),
                location: "Mysterious Room".to_string(),
                outfit: "Traveler's Cloak".to_string(),
            }),
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_new_game,
            get_turn,
            submit_action,
            submit_action_stream,
            list_saves,
            get_ollama_config,
            set_ollama_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
