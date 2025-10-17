import { invoke } from '@tauri-apps/api/core';

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

// Stream message types from the agent
export type AgentStreamMessage =
  | { type: 'text_chunk'; content: string }
  | { type: 'reasoning_chunk'; content: string }
  | { type: 'tool_call'; name: string; args: any }
  | { type: 'tool_result'; name: string; result: GameState }
  | { type: 'choices'; choices: string[] }
  | { type: 'turn_complete'; turn_number: number; story_text: string; choices: string[]; game_state: GameState }
  | { type: 'error'; message: string };

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

  async submitActionStream(sessionId: string, action: string): Promise<void> {
    return await invoke('submit_action_stream', { sessionId, action });
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
