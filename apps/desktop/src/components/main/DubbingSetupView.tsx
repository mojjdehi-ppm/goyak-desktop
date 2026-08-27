import React from 'react';
import { useTranslation } from 'react-i18next';
import { Info, Key, Mic } from 'lucide-react';
import { AppSelector } from './AppSelector';
import { LanguageSelector } from './LanguageSelector';
import { AudioApplicationInfo, AudioDeviceInfo } from '../../types/audio';
import { ChevronDown } from 'lucide-react';

const SYSTEM_AUDIO_ID = 'system';

/** Shared label treatment: quiet, uppercase, consistent rhythm. */
export const FieldLabel: React.FC<{ htmlFor?: string; children: React.ReactNode }> = ({
  htmlFor,
  children,
}) => (
  <label
    htmlFor={htmlFor}
    className="text-[11px] uppercase tracking-wider font-semibold text-base-content/45 block text-start"
  >
    {children}
  </label>
);

interface DubbingSetupViewProps {
  applications: AudioApplicationInfo[];
  selectedAppId: string;
  onSelectApp: (appId: string) => void;
  onRefreshApps: () => void;
  sourceLanguage: string;
  targetLanguage: string;
  onSourceLanguageChange: (lang: string) => void;
  onTargetLanguageChange: (lang: string) => void;
  outputDevices: AudioDeviceInfo[];
  selectedOutputId: string;
  onSelectOutputId: (id: string) => void;
  isApiKeyMissing: boolean;
  onOpenSettings?: () => void;
  onStartDubbing: () => void;
  isDubbingActive: boolean;
}

export const DubbingSetupView: React.FC<DubbingSetupViewProps> = ({
  applications,
  selectedAppId,
  onSelectApp,
  onRefreshApps,
  sourceLanguage,
  targetLanguage,
  onSourceLanguageChange,
  onTargetLanguageChange,
  outputDevices,
  selectedOutputId,
  onSelectOutputId,
  isApiKeyMissing,
  onOpenSettings,
  onStartDubbing,
  isDubbingActive,
}) => {
  const { t } = useTranslation();

  return (
    <div className="w-full max-w-lg mx-auto space-y-6 select-none animate-in fade-in duration-300">
      <div className="space-y-5 relative">
        <AppSelector
          applications={applications}
          selectedAppId={selectedAppId}
          onSelectApp={onSelectApp}
          onRefresh={onRefreshApps}
          disabled={isDubbingActive}
        />

        <LanguageSelector
          sourceLanguage={sourceLanguage}
          targetLanguage={targetLanguage}
          onSourceChange={onSourceLanguageChange}
          onTargetChange={onTargetLanguageChange}
          disabled={isDubbingActive}
        />

        <div className="space-y-2 relative z-10">
          <FieldLabel htmlFor="output-device">{t('home.outputDevice')}</FieldLabel>
          <div className="relative">
            <select
              id="output-device"
              value={selectedOutputId}
              onChange={(e) => onSelectOutputId(e.target.value)}
              disabled={isDubbingActive}
              className="w-full h-12 bg-base-200 border border-base-content/[0.07] hover:border-base-content/[0.14] text-base-content text-sm rounded-xl px-4 pr-10 rtl:pr-4 rtl:pl-10 focus:outline-none focus:border-primary transition-colors disabled:opacity-50 appearance-none cursor-pointer"
            >
              {outputDevices.map((dev) => (
                <option key={dev.id} value={dev.id} className="bg-base-200 text-base-content py-2">
                  {dev.name} {dev.isDefault ? `(${t('home.defaultDevice')})` : ''}
                </option>
              ))}
            </select>
            {/* appearance-none strips the native arrow; without one the select
                reads as a plain text box. */}
            <ChevronDown className="w-4 h-4 text-base-content/50 absolute right-3.5 rtl:right-auto rtl:left-3.5 top-1/2 -translate-y-1/2 pointer-events-none" />
          </div>
        </div>

        {/* Error prevention: with System Audio + the same output device, the
            echo guard pauses listening while the dub plays. Say so up front
            instead of letting users discover it as "missing dialogue". */}
        {selectedAppId === SYSTEM_AUDIO_ID && (
          <div className="flex items-start gap-2.5 text-xs leading-relaxed text-base-content/50 text-start px-1">
            <Info className="w-3.5 h-3.5 shrink-0 mt-0.5 text-info" />
            <span>{t('home.systemAudioHint')}</span>
          </div>
        )}
      </div>

      {isApiKeyMissing && (
        <div className="flex items-center justify-between gap-3 text-xs text-warning bg-warning/10 border border-warning/20 rounded-xl px-4 py-3">
          <div className="flex items-center gap-2.5 rtl:space-x-reverse min-w-0">
            <Key className="w-4 h-4 shrink-0" />
            <span className="text-start" dangerouslySetInnerHTML={{ __html: t('home.missingApiKey') }} />
          </div>
          {onOpenSettings && (
            <button
              onClick={onOpenSettings}
              className="text-xs px-3 py-1.5 font-semibold rounded-lg shrink-0 bg-warning/15 hover:bg-warning/25 transition-colors cursor-pointer"
            >
              {t('home.openSettings')}
            </button>
          )}
        </div>
      )}

      <button
        onClick={onStartDubbing}
        className="w-full h-14 rounded-2xl bg-primary hover:bg-primary/90 text-primary-content font-semibold text-base flex items-center justify-center gap-2.5 rtl:space-x-reverse shadow-lg shadow-primary/20 transition-all active:scale-[0.99] cursor-pointer"
      >
        <Mic className="w-5 h-5" />
        <span>{t('home.startDubbing')}</span>
      </button>
    </div>
  );
};
