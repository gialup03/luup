import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { listen } from '@tauri-apps/api/event';
import { useGame } from '../contexts/GameContext';
import { backend, type AgentStreamMessage } from '../services/backend';
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

  // Streaming state
  const [isStreaming, setIsStreaming] = useState(false);
  const [streamedText, setStreamedText] = useState('');
  const [streamedReasoning, setStreamedReasoning] = useState('');
  const [showReasoning, setShowReasoning] = useState(false);
  const [toolCallsInProgress, setToolCallsInProgress] = useState<Array<{ name: string; args: any }>>([]);
  const [streamError, setStreamError] = useState<string | null>(null);

  const isCurrentTurn = currentTurnIndex === turnHistory.length - 1;

  // Set up streaming event listener
  useEffect(() => {
    const setupListener = async () => {
      const unlisten = await listen<AgentStreamMessage>('agent-stream', (event) => {
        const message = event.payload;

        switch (message.type) {
          case 'text_chunk':
            setStreamedText((prev) => prev + message.content);
            break;

          case 'reasoning_chunk':
            setStreamedReasoning((prev) => prev + message.content);
            break;

          case 'tool_call':
            setToolCallsInProgress((prev) => [...prev, { name: message.name, args: message.args }]);
            break;

          case 'tool_result':
            // Tool executed successfully - remove from in-progress list
            setToolCallsInProgress((prev) => prev.filter((tc) => tc.name !== message.name));
            break;

          case 'turn_complete':
            // Turn is complete - add to history
            addTurn({
              turn_number: message.turn_number,
              story_text: message.story_text,
              choices: message.choices,
              game_state: message.game_state,
            });
            setIsStreaming(false);
            setIsSubmitting(false);
            setStreamedText('');
            setStreamedReasoning('');
            setToolCallsInProgress([]);
            setCustomAction('');
            break;

          case 'error':
            setStreamError(message.message);
            setIsStreaming(false);
            setIsSubmitting(false);
            break;
        }
      });

      return unlisten;
    };

    let unlistenFn: (() => void) | null = null;
    setupListener().then((fn) => {
      unlistenFn = fn;
    });

    return () => {
      if (unlistenFn) unlistenFn();
    };
  }, [addTurn]);

  const handleChoiceClick = (choice: string) => {
    setCustomAction(choice);
  };

  const handleSubmit = async () => {
    if (!sessionId || !customAction.trim() || !isCurrentTurn || isStreaming) return;

    setIsSubmitting(true);
    setIsStreaming(true);
    setStreamedText('');
    setStreamedReasoning('');
    setStreamError(null);

    try {
      await backend.submitActionStream(sessionId, customAction);
    } catch (error) {
      console.error('Failed to submit action:', error);
      setStreamError('Failed to connect to the storyteller. Please check your Ollama connection.');
      setIsStreaming(false);
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

        {/* Streaming Content */}
        {isStreaming && (
          <div className="mb-6 space-y-4">
            {/* Streaming Indicator */}
            <div className="glass-card p-4 border border-blue-500/30">
              <div className="flex items-center gap-3 mb-4">
                <div className="animate-pulse w-2 h-2 bg-blue-500 rounded-full"></div>
                <span className="text-slate-300 text-sm">The tale unfolds...</span>
              </div>

              {/* Streamed Text */}
              {streamedText && (
                <div className="prose prose-invert max-w-none">
                  <p className="text-slate-200 leading-relaxed whitespace-pre-wrap">{streamedText}</p>
                </div>
              )}

              {/* Tool Calls in Progress */}
              {toolCallsInProgress.length > 0 && (
                <div className="mt-4 space-y-2">
                  {toolCallsInProgress.map((tool, idx) => (
                    <div key={idx} className="flex items-center gap-2 text-sm text-blue-400">
                      <div className="animate-spin w-3 h-3 border-2 border-blue-400 border-t-transparent rounded-full"></div>
                      <span>Updating {tool.name.replace('set_', '')}...</span>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Reasoning Display */}
            {streamedReasoning && (
              <div className="glass-card p-4 border border-purple-500/30">
                <button
                  onClick={() => setShowReasoning(!showReasoning)}
                  className="flex items-center gap-2 text-sm text-purple-300 hover:text-purple-200 w-full"
                >
                  <span className={`transform transition-transform ${showReasoning ? 'rotate-90' : ''}`}>
                    ▶
                  </span>
                  <span>Model Reasoning</span>
                </button>
                {showReasoning && (
                  <div className="mt-3 text-sm text-slate-400 whitespace-pre-wrap font-mono">
                    {streamedReasoning}
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {/* Stream Error */}
        {streamError && (
          <div className="mb-6 glass-card p-4 border border-red-500/30">
            <div className="flex items-center gap-2 text-red-400">
              <span className="text-lg">⚠️</span>
              <span className="text-sm">{streamError}</span>
            </div>
          </div>
        )}

        {/* Choices */}
        {isCurrentTurn && !isStreaming && (
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
        {isCurrentTurn && !isStreaming && (
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
