// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

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
        .plugin(tauri_plugin_shell::init())
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
