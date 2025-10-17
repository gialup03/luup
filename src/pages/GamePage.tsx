import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useGame } from '../contexts/GameContext';
import { backend } from '../services/backend';
import { TurnDisplay } from '../components/TurnDisplay';
import { ChoiceButton } from '../components/ChoiceButton';
import { ActionInput } from '../components/ActionInput';
import { TurnNavigation } from '../components/TurnNavigation';

export const GamePage: React.FC = () => {
  const navigate = useNavigate();
  const { sessionId, currentTurn, turnHistory, currentTurnIndex, addTurn, navigateToTurn } =
    useGame();
  const [customAction, setCustomAction] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const isCurrentTurn = currentTurnIndex === turnHistory.length - 1;

  const handleChoiceClick = (choice: string) => {
    setCustomAction(choice);
  };

  const handleSubmit = async () => {
    if (!sessionId || !customAction.trim() || !isCurrentTurn) return;

    setIsSubmitting(true);
    try {
      const newTurn = await backend.submitAction(sessionId, customAction);
      addTurn(newTurn);
      setCustomAction('');
    } catch (error) {
      console.error('Failed to submit action:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!currentTurn) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <p className="text-slate-400 mb-6">No game in progress</p>
          <button
            onClick={() => navigate('/')}
            className="glass-button px-6 py-3 text-slate-300 hover:text-slate-50"
          >
            Return to Home
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <button
            onClick={() => navigate('/')}
            className="glass-button px-4 py-2 flex items-center gap-2 text-slate-300 hover:text-slate-50"
          >
            <span>←</span>
            <span>Home</span>
          </button>
          <h1 className="text-2xl font-heading font-semibold text-slate-50">
            Your Adventure
          </h1>
          <div className="w-24"></div> {/* Spacer for centering */}
        </div>

        {/* Story Display */}
        <TurnDisplay storyText={currentTurn.story_text} gameState={currentTurn.game_state} />

        {/* Choices */}
        {isCurrentTurn && (
          <div className="mb-6">
            <h3 className="text-lg font-heading font-semibold text-slate-50 mb-4">
              Choose Your Path
            </h3>
            <div className="grid gap-3">
              {currentTurn.choices.map((choice, index) => (
                <ChoiceButton
                  key={index}
                  choice={choice}
                  onClick={() => handleChoiceClick(choice)}
                />
              ))}
            </div>
          </div>
        )}

        {/* Custom Action Input */}
        {isCurrentTurn && (
          <div className="mb-6">
            <ActionInput
              value={customAction}
              onChange={setCustomAction}
              onSubmit={handleSubmit}
              disabled={isSubmitting}
            />
          </div>
        )}

        {/* Turn Navigation */}
        <TurnNavigation
          currentIndex={currentTurnIndex}
          totalTurns={turnHistory.length}
          onPrevious={() => navigateToTurn(currentTurnIndex - 1)}
          onNext={() => navigateToTurn(currentTurnIndex + 1)}
          onSubmit={handleSubmit}
          isCurrentTurn={isCurrentTurn}
        />
      </div>
    </div>
  );
};
