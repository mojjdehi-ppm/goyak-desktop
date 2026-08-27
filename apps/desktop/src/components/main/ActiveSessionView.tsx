import React, { useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { Square } from 'lucide-react';
import { AudioControls } from './AudioControls';
import { DubbingStatus } from '../../types/audio';
import { TranscriptEntry } from '../../stores/audioStore';

interface ActiveSessionViewProps {
  status: DubbingStatus;
  appName: string;
  sourceLanguageName: string;
  targetLanguageName: string;
  dubbedVolume: number;
  isDubbedMuted: boolean;
  onDubbedVolumeChange: (val: number) => void;
  onToggleDubbedMute: () => void;
  onStopDubbing: () => void;
  errorMessage: string | null;
  statusLine?: string | null;
  transcript?: TranscriptEntry[];
}

const WaveBar: React.FC<{ delay: string; height: string }> = ({ delay, height }) => (
  <span
    className="inline-block w-0.5 rounded-full bg-primary opacity-80"
    style={{
      height,
      animationName: 'dubWave',
      animationDuration: '1.1s',
      animationTimingFunction: 'ease-in-out',
      animationIterationCount: 'infinite',
      animationDirection: 'alternate',
      animationDelay: delay,
    }}
  />
);

export const ActiveSessionView: React.FC<ActiveSessionViewProps> = ({
  status,
  appName,
  sourceLanguageName,
  targetLanguageName,
  dubbedVolume,
  isDubbedMuted,
  onDubbedVolumeChange,
  onToggleDubbedMute,
  onStopDubbing,
  errorMessage,
  statusLine,
  transcript = [],
}) => {
  const { t } = useTranslation();
  const isConnecting = status === 'connecting';
  const hasTranscript = transcript.length > 0;

  // Chat-style auto-follow: stick to the newest entry, but stop following as
  // soon as the user scrolls up to read history.
  const chatRef = useRef<HTMLDivElement>(null);
  const followRef = useRef(true);
  useEffect(() => {
    const el = chatRef.current;
    if (el && followRef.current) el.scrollTop = el.scrollHeight;
  }, [transcript]);

  return (
    <div className="w-full mx-auto flex flex-col items-center gap-4 select-none px-1 py-1 animate-in fade-in duration-300">

      <div className="flex flex-col items-center gap-3 w-full">
        <div className="relative flex items-center justify-center w-20 h-20">
          <span
            className="absolute inset-0 rounded-full border border-primary/20"
            style={{
              animationName: 'dubPulseRing',
              animationDuration: '2.4s',
              animationTimingFunction: 'ease-out',
              animationIterationCount: 'infinite',
            }}
          />
          <span
            className="absolute inset-2 rounded-full border border-primary/15"
            style={{
              animationName: 'dubPulseRing',
              animationDuration: '2.4s',
              animationTimingFunction: 'ease-out',
              animationIterationCount: 'infinite',
              animationDelay: '0.6s',
            }}
          />
          <span className="relative z-10 w-12 h-12 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center">
            {isConnecting ? (
              <span
                className="w-5 h-5 rounded-full border-2 border-primary border-t-transparent"
                style={{
                  animationName: 'spin',
                  animationDuration: '0.8s',
                  animationTimingFunction: 'linear',
                  animationIterationCount: 'infinite',
                }}
              />
            ) : (
              <span className="flex items-end gap-[3px] h-5">
                {[
                  { delay: '0s', height: '6px' },
                  { delay: '0.15s', height: '14px' },
                  { delay: '0.3s', height: '10px' },
                  { delay: '0.45s', height: '18px' },
                  { delay: '0.1s', height: '8px' },
                  { delay: '0.25s', height: '14px' },
                  { delay: '0.35s', height: '6px' },
                ].map((b, i) => (
                  <WaveBar key={i} delay={b.delay} height={b.height} />
                ))}
              </span>
            )}
          </span>
        </div>

        <div className="text-center space-y-1">
          <div className="text-[11px] uppercase tracking-widest text-base-content/40 font-medium">
            {isConnecting ? t('common.connecting') : t('common.playing')}
          </div>
          <h2 className="text-xl font-bold text-base-content tracking-tight leading-snug px-2 line-clamp-2" dir="auto">
            {appName}
          </h2>
          <div className="flex items-center justify-center gap-2 text-xs text-base-content/50">
            <span>{sourceLanguageName}</span>
            <span className="text-primary">→</span>
            <span className="text-base-content/80 font-semibold">{targetLanguageName}</span>
          </div>
          {/* Latest pipeline event - the session is a black box without it. */}
          {statusLine && (
            <div
              className="text-[10px] font-mono text-base-content/40 max-w-full truncate px-2 pt-1"
              dir="ltr"
              aria-live="polite"
            >
              {statusLine}
            </div>
          )}
        </div>
      </div>

      {/* Empty state: streaming but nothing heard yet. Without this, a quiet
          source looks identical to a broken session. */}
      {!isConnecting && !hasTranscript && (
        <p className="text-[11px] text-base-content/40 text-center px-4">
          {t('home.waitingForSpeech')}
        </p>
      )}

      {/* Chat-style transcript: original speech on one side, its dub on the
          other, scrollable like a conversation. Live proof the dub is
          advancing through the dialogue. */}
      {hasTranscript && (
        <div
          ref={chatRef}
          onScroll={(e) => {
            const el = e.currentTarget;
            followRef.current = el.scrollHeight - el.scrollTop - el.clientHeight < 60;
          }}
          className="w-full h-[30vh] min-h-[140px] overflow-y-auto bg-base-200/40 border border-base-content/[0.07] rounded-2xl p-3 space-y-2 select-text"
        >
          {transcript.map((entry) => (
            <div key={entry.id} className="space-y-1.5">
              {entry.heard && (
                <div className="flex justify-start">
                  <div
                    className="max-w-[85%] rounded-2xl rounded-bl-md bg-base-300/70 border border-base-content/[0.05] px-3.5 py-2 text-xs leading-relaxed text-base-content/55"
                    dir="auto"
                  >
                    {entry.heard}
                  </div>
                </div>
              )}
              {entry.dubbed && (
                <div className="flex justify-end">
                  <div
                    className="max-w-[85%] rounded-2xl rounded-br-md bg-primary/15 border border-primary/20 px-3.5 py-2 text-sm leading-relaxed text-base-content/95"
                    dir="auto"
                  >
                    {entry.dubbed}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      <div className="w-full bg-base-200/70 border border-base-content/[0.07] rounded-2xl p-3.5">
        <AudioControls
          dubbedVolume={dubbedVolume}
          isDubbedMuted={isDubbedMuted}
          onDubbedVolumeChange={onDubbedVolumeChange}
          onToggleDubbedMute={onToggleDubbedMute}
        />
      </div>

      {errorMessage && (
        <div className="w-full text-xs text-error bg-error/10 border border-error/25 rounded-xl px-4 py-3 text-center leading-relaxed">
          {errorMessage}
        </div>
      )}

      <button
        onClick={onStopDubbing}
        className="w-full h-12 rounded-xl font-semibold text-sm flex items-center justify-center gap-2.5 cursor-pointer active:scale-[0.99] border border-base-content/[0.07] text-base-content/50 bg-transparent hover:bg-error/10 hover:border-error/40 hover:text-error"
        style={{ transition: 'background-color 220ms ease, border-color 220ms ease, color 220ms ease, transform 120ms ease' }}
      >
        <Square className="w-3.5 h-3.5 fill-current" />
        <span>{t('home.stopDubbing')}</span>
      </button>

      <style>{`
        @keyframes dubWave {
          from { transform: scaleY(0.35); opacity: 0.5; }
          to   { transform: scaleY(1);    opacity: 1;   }
        }
        @keyframes dubPulseRing {
          0%   { transform: scale(0.92); opacity: 0.6; }
          60%  { transform: scale(1.08); opacity: 0.1; }
          100% { transform: scale(1.14); opacity: 0;   }
        }
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  );
};
