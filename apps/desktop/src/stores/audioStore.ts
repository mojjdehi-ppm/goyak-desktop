import { useState, useEffect, useCallback, useRef } from 'react';
import { AudioApplicationInfo, AudioDeviceInfo, AudioSourceType, DubbingStatus } from '../types/audio';
import {
  fetchAudioApplications,
  fetchAudioDevices,
  startDubbingCommand,
  stopDubbingCommand,
  updateVolumeCommand,
  listenDubbingEvents,
} from '../lib/tauri';
import { useSettingsStore } from './settingsStore';

/** Must match `SYSTEM_AUDIO_ID` in `src-tauri/src/audio/windows.rs`. */
export const SYSTEM_AUDIO_ID = 'system';

/** One dialog entry of the live transcript: original speech and its dub. */
export interface TranscriptEntry {
  id: number;
  heard: string;
  dubbed: string;
}

/** Audio silence longer than this starts a new transcript entry. */
const SEGMENT_GAP_MS = 2500;
const MAX_TRANSCRIPT_ENTRIES = 60;

export interface AudioStoreState {
  status: DubbingStatus;
  selectedAppId: string;
  applications: AudioApplicationInfo[];
  outputDevices: AudioDeviceInfo[];
  selectedOutputId: string;
  dubbedVolume: number;
  isDubbedMuted: boolean;
  sourceLanguage: string;
  targetLanguage: string;
  errorMessage: string | null;
}

export function useAudioStore() {
  const { settings, updateSettings } = useSettingsStore();

  const [status, setStatus] = useState<DubbingStatus>('ready');
  // 'system' is the whole-device source the backend actually understands. The
  // old 'chrome' placeholder matched nothing on Windows.
  const [selectedAppId, setSelectedAppId] = useState<string>(SYSTEM_AUDIO_ID);
  const [applications, setApplications] = useState<AudioApplicationInfo[]>([]);
  const [outputDevices, setOutputDevices] = useState<AudioDeviceInfo[]>([]);
  const [selectedOutputId, setSelectedOutputId] = useState<string>(settings.outputDeviceId || 'default');

  const [dubbedVolume, setDubbedVolumeState] = useState<number>(settings.dubbedVolume);
  const [isDubbedMuted, setIsDubbedMutedState] = useState<boolean>(settings.isDubbedMuted);

  const [sourceLanguage, setSourceLanguage] = useState<string>(settings.sourceLanguage || 'auto');
  const [targetLanguage, setTargetLanguage] = useState<string>(settings.targetLanguage || 'fa');

  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [logs, setLogs] = useState<string[]>([
    `[${new Date().toLocaleTimeString()}] [SYS] Dubly Real-Time Audio Engine Ready`,
  ]);

  // Live session feedback: the last pipeline event, plus a chat-style
  // transcript. Fragments arrive incrementally; a pause in the audio longer
  // than SEGMENT_GAP_MS starts a new dialog entry, like messages in a chat.
  const [statusLine, setStatusLine] = useState<string | null>(null);
  const [transcript, setTranscript] = useState<TranscriptEntry[]>([]);
  const lastFragmentAtRef = useRef<number>(0);
  const nextEntryIdRef = useRef<number>(1);

  const appendTranscriptFragment = useCallback((kind: 'heard' | 'dubbed', text: string) => {
    const now = Date.now();
    const isNewEntry = now - lastFragmentAtRef.current > SEGMENT_GAP_MS;
    lastFragmentAtRef.current = now;
    setTranscript((prev) => {
      const entries =
        isNewEntry || prev.length === 0
          ? [...prev.slice(-(MAX_TRANSCRIPT_ENTRIES - 1)), { id: nextEntryIdRef.current++, heard: '', dubbed: '' }]
          : prev;
      const last = entries[entries.length - 1];
      const updated = { ...last, [kind]: last[kind] + text };
      return [...entries.slice(0, -1), updated];
    });
  }, []);

  const unlistenRef = useRef<(() => void) | null>(null);

  const addLog = useCallback((msg: string) => {
    const time = new Date().toLocaleTimeString();
    setLogs((prev) => [...prev.slice(-49), `[${time}] ${msg}`]);
  }, []);

  const refreshDevicesAndApps = useCallback(async () => {
    const apps = await fetchAudioApplications();
    const devs = await fetchAudioDevices();
    setApplications(apps);
    setOutputDevices(devs);
    // Keep the selection pointing at a source that still exists. A stale id was
    // previously sent to the backend unchanged, where it degraded into
    // whole-device capture without anyone being told.
    setSelectedAppId((current) =>
      apps.length === 0 || apps.some((a) => a.id === current) ? current : apps[0].id,
    );
    // Same reconciliation for the output device. The stored id ('default', or a
    // dev_N from a previous boot) can point at nothing; the <select> then
    // DISPLAYS the first device while the backend plays to the system default -
    // which on this machine was a disconnected "Digital Output". Silent dub.
    setSelectedOutputId((current) =>
      devs.length === 0 || devs.some((d) => d.id === current)
        ? current
        : (devs.find((d) => d.isDefault) ?? devs[0]).id,
    );
  }, []);

  useEffect(() => {
    refreshDevicesAndApps();
    const interval = setInterval(refreshDevicesAndApps, 5000);
    return () => clearInterval(interval);
  }, [refreshDevicesAndApps]);

  const setupTauriListeners = useCallback(async () => {
    if (unlistenRef.current) {
      unlistenRef.current();
      unlistenRef.current = null;
    }
    const unlisten = await listenDubbingEvents(
      (s) => {
        if (s === 'playing') setStatus('playing');
        else if (s === 'connecting') setStatus('connecting');
        else if (s === 'ready') setStatus('ready');
        else if (s === 'error') setStatus('error');
      },
      (log) => {
        addLog(log);
        if (log.startsWith('[HEARD] ')) {
          appendTranscriptFragment('heard', log.slice(8));
        } else if (log.startsWith('[DUB] ')) {
          appendTranscriptFragment('dubbed', log.slice(6));
        } else {
          setStatusLine(log);
        }
      },
      (err) => {
        setErrorMessage(err);
        setStatus('error');
        addLog(`[ERROR] ${err}`);
      },
    );
    unlistenRef.current = unlisten;
  }, [addLog, appendTranscriptFragment]);

  useEffect(() => {
    setupTauriListeners();
    return () => {
      if (unlistenRef.current) unlistenRef.current();
    };
  }, [setupTauriListeners]);

  const toggleDubbing = async () => {
    if (status === 'ready' || status === 'error') {
      if (!settings.geminiApiKey || settings.geminiApiKey.trim() === '') {
        setStatus('error');
        setErrorMessage('Please enter your Gemini API Key in Settings (⚙️) to connect to Gemini Live AI.');
        addLog('[ERROR] Gemini API Key is missing. Live dubbing aborted.');
        return;
      }

      setStatus('connecting');
      setErrorMessage(null);
      setStatusLine(null);
      setTranscript([]);
      lastFragmentAtRef.current = 0;
      addLog(`[SYS] Starting pipeline — App: ${selectedAppId}`);
      addLog(`[WASAPI] Initializing Windows loopback capture...`);
      addLog(`[GEMINI] Tone: ${settings.voiceTone} | Vibe: ${settings.voiceVibe}`);

      let success = false;
      let failure: string | null = null;
      try {
        success = await startDubbingCommand({
          appId: selectedAppId,
          sourceLanguage,
          targetLanguage,
          apiKey: settings.geminiApiKey,
          outputDeviceId: selectedOutputId,
          model: settings.geminiModel || 'gemini-3.5-live-translate-preview',
          voiceTone: settings.voiceTone,
          voiceVibe: settings.voiceVibe,
        });
      } catch (e) {
        failure = typeof e === 'string' ? e : (e as Error)?.message ?? String(e);
      }

      if (!success) {
        const message =
          failure ?? 'Failed to start dubbing pipeline. Check your API key and try again.';
        setStatus('error');
        setErrorMessage(message);
        addLog(`[ERROR] ${message}`);
      } else {
        setStatus('listening');
        addLog(`[PCM] Audio capture active → sending to Gemini Live`);
      }
    } else {
      setStatus('ready');
      await stopDubbingCommand();
      addLog('[SYS] Dubbing pipeline stopped.');
    }
  };

  const setDubbedVolume = (val: number) => {
    setDubbedVolumeState(val);
    updateSettings({ dubbedVolume: val });
    updateVolumeCommand({ dubbedVolume: val, isDubbedMuted });
  };

  const toggleDubbedMute = () => {
    const next = !isDubbedMuted;
    setIsDubbedMutedState(next);
    updateSettings({ isDubbedMuted: next });
    updateVolumeCommand({ dubbedVolume, isDubbedMuted: next });
  };

  const changeOutputDevice = (id: string) => {
    setSelectedOutputId(id);
    updateSettings({ outputDeviceId: id });
    const dev = outputDevices.find((d) => d.id === id);
    if (dev) {
      addLog(`[AUDIO] Output device changed to: ${dev.name}`);
    }
  };

  const updateSourceLang = (lang: string) => {
    setSourceLanguage(lang);
    updateSettings({ sourceLanguage: lang });
  };

  const updateTargetLang = (lang: string) => {
    setTargetLanguage(lang);
    updateSettings({ targetLanguage: lang });
  };

  return {
    status,
    selectedAppId,
    setSelectedAppId,
    applications,
    outputDevices,
    selectedOutputId,
    setSelectedOutputId: changeOutputDevice,
    dubbedVolume,
    setDubbedVolume,
    isDubbedMuted,
    toggleDubbedMute,
    sourceLanguage,
    setSourceLanguage: updateSourceLang,
    targetLanguage,
    setTargetLanguage: updateTargetLang,
    errorMessage,
    logs,
    statusLine,
    transcript,
    toggleDubbing,
    refreshDevicesAndApps,
  };
}
