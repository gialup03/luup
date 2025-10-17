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
