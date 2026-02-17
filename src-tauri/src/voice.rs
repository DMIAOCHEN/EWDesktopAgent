// Voice module - Audio processing and TTS
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub wakeword_enabled: bool,
    pub asr_provider: String,
    pub tts_provider: String,
    pub language: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            wakeword_enabled: true,
            asr_provider: "third_party".to_string(),
            tts_provider: "kokoro".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}

pub struct VoiceService {
    config: VoiceConfig,
}

impl VoiceService {
    pub fn new(config: VoiceConfig) -> Self {
        Self { config }
    }

    // TODO: Implement Snowboy wake word detection
    // TODO: Implement Silero VAD integration
    // TODO: Implement ASR client for third-party API
    // TODO: Implement Kokoro TTS integration
    // TODO: Implement cloud TTS fallback
}
