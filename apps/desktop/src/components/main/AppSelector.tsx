import React, { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { RefreshCw, ChevronDown, Check } from 'lucide-react';
import { AudioApplicationInfo } from '../../types/audio';
import { FieldLabel } from './DubbingSetupView';

interface AppSelectorProps {
  applications: AudioApplicationInfo[];
  selectedAppId: string;
  onSelectApp: (appId: string) => void;
  onRefresh: () => void;
  disabled?: boolean;
}

export const AppSelector: React.FC<AppSelectorProps> = ({
  applications,
  selectedAppId,
  onSelectApp,
  onRefresh,
  disabled = false,
}) => {
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Deliberately no `|| applications[0]` fallback: showing a source that is not
  // the one the backend will be told to capture is how a stale selection went
  // unnoticed.
  const selectedApp = applications.find((a) => a.id === selectedAppId);

  return (
    <div className="space-y-2 relative z-30">
      <div className="flex items-center justify-between">
        <FieldLabel>{t('home.selectApp')}</FieldLabel>
        <button
          type="button"
          onClick={onRefresh}
          className="text-[11px] text-base-content/40 hover:text-primary flex items-center gap-1.5 transition-colors font-medium cursor-pointer"
        >
          <span>{t('home.refresh')}</span>
          <RefreshCw className="w-3 h-3" />
        </button>
      </div>

      <div className="relative" ref={containerRef}>
        <button
          type="button"
          disabled={disabled}
          onClick={() => setIsOpen(!isOpen)}
          aria-expanded={isOpen}
          className="w-full h-12 bg-base-200 border border-base-content/[0.07] hover:border-base-content/[0.14] text-base-content text-sm rounded-xl px-4 flex items-center justify-between focus:outline-none focus:border-primary transition-colors disabled:opacity-50 cursor-pointer min-w-0"
        >
          <span className="truncate text-start font-medium block min-w-0 flex-1 pr-2 rtl:pr-0 rtl:pl-2">
            {selectedApp ? selectedApp.name : t('home.selectApp')}
          </span>
          <ChevronDown
            className={`w-4 h-4 text-base-content/50 shrink-0 transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`}
          />
        </button>

        {isOpen && !disabled && (
          <div className="absolute z-50 w-full mt-2 bg-elevated border border-base-content/[0.09] rounded-xl shadow-2xl shadow-black/60 overflow-hidden py-1 max-h-60 overflow-y-auto">
            {applications.map((app) => {
              const isSelected = selectedAppId === app.id;
              return (
                <button
                  key={app.id}
                  type="button"
                  onClick={() => {
                    onSelectApp(app.id);
                    setIsOpen(false);
                  }}
                  className={`w-full flex items-center justify-between gap-2 px-4 py-2.5 text-sm transition-colors cursor-pointer text-start ${
                    isSelected
                      ? 'text-primary bg-primary/10 font-medium'
                      : 'text-base-content/80 hover:bg-base-content/5 hover:text-base-content'
                  }`}
                >
                  <span className="truncate block min-w-0 flex-1">{app.name}</span>
                  {isSelected && <Check className="w-4 h-4 shrink-0" />}
                </button>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};
