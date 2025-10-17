import React from 'react';

interface TurnNavigationProps {
  currentIndex: number;
  totalTurns: number;
  onPrevious: () => void;
  onNext: () => void;
  onSubmit: () => void;
  isCurrentTurn: boolean;
}

export const TurnNavigation: React.FC<TurnNavigationProps> = ({
  currentIndex,
  totalTurns,
  onPrevious,
  onNext,
  onSubmit,
  isCurrentTurn,
}) => {
  return (
    <div className="flex justify-between items-center gap-4">
      <button
        onClick={onPrevious}
        disabled={currentIndex === 0}
        className="glass-button px-6 py-3 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        <span>←</span>
        <span>Previous</span>
      </button>

      {isCurrentTurn ? (
        <button
          onClick={onSubmit}
          className="px-8 py-3 bg-gradient-to-r from-blue-500 to-indigo-500 hover:from-blue-600 hover:to-indigo-600 rounded-xl font-semibold text-white transition-all shadow-lg shadow-blue-500/20"
        >
          Submit Choice
        </button>
      ) : (
        <span className="text-slate-400 text-sm">
          Turn {currentIndex + 1} of {totalTurns}
        </span>
      )}

      <button
        onClick={onNext}
        disabled={currentIndex >= totalTurns - 1}
        className="glass-button px-6 py-3 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        <span>Next</span>
        <span>→</span>
      </button>
    </div>
  );
};
