use crate::audio::{
    start_cpal_playback, start_loopback_capture, CaptureTarget, FeedbackGate, PlatformAudioEngine,
};
use crate::errors::DublyError;
use crate::gemini::run_gemini_live_session;
use crate::state::{AppState, AudioAppInfo, AudioDeviceInfo};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::{Emitter, State};

/// Capture chunks are 100 ms, so 200 slots is 20 s of slack before back-pressure.
const PCM_CHANNEL_CAPACITY: usize = 200;
/// Playback chunks are small; this bounds how far ahead the dub can run.
const PLAYBACK_CHANNEL_CAPACITY: usize = 200;

#[tauri::command]
pub async fn get_audio_applications() -> Result<Vec<AudioAppInfo>, DublyError> {
    PlatformAudioEngine::get_applications()
}

#[tauri::command]
pub async fn get_output_devices() -> Result<Vec<AudioDeviceInfo>, DublyError> {
    PlatformAudioEngine::get_output_devices()
}

#[tauri::command]
pub async fn start_dubbing(
    app: tauri::AppHandle,
    app_id: Option<String>,
    source_language: String,
    target_language: String,
    api_key: String,
    output_device_id: Option<String>,
    model: Option<String>,
    voice_tone: Option<String>,
    voice_vibe: Option<String>,
    state: State<'_, AppState>,
) -> Result<bool, DublyError> {
    if api_key.trim().is_empty() {
        let _ = app.emit("dubbing_log", "[ERROR] start_dubbing rejected: Gemini API key is empty");
        return Err(DublyError::IpcError(
            "Gemini API key is required".to_string(),
        ));
    }

    // Resolve the capture target before touching any state, so a bad selection
    // fails loudly instead of quietly falling back to whole-device capture.
    let capture_target = CaptureTarget::from_app_id(app_id.as_deref()).map_err(|e| {
        let _ = app.emit("dubbing_log", format!("[ERROR] start_dubbing rejected: {}", e));
        e
    })?;

    state.pipeline_stop.store(false, Ordering::SeqCst);
    *state.is_dubbing_active.lock().unwrap() = true;
    *state.selected_app_id.lock().unwrap() = app_id.clone();
    *state.source_language.lock().unwrap() = source_language.clone();
    *state.target_language.lock().unwrap() = target_language.clone();
    *state.api_key.lock().unwrap() = api_key.clone();

    let stop = Arc::clone(&state.pipeline_stop);
    let dubbed_volume = Arc::clone(&state.dubbed_volume);
    let is_dubbed_muted = Arc::clone(&state.is_dubbed_muted);
    let is_dubbing_active = Arc::clone(&state.is_dubbing_active);

    let (pcm_tx, pcm_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(PCM_CHANNEL_CAPACITY);
    let (playback_tx, playback_rx) =
        std::sync::mpsc::sync_channel::<Vec<u8>>(PLAYBACK_CHANNEL_CAPACITY);

    let output_idx: Option<usize> = output_device_id
        .as_deref()
        .and_then(|id| id.strip_prefix("dev_").and_then(|n| n.parse::<usize>().ok()));

    // The echo guard is only needed when capture and playback share the same
    // endpoint, which is exactly the whole-device case.
    let gate = Arc::new(FeedbackGate::new(capture_target.needs_echo_guard()));

    let _ = app.emit(
        "dubbing_log",
        format!(
            "[SYS] Starting dubbing pipeline. Source: {:?}, target lang: {}",
            capture_target, target_language
        ),
    );
    let _ = app.emit("dubbing_status", "connecting");

    start_loopback_capture(
        pcm_tx,
        Arc::clone(&stop),
        app.clone(),
        capture_target.pid(),
        Arc::clone(&gate),
    );

    start_cpal_playback(
        playback_rx,
        Arc::clone(&stop),
        dubbed_volume,
        is_dubbed_muted,
        output_idx,
        app.clone(),
        Arc::clone(&gate),
    );

    let stop_gemini = Arc::clone(&stop);
    let app_gemini = app.clone();

    let gemini_model = model
        .filter(|m| !m.trim().is_empty())
        .unwrap_or_else(|| crate::gemini::GEMINI_LIVE_MODEL.to_string());

    tokio::spawn(async move {
        let _ = app_gemini.emit("dubbing_log", "[GEMINI] Connecting to Gemini Live WebSocket...");
        let result = run_gemini_live_session(
            api_key,
            target_language,
            gemini_model,
            voice_tone,
            voice_vibe,
            pcm_rx,
            playback_tx,
            stop_gemini.clone(),
            app_gemini.clone(),
        )
        .await;

        // Whatever happened, the pipeline is over: tear down the capture and
        // playback threads and let the UI leave the "active session" screen.
        stop_gemini.store(true, Ordering::SeqCst);
        if let Ok(mut active) = is_dubbing_active.lock() {
            *active = false;
        }

        match result {
            Ok(_) => {
                let _ = app_gemini.emit("dubbing_log", "[GEMINI] Session ended cleanly");
                let _ = app_gemini.emit("dubbing_status", "ready");
            }
            Err(e) => {
                let _ = app_gemini.emit("dubbing_log", format!("[ERROR] Gemini session error: {}", e));
                let _ = app_gemini.emit("dubbing_status", "error");
                let _ = app_gemini.emit("dubbing_error", e.to_string());
            }
        }
    });

    Ok(true)
}

#[tauri::command]
pub async fn stop_dubbing(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<bool, DublyError> {
    // Logged so a session that ends here is distinguishable from one the
    // server terminated - they look identical from the pipeline's side.
    let _ = app.emit("dubbing_log", "[SYS] Stop requested by user");
    state.pipeline_stop.store(true, Ordering::SeqCst);
    *state.is_dubbing_active.lock().unwrap() = false;
    Ok(true)
}

#[tauri::command]
pub async fn update_audio_volumes(
    dubbed_volume: f32,
    is_dubbed_muted: bool,
    state: State<'_, AppState>,
) -> Result<bool, DublyError> {
    *state.dubbed_volume.lock().unwrap() = dubbed_volume / 100.0;
    *state.is_dubbed_muted.lock().unwrap() = is_dubbed_muted;
    Ok(true)
}
