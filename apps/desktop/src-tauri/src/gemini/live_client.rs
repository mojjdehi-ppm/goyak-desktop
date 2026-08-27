use crate::errors::DublyError;
use crate::gemini::protocol::{
    GeminiAudioInputMessage, GeminiBlob, GeminiGenerationConfig, GeminiPrebuiltVoice,
    GeminiRealtimeInput, GeminiServerMessage, GeminiSetup, GeminiSetupMessage, GeminiSpeechConfig,
    GeminiSystemInstruction, GeminiTextPart, GeminiTranscriptionConfig, GeminiTranslationConfig,
    GeminiVoiceConfig,
};
use base64::{engine::general_purpose, Engine as _};
use futures_util::{SinkExt, Stream, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub const GEMINI_LIVE_MODEL: &str = "gemini-3.5-live-translate-preview";

const GEMINI_WS_URL: &str = "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";

/// How long to wait for `setupComplete` before giving up.
const SETUP_TIMEOUT: Duration = Duration::from_secs(20);

/// Poll interval so both loops notice `stop` without waiting on the socket.
const IDLE_TICK: Duration = Duration::from_millis(250);

/// The `*-live-translate-*` family is a pure translation pipeline: it needs a
/// `translationConfig` and rejects system instructions, tools and voice
/// selection. Conversational Live models are the opposite. Getting this wrong is
/// what broke dubbing, so the model name decides which setup we build.
fn is_translate_model(model: &str) -> bool {
    model.contains("live-translate")
}

/// Map the UI language codes onto the BCP-47 codes the API expects.
fn to_bcp47(code: &str) -> &str {
    match code {
        // The UI offers a generic "Chinese"; the API wants an explicit script.
        "zh" => "zh-Hans",
        // "auto" is only meaningful as a source; a target must be concrete.
        "auto" | "" => "en",
        other => other,
    }
}

fn target_lang_name(code: &str) -> &'static str {
    match code {
        "fa" => "Persian (Farsi)",
        "en" => "English",
        "es" => "Spanish",
        "fr" => "French",
        "de" => "German",
        "ar" => "Arabic",
        "zh" => "Chinese (Mandarin)",
        "ja" => "Japanese",
        "ko" => "Korean",
        "ru" => "Russian",
        "tr" => "Turkish",
        _ => "Persian (Farsi)",
    }
}

fn build_agent_system_prompt(
    target_language: &str,
    voice_tone: Option<&str>,
    voice_vibe: Option<&str>,
) -> String {
    let lang_name = target_lang_name(target_language);
    let tone_str = voice_tone.unwrap_or("natural");
    let vibe_str = voice_vibe.unwrap_or("gaming");

    let tone_instruction = match tone_str {
        "energetic" => "Adopt an energetic, enthusiastic, and high-octane delivery style with expressive vocal peaks.",
        "calm" => "Adopt a calm, relaxed, gentle, and articulate delivery style with steady pacing.",
        "dramatic" => "Adopt a dramatic, intense, cinematic, and movie-trailer style voice full of emotion.",
        "funny" => "Adopt a playful, humorous, witty, and upbeat cartoon-like voice tone.",
        _ => "Adopt a natural, clear, and balanced human dubbing voice tone.",
    };

    let vibe_instruction = match vibe_str {
        "gaming" => "Context: Gaming & Esports stream. Speak like an excited gaming commentator or streamer.",
        "movies" => "Context: Movie or TV series dubbing. Match character acting, cinema drama, and theatrical dialogue.",
        "anime" => "Context: Anime & Cartoon dubbing. Use dynamic character voice acting with high emotion.",
        "podcast" => "Context: Podcast & Talkshow. Speak in a warm, personable, conversational podcast host style.",
        "vlog" => "Context: Vlog & Content creation. Speak in a friendly, engaging, casual vlogger tone.",
        "educational" => "Context: Educational documentary or tutorial. Speak clearly, authoritatively, and articulately.",
        _ => "Context: General dubbing. Maintain natural content flow.",
    };

    format!(
        r#"
You are a real-time AI dubbing voice actor and translator.

Translate input audio into {language}.
Output ONLY the translated speech audio.

Voice Style & Tone Guidelines:
- {tone_instruction}
- {vibe_instruction}

Core Rules:
- Speak like a professional voice actor.
- Match emotional peaks, laughter, tension, and excitement of original speakers.
- Maintain consistent voice identity, speed, and character.
- Do NOT add commentary, explanations, or meta text.

Names & Technical Terms Rule
Keep brand names, product names, technologies, programming terms, acronyms, and other proper nouns in their original language.
Do not translate them unless a widely accepted translation exists in the target language.
 terms like: Docker, Kubernetes, GitHub, JavaScript, TypeScript, Node.js, React, API, SDK, SQL, MongoDB, Redis, AWS, Gemini, Goyak, Windows, Tauri.

Make the dubbing sound authentic, immersive, and natural!
"#,
        language = lang_name,
        tone_instruction = tone_instruction,
        vibe_instruction = vibe_instruction
    )
}

/// Public so `test_gemini_connection` can send the exact same setup the real
/// session sends. The old test used a simplified setup, so it could report
/// "Connected!" while the actual dubbing session was rejected - the worst kind
/// of green checkmark.
pub fn build_setup_message(
    model_full: &str,
    target_language: &str,
    voice_tone: Option<&str>,
    voice_vibe: Option<&str>,
) -> GeminiSetupMessage {
    if is_translate_model(model_full) {
        GeminiSetupMessage {
            setup: GeminiSetup {
                model: model_full.to_string(),
                generation_config: GeminiGenerationConfig {
                    response_modalities: vec!["AUDIO".to_string()],
                    speech_config: None,
                    translation_config: Some(GeminiTranslationConfig {
                        target_language_code: to_bcp47(target_language).to_string(),
                        echo_target_language: false,
                    }),
                },
                system_instruction: None,
                input_audio_transcription: Some(GeminiTranscriptionConfig::default()),
                output_audio_transcription: Some(GeminiTranscriptionConfig::default()),
            },
        }
    } else {
        GeminiSetupMessage {
            setup: GeminiSetup {
                model: model_full.to_string(),
                generation_config: GeminiGenerationConfig {
                    response_modalities: vec!["AUDIO".to_string()],
                    speech_config: Some(GeminiSpeechConfig {
                        voice_config: GeminiPrebuiltVoice {
                            prebuilt_voice_config: GeminiVoiceConfig {
                                voice_name: "Aoede".to_string(),
                            },
                        },
                    }),
                    translation_config: None,
                },
                system_instruction: Some(GeminiSystemInstruction {
                    parts: vec![GeminiTextPart {
                        text: build_agent_system_prompt(target_language, voice_tone, voice_vibe),
                    }],
                }),
                input_audio_transcription: None,
                output_audio_transcription: None,
            },
        }
    }
}

pub async fn run_gemini_live_session(
    api_key: String,
    target_language: String,
    model: String,
    voice_tone: Option<String>,
    voice_vibe: Option<String>,
    mut pcm_rx: mpsc::Receiver<Vec<u8>>,
    playback_tx: std::sync::mpsc::SyncSender<Vec<u8>>,
    stop: Arc<AtomicBool>,
    app: tauri::AppHandle,
) -> Result<(), DublyError> {
    let url = format!("{}?key={}", GEMINI_WS_URL, api_key);

    let (ws_stream, _) = connect_async(&url)
        .await
        .map_err(|e| DublyError::GeminiConnectionError(e.to_string()))?;

    let _ = app.emit("dubbing_log", "[GEMINI] WebSocket connected to Gemini Live API");

    let (mut ws_write, mut ws_read) = ws_stream.split();

    let model_full = if model.starts_with("models/") {
        model.clone()
    } else {
        format!("models/{}", model)
    };

    let translate_mode = is_translate_model(&model_full);
    let setup_msg = build_setup_message(
        &model_full,
        &target_language,
        voice_tone.as_deref(),
        voice_vibe.as_deref(),
    );

    if translate_mode {
        let _ = app.emit(
            "dubbing_log",
            format!(
                "[GEMINI] Translation mode - targetLanguageCode={}",
                to_bcp47(&target_language)
            ),
        );
    } else {
        let _ = app.emit(
            "dubbing_log",
            format!("[GEMINI] Conversational Live mode - model {}", model_full),
        );
    }

    let setup_json =
        serde_json::to_string(&setup_msg).map_err(|e| DublyError::IpcError(e.to_string()))?;

    ws_write
        .send(Message::Text(setup_json))
        .await
        .map_err(|e| DublyError::GeminiConnectionError(e.to_string()))?;

    wait_for_setup_complete(&mut ws_read, &app).await?;

    let stop_sender = stop.clone();
    let app_sender = app.clone();

    let sender_task = tokio::spawn(async move {
        let mut chunks_sent: u64 = 0;
        while !stop_sender.load(Ordering::Relaxed) {
            match tokio::time::timeout(IDLE_TICK, pcm_rx.recv()).await {
                Ok(Some(pcm_bytes)) => {
                    let audio_msg = GeminiAudioInputMessage {
                        realtime_input: GeminiRealtimeInput {
                            audio: GeminiBlob {
                                mime_type: "audio/pcm;rate=16000".to_string(),
                                data: general_purpose::STANDARD.encode(&pcm_bytes),
                            },
                        },
                    };
                    let json = match serde_json::to_string(&audio_msg) {
                        Ok(j) => j,
                        Err(_) => continue,
                    };
                    if ws_write.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                    chunks_sent += 1;
                    // One line per chunk floods the log at 10 chunks/second.
                    if chunks_sent % 50 == 1 {
                        let _ = app_sender.emit(
                            "dubbing_log",
                            format!("[STREAM] {} PCM chunks sent to Gemini Live", chunks_sent),
                        );
                    }
                }
                Ok(None) => break,
                Err(_) => {}
            }
        }
        let _ = ws_write.send(Message::Close(None)).await;
    });

    let mut interruptions: u64 = 0;

    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        let msg = match tokio::time::timeout(IDLE_TICK, ws_read.next()).await {
            Err(_) => continue,
            Ok(None) => {
                let _ = app.emit("dubbing_log", "[GEMINI] Server ended the stream");
                break;
            }
            Ok(Some(Ok(m))) => m,
            Ok(Some(Err(e))) => {
                let _ = app.emit("dubbing_log", format!("[GEMINI] WS read error: {}", e));
                break;
            }
        };

        let text_payload = match msg {
            Message::Text(text) => Some(text),
            Message::Binary(bin) => String::from_utf8(bin).ok(),
            Message::Close(frame) => {
                let reason = frame
                    .map(|f| format!("{} ({:?})", f.reason, f.code))
                    .unwrap_or_else(|| "no reason given".to_string());
                let _ = app.emit(
                    "dubbing_log",
                    format!("[GEMINI] Server closed the connection: {}", reason),
                );
                break;
            }
            _ => None,
        };

        let text = match text_payload {
            Some(t) => t,
            None => continue,
        };

        let server_msg = match serde_json::from_str::<GeminiServerMessage>(&text) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if server_msg.go_away.is_some() {
            let _ = app.emit(
                "dubbing_log",
                "[GEMINI] Server sent goAway - session is about to end",
            );
        }

        let server_content = match server_msg.server_content {
            Some(c) => c,
            None => continue,
        };

        // Drop queued audio the server just cancelled. Without this the stale
        // audio keeps playing on top of whatever comes next.
        if server_content.interrupted.unwrap_or(false) {
            interruptions += 1;
            let _ = playback_tx.try_send(Vec::new());
            let _ = app.emit(
                "dubbing_log",
                format!(
                    "[PLAYBACK] Generation interrupted (#{}) - playback queue cleared",
                    interruptions
                ),
            );
        }

        if let Some(t) = server_content
            .input_transcription
            .as_ref()
            .and_then(|t| t.text.as_deref())
        {
            if !t.trim().is_empty() {
                let _ = app.emit("dubbing_log", format!("[HEARD] {}", t));
            }
        }

        if let Some(t) = server_content
            .output_transcription
            .as_ref()
            .and_then(|t| t.text.as_deref())
        {
            if !t.trim().is_empty() {
                let _ = app.emit("dubbing_log", format!("[DUB] {}", t));
            }
        }

        if let Some(model_turn) = server_content.model_turn {
            for part in model_turn.parts {
                let inline_data = match part.inline_data {
                    Some(d) => d,
                    None => continue,
                };
                let pcm_bytes = match general_purpose::STANDARD.decode(&inline_data.data) {
                    Ok(b) => b,
                    Err(_) => continue,
                };
                if pcm_bytes.is_empty() {
                    continue;
                }
                // A full queue means playback is already behind. Dropping the
                // newest chunk keeps latency bounded instead of unbounded.
                if playback_tx.try_send(pcm_bytes).is_err() {
                    let _ = app.emit(
                        "dubbing_log",
                        "[PLAYBACK] Output queue full - dropped a chunk to stay in sync",
                    );
                }
            }
        }
    }

    stop.store(true, Ordering::Relaxed);
    let _ = tokio::time::timeout(Duration::from_secs(2), sender_task).await;

    Ok(())
}

async fn wait_for_setup_complete<S>(
    ws_read: &mut S,
    app: &tauri::AppHandle,
) -> Result<(), DublyError>
where
    S: Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + SETUP_TIMEOUT;

    loop {
        let next = tokio::time::timeout_at(deadline, ws_read.next())
            .await
            .map_err(|_| {
                DublyError::GeminiConnectionError(
                    "Timed out waiting for setupComplete from Gemini Live".to_string(),
                )
            })?;

        let msg = match next {
            None => {
                return Err(DublyError::GeminiConnectionError(
                    "WebSocket closed before setup completed".to_string(),
                ))
            }
            Some(Err(e)) => {
                return Err(DublyError::GeminiConnectionError(format!(
                    "WS read error during setup: {}",
                    e
                )))
            }
            Some(Ok(m)) => m,
        };

        let text = match msg {
            Message::Text(text) => text,
            Message::Binary(bin) => match String::from_utf8(bin) {
                Ok(t) => t,
                Err(_) => continue,
            },
            Message::Close(frame) => {
                let reason = frame
                    .map(|f| format!("{} ({:?})", f.reason, f.code))
                    .unwrap_or_else(|| "no reason given".to_string());
                return Err(DublyError::GeminiConnectionError(format!(
                    "Server rejected the session setup: {}",
                    reason
                )));
            }
            _ => continue,
        };

        let parsed = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if parsed.get("setupComplete").is_some() {
            let _ = app.emit("dubbing_log", "[GEMINI] Setup complete. Streaming audio...");
            let _ = app.emit("dubbing_status", "playing");
            return Ok(());
        }

        if let Some(error) = parsed.get("error") {
            return Err(DublyError::GeminiConnectionError(format!(
                "Gemini rejected the setup: {}",
                error
            )));
        }

        let _ = app.emit(
            "dubbing_log",
            format!(
                "[GEMINI] Unexpected setup frame: {}",
                text.chars().take(300).collect::<String>()
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pins the wire format the server actually accepts. The live-translate
    /// docs' own WebSocket example nests the transcription configs inside
    /// generationConfig, and the server rejects that exact shape with
    /// `Unknown name "inputAudioTranscription" at 'setup.generation_config'`.
    #[test]
    fn translate_setup_puts_transcription_at_setup_level() {
        let msg =
            build_setup_message("models/gemini-3.5-live-translate-preview", "fa", None, None);
        let json = serde_json::to_value(&msg).unwrap();
        let setup = &json["setup"];

        assert!(setup["inputAudioTranscription"].is_object());
        assert!(setup["outputAudioTranscription"].is_object());
        assert!(setup["generationConfig"]["inputAudioTranscription"].is_null());
        assert!(setup["generationConfig"]["outputAudioTranscription"].is_null());
        assert_eq!(
            setup["generationConfig"]["translationConfig"]["targetLanguageCode"],
            "fa"
        );
        assert_eq!(
            setup["generationConfig"]["translationConfig"]["echoTargetLanguage"],
            false
        );
        // Translate models reject system instructions and voice selection.
        assert!(setup["systemInstruction"].is_null());
        assert!(setup["generationConfig"]["speechConfig"].is_null());
    }

    #[test]
    fn agent_setup_keeps_prompt_and_voice() {
        let msg = build_setup_message(
            "models/gemini-2.5-flash-native-audio",
            "fa",
            Some("calm"),
            Some("movies"),
        );
        let json = serde_json::to_value(&msg).unwrap();
        let setup = &json["setup"];

        assert!(setup["systemInstruction"].is_object());
        assert!(setup["generationConfig"]["speechConfig"].is_object());
        assert!(setup["generationConfig"]["translationConfig"].is_null());
        assert!(setup["inputAudioTranscription"].is_null());
        assert!(setup["outputAudioTranscription"].is_null());
    }

    #[test]
    fn ui_language_codes_map_to_bcp47() {
        assert_eq!(to_bcp47("zh"), "zh-Hans");
        assert_eq!(to_bcp47("auto"), "en");
        assert_eq!(to_bcp47(""), "en");
        assert_eq!(to_bcp47("fa"), "fa");
    }

    #[test]
    fn audio_goes_out_as_realtime_input_audio_blob() {
        let msg = GeminiAudioInputMessage {
            realtime_input: GeminiRealtimeInput {
                audio: GeminiBlob {
                    mime_type: "audio/pcm;rate=16000".to_string(),
                    data: "AAAA".to_string(),
                },
            },
        };
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(
            json["realtimeInput"]["audio"]["mimeType"],
            "audio/pcm;rate=16000"
        );
        // The deprecated field must not resurface.
        assert!(json["realtimeInput"]["mediaChunks"].is_null());
    }
}
