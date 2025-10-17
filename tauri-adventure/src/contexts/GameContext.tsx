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
