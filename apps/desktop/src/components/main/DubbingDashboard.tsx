import React, { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { AlertCircle, Check, Copy } from 'lucide-react';
import { useAudioStore } from '../../stores/audioStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { SUPPORTED_AUDIO_LANGUAGES } from '@dubly/shared';
import { ActiveSessionView } from './ActiveSessionView';
import { DubbingSetupView } from './DubbingSetupView';

interface DubbingDashboardProps {
  onOpenSettings?: () => void;
}

export const DubbingDashboard: React.FC<DubbingDashboardProps> = ({ onOpenSettings }) => {
  const { t } = useTranslation();
  const { settings } = useSettingsStore();
  const {
    status,
    selectedAppId,
    setSelectedAppId,
    applications,
    outputDevices,
    selectedOutputId,
    setSelectedOutputId,
    dubbedVolume,
    setDubbedVolume,
    isDubbedMuted,
    toggleDubbedMute,
    sourceLanguage,
    setSourceLanguage,
    targetLanguage,
    setTargetLanguage,
    errorMessage,
    logs,
    statusLine,
    transcript,
    toggleDubbing,
    refreshDevicesAndApps,
  } = useAudioStore();

  const isDubbingActive = status === 'listening' || status === 'translating' || status === 'playing' || status === 'connecting';
  const isApiKeyMissing = !settings.geminiApiKey || settings.geminiApiKey.trim() === '';

  const [isCopied, setIsCopied] = useState(false);
  const logPanelRef = useRef<HTMLDivElement>(null);

  // Keep the newest log line in view - the newest line is the one that matters.
  useEffect(() => {
    const el = logPanelRef.current;
    if (el) el.scrollTop = el.scrollHeight;
  }, [logs]);

  const copyLogs = async () => {
    const text = logs.join('\n');
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // WebView clipboard API can be unavailable; fall back to a hidden textarea.
      const ta = document.createElement('textarea');
      ta.value = text;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      ta.remove();
    }
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  };

  const getLanguageName = (code: string) => {
    if (code === 'auto') return 'Auto';
    const lang = SUPPORTED_AUDIO_LANGUAGES.find((l) => l.code === code);
    return lang ? `${lang.flag} ${lang.nativeName}` : code;
  };

  const getAppName = (id: string) => {
    const app = applications.find(a => a.id === id);
    return app ? app.name : id;
  };

  return (
    <div className="w-full max-w-lg m-auto flex flex-col gap-3">
      {isDubbingActive ? (
        <ActiveSessionView
          status={status}
          appName={getAppName(selectedAppId)}
          sourceLanguageName={getLanguageName(sourceLanguage)}
          targetLanguageName={getLanguageName(targetLanguage)}
          dubbedVolume={dubbedVolume}
          isDubbedMuted={isDubbedMuted}
          onDubbedVolumeChange={setDubbedVolume}
          onToggleDubbedMute={toggleDubbedMute}
          onStopDubbing={toggleDubbing}
          errorMessage={errorMessage}
          statusLine={statusLine}
          transcript={transcript}
        />
      ) : (
        <>
          <DubbingSetupView
            applications={applications}
            selectedAppId={selectedAppId}
            onSelectApp={setSelectedAppId}
            onRefreshApps={refreshDevicesAndApps}
            sourceLanguage={sourceLanguage}
            targetLanguage={targetLanguage}
            onSourceLanguageChange={setSourceLanguage}
            onTargetLanguageChange={setTargetLanguage}
            outputDevices={outputDevices}
            selectedOutputId={selectedOutputId}
            onSelectOutputId={setSelectedOutputId}
            isApiKeyMissing={isApiKeyMissing}
            onOpenSettings={onOpenSettings}
            onStartDubbing={toggleDubbing}
            isDubbingActive={isDubbingActive}
          />
          {/* A session that dies used to bounce back here with no explanation
              visible anywhere - the error only existed in component state. */}
          {errorMessage && (
            <div
              role="alert"
              className="w-full flex items-start gap-2.5 text-xs text-error bg-error/10 border border-error/20 rounded-xl px-4 py-3 leading-relaxed text-start select-text"
            >
              <AlertCircle className="w-4 h-4 shrink-0 mt-0.5" />
              <span className="break-words min-w-0" dir="auto">{errorMessage}</span>
            </div>
          )}
        </>
      )}

      <details className="w-full">
        <summary className="text-[11px] text-base-content/40 hover:text-base-content/70 cursor-pointer select-none px-1 transition-colors">
          {t('home.debugLogs')}
        </summary>
        <div className="mt-2 relative">
          <button
            type="button"
            onClick={copyLogs}
            aria-label={t('home.copyLogs')}
            className="absolute top-2 right-2 z-10 flex items-center gap-1.5 text-[10px] px-2.5 py-1.5 rounded-lg bg-base-300 border border-base-content/[0.07] text-base-content/60 hover:text-base-content transition-colors cursor-pointer"
          >
            {isCopied ? <Check className="w-3 h-3 text-success" /> : <Copy className="w-3 h-3" />}
            <span>{isCopied ? t('home.copied') : t('home.copyLogs')}</span>
          </button>
          <div
            ref={logPanelRef}
            role="log"
            aria-live="polite"
            className="bg-base-200/70 border border-base-content/[0.07] rounded-xl p-3 pt-9 max-h-44 overflow-y-auto font-mono text-[10px] leading-relaxed text-base-content/70 whitespace-pre-wrap select-text text-left"
            dir="ltr"
          >
            {logs.join('\n')}
          </div>
        </div>
      </details>
    </div>
  );
};
