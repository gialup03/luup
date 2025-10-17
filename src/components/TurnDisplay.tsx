import React from 'react';
import type { GameState } from '../services/backend';

interface TurnDisplayProps {
  storyText: string;
  gameState: GameState;
}

export const TurnDisplay: React.FC<TurnDisplayProps> = ({ storyText, gameState }) => {
  return (
    <div className="glass-card p-8 mb-6">
      {/* Game State Badge */}
      <div className="flex gap-3 mb-6 flex-wrap">
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
