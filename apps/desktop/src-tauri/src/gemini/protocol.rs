use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiVoiceConfig {
    pub voice_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPrebuiltVoice {
    pub prebuilt_voice_config: GeminiVoiceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiSpeechConfig {
    pub voice_config: GeminiPrebuiltVoice,
}

/// `translationConfig` — required by the `*-live-translate-*` models.
///
/// Without it the server falls back to `targetLanguageCode: "en"`, which is why
/// selecting Persian in the UI had no effect on the produced audio.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiTranslationConfig {
    /// BCP-47 code, e.g. `fa`, `en`, `zh-Hans`.
    pub target_language_code: String,
    /// `false` keeps the model silent when the input is already in the target
    /// language. That is what a dubbing app wants, and it also stops the model
    /// from parroting its own output back if any of it leaks into the capture.
    pub echo_target_language: bool,
}

/// Marker object: serialises to `{}`. Its presence enables transcripts.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GeminiTranscriptionConfig {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiGenerationConfig {
    pub response_modalities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<GeminiSpeechConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_config: Option<GeminiTranslationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiTextPart {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiSystemInstruction {
    pub parts: Vec<GeminiTextPart>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiSetup {
    pub model: String,
    pub generation_config: GeminiGenerationConfig,
    /// Translation models reject system instructions, so this stays `None` for
    /// them and is only populated for conversational Live models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<GeminiSystemInstruction>,
    /// Transcription configs live at the SETUP level, not inside
    /// generationConfig. Google's own live-translate WebSocket example puts
    /// them in generationConfig, but the real server rejects that with
    /// `Unknown name "inputAudioTranscription" at 'setup.generation_config'`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<GeminiTranscriptionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_transcription: Option<GeminiTranscriptionConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiSetupMessage {
    pub setup: GeminiSetup,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiBlob {
    pub mime_type: String,
    pub data: String,
}

/// `realtimeInput.audio` — the current field. `mediaChunks` is deprecated.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRealtimeInput {
    pub audio: GeminiBlob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiAudioInputMessage {
    pub realtime_input: GeminiRealtimeInput,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiInlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPart {
    pub inline_data: Option<GeminiInlineData>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiModelTurn {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiTranscription {
    pub text: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiServerContent {
    pub model_turn: Option<GeminiModelTurn>,
    pub turn_complete: Option<bool>,
    pub generation_complete: Option<bool>,
    /// Set when the server cancels the audio it was generating. Anything still
    /// queued for playback belongs to a turn that no longer exists and has to
    /// be dropped, otherwise it plays on top of the next one.
    pub interrupted: Option<bool>,
    pub input_transcription: Option<GeminiTranscription>,
    pub output_transcription: Option<GeminiTranscription>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiServerMessage {
    pub server_content: Option<GeminiServerContent>,
    pub setup_complete: Option<serde_json::Value>,
    pub go_away: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSetupConfig {
    pub model: String,
    pub source_language: String,
    pub target_language: String,
}
