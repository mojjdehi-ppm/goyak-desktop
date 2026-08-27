pub mod feedback;
pub mod linux;
pub mod macos;
pub mod pcm;
pub mod stream;
pub mod windows;

use crate::errors::DublyError;
use crate::state::{AudioAppInfo, AudioDeviceInfo};

pub use feedback::FeedbackGate;
pub use windows::{start_cpal_playback, start_loopback_capture, SYSTEM_AUDIO_ID};

pub trait AudioSource {
    fn list_applications(&self) -> Result<Vec<AudioAppInfo>, DublyError>;
    fn list_output_devices(&self) -> Result<Vec<AudioDeviceInfo>, DublyError>;
}

/// Which audio the pipeline should tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureTarget {
    /// Whole default render endpoint. Picks up our own dubbed output too, so it
    /// needs the echo guard.
    SystemWide,
    /// One process tree. Cannot contain Goyak's own playback.
    Process(u32),
}

impl CaptureTarget {
    /// Resolve the id the UI sent.
    ///
    /// An unrecognised id is an error rather than a silent fallback: falling
    /// back to `SystemWide` is what turned a stale selection into an echo loop.
    pub fn from_app_id(app_id: Option<&str>) -> Result<Self, DublyError> {
        match app_id.map(str::trim) {
            None | Some("") | Some(SYSTEM_AUDIO_ID) => Ok(CaptureTarget::SystemWide),
            Some(id) => id.parse::<u32>().map(CaptureTarget::Process).map_err(|_| {
                DublyError::IpcError(format!(
                    "Unknown audio source '{}'. Pick an application from the list, or choose System Audio.",
                    id
                ))
            }),
        }
    }

    pub fn pid(&self) -> u32 {
        match self {
            CaptureTarget::SystemWide => 0,
            CaptureTarget::Process(pid) => *pid,
        }
    }

    pub fn needs_echo_guard(&self) -> bool {
        matches!(self, CaptureTarget::SystemWide)
    }
}

pub struct PlatformAudioEngine;

impl PlatformAudioEngine {
    pub fn get_applications() -> Result<Vec<AudioAppInfo>, DublyError> {
        #[cfg(target_os = "windows")]
        {
            let capture = windows::WindowsAudioCapture::new();
            return capture.list_applications();
        }

        #[cfg(target_os = "macos")]
        {
            let capture = macos::MacOSAudioCapture::new();
            return capture.list_applications();
        }

        #[cfg(target_os = "linux")]
        {
            let capture = linux::LinuxPipeWireCapture::new();
            return capture.list_applications();
        }

        #[allow(unreachable_code)]
        Ok(vec![])
    }

    pub fn get_output_devices() -> Result<Vec<AudioDeviceInfo>, DublyError> {
        #[cfg(target_os = "windows")]
        {
            let capture = windows::WindowsAudioCapture::new();
            return capture.list_output_devices();
        }

        #[cfg(target_os = "macos")]
        {
            let capture = macos::MacOSAudioCapture::new();
            return capture.list_output_devices();
        }

        #[cfg(target_os = "linux")]
        {
            let capture = linux::LinuxPipeWireCapture::new();
            return capture.list_output_devices();
        }

        #[allow(unreachable_code)]
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_numeric_app_id_to_process_capture() {
        let target = CaptureTarget::from_app_id(Some("4242")).unwrap();
        assert_eq!(target, CaptureTarget::Process(4242));
        assert_eq!(target.pid(), 4242);
        assert!(!target.needs_echo_guard());
    }

    #[test]
    fn resolves_system_id_to_device_capture() {
        let target = CaptureTarget::from_app_id(Some(SYSTEM_AUDIO_ID)).unwrap();
        assert_eq!(target, CaptureTarget::SystemWide);
        assert_eq!(target.pid(), 0);
        assert!(target.needs_echo_guard());
    }

    #[test]
    fn missing_app_id_defaults_to_device_capture() {
        assert_eq!(
            CaptureTarget::from_app_id(None).unwrap(),
            CaptureTarget::SystemWide
        );
    }

    /// The bug behind issue #3: the UI's placeholder id "chrome" used to parse
    /// to PID 0, silently switching to whole-device loopback, which fed the
    /// dubbed audio straight back into the model.
    #[test]
    fn unknown_app_id_is_rejected_instead_of_capturing_the_whole_device() {
        assert!(CaptureTarget::from_app_id(Some("chrome")).is_err());
    }
}
