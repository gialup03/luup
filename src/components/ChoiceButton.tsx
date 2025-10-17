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
