import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { backend, SaveGame } from '../services/backend';
import { useGame } from '../contexts/GameContext';

export const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { setSessionId, addTurn } = useGame();
  const [saves, setSaves] = useState<SaveGame[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    backend.listSaves().then(setSaves);
  }, []);

  const handleNewGame = async () => {
    setLoading(true);
    try {
      const sessionId = await backend.startNewGame();
      setSessionId(sessionId);
      const initialTurn = await backend.getTurn(sessionId, 0);
      addTurn(initialTurn);
      navigate('/game');
    } catch (error) {
      console.error('Failed to start new game:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-8">
      <div className="max-w-2xl w-full">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-6xl font-heading font-bold mb-4 bg-gradient-to-r from-blue-400 to-indigo-400 bg-clip-text text-transparent">
            Adventure Awaits
          </h1>
          <p className="text-slate-300 text-lg">Your story begins here</p>
        </div>

        {/* New Game Button */}
        <button
          onClick={handleNewGame}
          disabled={loading}
          className="w-full glass-card p-8 mb-6 hover:bg-glass-hover transition-all group disabled:opacity-50"
        >
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-heading font-semibold mb-2 text-slate-50">
                {loading ? 'Starting...' : 'Start New Adventure'}
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
