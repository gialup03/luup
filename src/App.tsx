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
