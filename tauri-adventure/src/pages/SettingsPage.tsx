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
