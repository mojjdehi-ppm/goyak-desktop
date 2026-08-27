use crate::errors::DublyError;
use crate::state::{AudioAppInfo, AudioDeviceInfo};

pub struct LinuxPipeWireCapture;

impl LinuxPipeWireCapture {
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
                id: "303".to_string(),
                name: "Firefox PipeWire Stream".to_string(),
                process_id: 303,
                is_audio_active: true,
            },
        ])
    }

    pub fn list_output_devices(&self) -> Result<Vec<AudioDeviceInfo>, DublyError> {
        Ok(vec![AudioDeviceInfo {
            id: "default".to_string(),
            name: "ALSA / PipeWire Sink".to_string(),
            is_default: true,
        }])
    }
}
