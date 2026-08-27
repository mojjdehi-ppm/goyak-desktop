use crate::errors::DublyError;
use crate::state::{AudioAppInfo, AudioDeviceInfo};

pub struct MacOSAudioCapture;

impl MacOSAudioCapture {
    pub fn new() -> Self {
        Self
    }

    pub fn list_applications(&self) -> Result<Vec<AudioAppInfo>, DublyError> {
        // Ids must be either "system" or a numeric PID: anything else is
        // rejected by CaptureTarget::from_app_id.
        Ok(vec![
            AudioAppInfo {
                id: "system".to_string(),
                name: "System Audio (all applications)".to_string(),
                process_id: 0,
                is_audio_active: true,
            },
            AudioAppInfo {
                id: "101".to_string(),
                name: "Google Chrome".to_string(),
                process_id: 101,
                is_audio_active: true,
            },
            AudioAppInfo {
                id: "202".to_string(),
                name: "Safari".to_string(),
                process_id: 202,
                is_audio_active: false,
            },
        ])
    }

    pub fn list_output_devices(&self) -> Result<Vec<AudioDeviceInfo>, DublyError> {
        Ok(vec![AudioDeviceInfo {
            id: "default".to_string(),
            name: "MacBook Pro Speakers".to_string(),
            is_default: true,
        }])
    }
}
