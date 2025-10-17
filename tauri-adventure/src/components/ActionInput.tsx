import React from 'react';

interface ActionInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  disabled?: boolean;
}

export const ActionInput: React.FC<ActionInputProps> = ({
  value,
  onChange,
  onSubmit,
  disabled,
}) => {
  return (
    <div className="glass-card p-6">
      <label className="block text-sm font-medium text-slate-300 mb-3">
        Your Action
      </label>
      <textarea
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
            onSubmit();
          }
        }}
        disabled={disabled}
        placeholder="Type your custom action here, or click a choice above..."
        className="w-full bg-glass-base backdrop-blur-glass rounded-xl border border-white/10 p-4 text-slate-50 placeholder-slate-500 focus:outline-none focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20 resize-none"
        rows={3}
      />
      <p className="text-xs text-slate-400 mt-2">Ctrl/⌘ + Enter to submit</p>
    </div>
  );
};
