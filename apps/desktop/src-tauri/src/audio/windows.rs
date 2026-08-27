use crate::audio::feedback::FeedbackGate;
use crate::errors::DublyError;
use crate::state::{AudioAppInfo, AudioDeviceInfo};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::Emitter;

/// Id used by the UI to ask for whole-device loopback instead of one process.
pub const SYSTEM_AUDIO_ID: &str = "system";

/// Gemini Live wants 16 kHz mono 16-bit PCM in 100 ms chunks.
const CAPTURE_SAMPLE_RATE: u32 = 16000;
const CAPTURE_CHUNK_MS: usize = 100;
const CAPTURE_CHUNK_SAMPLES: usize = (CAPTURE_SAMPLE_RATE as usize / 1000) * CAPTURE_CHUNK_MS;

/// Gemini Live returns 24 kHz mono 16-bit PCM.
const PLAYBACK_SAMPLE_RATE: u32 = 24000;

/// Ceiling on queued dubbed audio. Gemini generates faster than real time, so
/// without a ceiling the queue grows forever and the dub drifts further and
/// further behind the picture until it sounds stuck.
const MAX_PLAYBACK_BUFFER_MS: usize = 4000;

pub struct WindowsAudioCapture;

impl WindowsAudioCapture {
    pub fn new() -> Self {
        Self
    }

    pub fn list_applications(&self) -> Result<Vec<AudioAppInfo>, DublyError> {
        use std::collections::HashSet;
        use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
        use windows::Win32::UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            IsWindowVisible,
        };

        let pids: HashSet<u32> = HashSet::new();
        let mut apps = Vec::new();

        unsafe extern "system" fn enum_window(hwnd: HWND, lparam: LPARAM) -> BOOL {
            if IsWindowVisible(hwnd).as_bool() {
                let length = GetWindowTextLengthW(hwnd);
                if length > 0 {
                    let mut buf = vec![0u16; (length + 1) as usize];
                    GetWindowTextW(hwnd, &mut buf);
                    let title = String::from_utf16_lossy(&buf);
                    let title = title.trim_end_matches('\0').trim();

                    if !title.is_empty() && title != "Program Manager" {
                        let mut pid = 0;
                        GetWindowThreadProcessId(hwnd, Some(&mut pid));

                        // Exclude our own application from the list
                        if pid != std::process::id() {
                            let data = &mut *(lparam.0 as *mut (HashSet<u32>, Vec<AudioAppInfo>));
                            if data.0.insert(pid) {
                                data.1.push(AudioAppInfo {
                                    id: pid.to_string(),
                                    name: title.to_string(),
                                    process_id: pid,
                                    is_audio_active: true,
                                });
                            }
                        }
                    }
                }
            }
            BOOL::from(true)
        }

        unsafe {
            let mut data = (pids, apps);
            let data_ptr = &mut data as *mut _ as isize;
            let _ = EnumWindows(Some(enum_window), LPARAM(data_ptr));
            apps = data.1;
        }

        // Whole-device capture is offered explicitly rather than being what you
        // silently get when an app id fails to resolve.
        apps.insert(
            0,
            AudioAppInfo {
                id: SYSTEM_AUDIO_ID.to_string(),
                name: "System Audio (all applications)".to_string(),
                process_id: 0,
                is_audio_active: true,
            },
        );

        Ok(apps)
    }

    pub fn list_output_devices(&self) -> Result<Vec<AudioDeviceInfo>, DublyError> {
        let host = cpal::default_host();
        let mut devices_list = Vec::new();
        let default_device_name = host.default_output_device().and_then(|d| d.name().ok());

        if let Ok(devices) = host.output_devices() {
            for (idx, dev) in devices.enumerate() {
                if let Ok(name) = dev.name() {
                    let is_default = default_device_name.as_ref().map_or(false, |d| d == &name);
                    devices_list.push(AudioDeviceInfo {
                        id: format!("dev_{}", idx),
                        name,
                        is_default,
                    });
                }
            }
        }

        if devices_list.is_empty() {
            devices_list.push(AudioDeviceInfo {
                id: "default".to_string(),
                name: "Default System Output Device".to_string(),
                is_default: true,
            });
        }

        Ok(devices_list)
    }
}

pub fn start_loopback_capture(
    pcm_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    stop: Arc<AtomicBool>,
    app: tauri::AppHandle,
    pid: u32,
    gate: Arc<FeedbackGate>,
) {
    std::thread::spawn(move || {
        wasapi_loopback_thread(pcm_tx, stop, app, pid, gate);
    });
}

use windows::core::{implement, w, Interface, HRESULT, PROPVARIANT};
use windows::Win32::Media::Audio::{
    IActivateAudioInterfaceAsyncOperation, IActivateAudioInterfaceCompletionHandler,
    IActivateAudioInterfaceCompletionHandler_Impl,
};

#[implement(IActivateAudioInterfaceCompletionHandler)]
struct AudioActivationHandler {
    tx: std::sync::mpsc::Sender<windows::core::Result<windows::Win32::Media::Audio::IAudioClient>>,
}

impl IActivateAudioInterfaceCompletionHandler_Impl for AudioActivationHandler_Impl {
    fn ActivateCompleted(
        &self,
        operation: Option<&IActivateAudioInterfaceAsyncOperation>,
    ) -> windows::core::Result<()> {
        if let Some(op) = operation {
            let mut status = HRESULT(0);
            let mut unk = None;
            unsafe {
                op.GetActivateResult(&mut status, &mut unk)?;
                if status.is_err() {
                    let _ = self.tx.send(Err(status.into()));
                } else if let Some(u) = unk {
                    let client: windows::Win32::Media::Audio::IAudioClient = u.cast()?;
                    let _ = self.tx.send(Ok(client));
                } else {
                    let _ = self.tx.send(Err(windows::core::Error::new(
                        HRESULT(0x80004005u32 as i32),
                        "Unknown activation error",
                    )));
                }
            }
        }
        Ok(())
    }
}

fn wasapi_loopback_thread(
    pcm_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    stop: Arc<AtomicBool>,
    app: tauri::AppHandle,
    pid: u32,
    gate: Arc<FeedbackGate>,
) {
    use windows::Win32::System::Com::*;

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        match wasapi_loopback_inner(&pcm_tx, &stop, &app, pid, &gate) {
            Ok(_) => {}
            Err(e) => {
                let _ = app.emit("dubbing_log", format!("[ERROR] WASAPI loopback: {}", e));
                let _ = app.emit("dubbing_status", "error");
                let _ = app.emit("dubbing_error", e.to_string());
            }
        }

        CoUninitialize();
    }
}

unsafe fn wasapi_loopback_inner(
    pcm_tx: &tokio::sync::mpsc::Sender<Vec<u8>>,
    stop: &Arc<AtomicBool>,
    app: &tauri::AppHandle,
    pid: u32,
    gate: &Arc<FeedbackGate>,
) -> windows::core::Result<()> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Media::Audio::*;
    use windows::Win32::System::Com::*;
    use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

    let audio_client: IAudioClient = if pid == 0 {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        device.Activate::<IAudioClient>(CLSCTX_ALL, None)?
    } else {
        let loopback_params = AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
            TargetProcessId: pid,
            ProcessLoopbackMode: PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
        };

        let mut activation_params: AUDIOCLIENT_ACTIVATION_PARAMS = std::mem::zeroed();
        activation_params.ActivationType = AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK;
        activation_params.Anonymous.ProcessLoopbackParams = loopback_params;

        #[repr(C)]
        struct InitPropVariant {
            vt: u16,
            reserved1: u16,
            reserved2: u16,
            reserved3: u16,
            blob: windows::Win32::System::Com::BLOB,
        }

        let blob = windows::Win32::System::Com::BLOB {
            cbSize: std::mem::size_of::<AUDIOCLIENT_ACTIVATION_PARAMS>() as u32,
            pBlobData: &mut activation_params as *mut _ as *mut u8,
        };

        let mut fake_prop: InitPropVariant = std::mem::zeroed();
        fake_prop.vt = 65; // VT_BLOB
        fake_prop.blob = blob;

        let render_id = w!(r#"VAD\Process_Loopback"#);
        let (tx, rx) = std::sync::mpsc::channel();
        let handler: IActivateAudioInterfaceCompletionHandler = AudioActivationHandler { tx }.into();
        let props_ptr = &fake_prop as *const _ as *const PROPVARIANT;

        ActivateAudioInterfaceAsync(render_id, &IAudioClient::IID, Some(&*props_ptr), &handler)?;

        match rx.recv() {
            Ok(Ok(client)) => client,
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                return Err(windows::core::Error::new(
                    HRESULT(0x80004005u32 as i32),
                    "Failed to receive audio client from async activation",
                ))
            }
        }
    };

    let (format_ptr, sample_rate, channels, bits) = if pid == 0 {
        let ptr = audio_client.GetMixFormat()?;
        (
            ptr,
            (*ptr).nSamplesPerSec,
            (*ptr).nChannels as usize,
            (*ptr).wBitsPerSample,
        )
    } else {
        const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;
        const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: windows::core::GUID = windows::core::GUID {
            data1: 0x00000003,
            data2: 0x0000,
            data3: 0x0010,
            data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
        };
        let wfx = WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE,
                nChannels: 2,
                nSamplesPerSec: 48000,
                wBitsPerSample: 32,
                nBlockAlign: (2 * 32) / 8,
                nAvgBytesPerSec: 48000 * ((2 * 32) / 8),
                cbSize: (std::mem::size_of::<WAVEFORMATEXTENSIBLE>()
                    - std::mem::size_of::<WAVEFORMATEX>()) as u16,
            },
            Samples: windows::Win32::Media::Audio::WAVEFORMATEXTENSIBLE_0 {
                wValidBitsPerSample: 32,
            },
            dwChannelMask: 3, // SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT
            SubFormat: KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
        };
        // Initialize reads this synchronously, but the allocation has to outlive
        // the call, so it goes on the heap and is freed at the end.
        let format_ptr = Box::into_raw(Box::new(wfx)) as *mut WAVEFORMATEX;
        (format_ptr, 48000, 2, 32)
    };

    let _ = app.emit(
        "dubbing_log",
        format!(
            "[WASAPI] Loopback format: {}Hz / {}ch / {}bit",
            sample_rate, channels, bits
        ),
    );

    let mut stream_flags = AUDCLNT_STREAMFLAGS_LOOPBACK;
    let mut buffer_duration = 10_000_000i64;
    let event_handle = if pid != 0 {
        stream_flags |= AUDCLNT_STREAMFLAGS_EVENTCALLBACK;
        buffer_duration = 0i64;
        CreateEventW(None, false, false, None)?
    } else {
        HANDLE::default()
    };

    audio_client.Initialize(
        AUDCLNT_SHAREMODE_SHARED,
        stream_flags,
        buffer_duration,
        0i64,
        format_ptr,
        None,
    )?;

    if pid != 0 {
        audio_client.SetEventHandle(event_handle)?;
    }

    let capture_client: IAudioCaptureClient = audio_client.GetService()?;
    audio_client.Start()?;

    if pid == 0 {
        let _ = app.emit(
            "dubbing_log",
            "[WASAPI] System-wide loopback active on the default render endpoint",
        );
    } else {
        let _ = app.emit(
            "dubbing_log",
            format!("[WASAPI] Process loopback active for PID {}", pid),
        );
    }

    // Averaging decimator. Source samples left over at the end of a packet are
    // carried into the next one, so window boundaries stay continuous instead
    // of restarting mid-stream and clicking at every packet edge. Averaging
    // (rather than picking one sample per window) also keeps the content above
    // 8 kHz from aliasing down into the speech band.
    let resample_step = sample_rate as f64 / CAPTURE_SAMPLE_RATE as f64;
    let mut carry: Vec<f32> = Vec::new();
    // Sub-sample phase left over from the previous packet. Rounding it away
    // each packet is worth ~0.2% of timing drift at rates like 22.05 kHz.
    let mut carry_phase: f64 = 0.0;
    let mut pending: Vec<i16> = Vec::with_capacity(CAPTURE_CHUNK_SAMPLES);
    let mut dropped_chunks: u64 = 0;

    // AUDCLNT_BUFFERFLAGS_SILENT
    const BUFFERFLAGS_SILENT: u32 = 0x2;

    while !stop.load(Ordering::Relaxed) {
        if pid != 0 {
            WaitForSingleObject(event_handle, 100);
        }

        let packet_size = capture_client.GetNextPacketSize()?;

        if packet_size == 0 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        let mut num_frames: u32 = 0;
        let mut flags: u32 = 0;

        capture_client.GetBuffer(&mut data_ptr, &mut num_frames, &mut flags, None, None)?;

        if num_frames == 0 || data_ptr.is_null() {
            capture_client.ReleaseBuffer(num_frames)?;
            continue;
        }

        let total_samples = num_frames as usize * channels;
        let is_silent = flags & BUFFERFLAGS_SILENT != 0;

        // When SILENT is set the buffer contents are undefined and must be
        // treated as digital silence rather than read as audio.
        let float_samples: Vec<f32> = if is_silent {
            vec![0.0; total_samples]
        } else if bits == 32 {
            std::slice::from_raw_parts(data_ptr as *const f32, total_samples).to_vec()
        } else if bits == 16 {
            std::slice::from_raw_parts(data_ptr as *const i16, total_samples)
                .iter()
                .map(|&s| s as f32 / 32768.0)
                .collect()
        } else {
            vec![0.0; total_samples]
        };

        capture_client.ReleaseBuffer(num_frames)?;

        if float_samples.is_empty() {
            continue;
        }

        let mono: Vec<f32> = if channels > 1 {
            float_samples
                .chunks(channels)
                .map(|c| c.iter().sum::<f32>() / channels as f32)
                .collect()
        } else {
            float_samples
        };

        // While our own dubbed audio is going out of the speakers, whatever the
        // device loopback hears is mostly that. Forwarding it would make the
        // model translate its own output over and over.
        if gate.should_suppress_capture() {
            pending.clear();
            carry.clear();
            carry_phase = 0.0;
            continue;
        }

        let mut src = std::mem::take(&mut carry);
        src.extend_from_slice(&mono);

        let mut pos: f64 = carry_phase;
        while (pos as usize) < src.len() && (pos + resample_step) as usize <= src.len() {
            let start = pos as usize;
            // At least one sample wide, never past the end of `src`.
            let end = ((pos + resample_step) as usize)
                .max(start + 1)
                .min(src.len());
            let window = &src[start..end];
            let avg = window.iter().sum::<f32>() / window.len() as f32;
            pending.push((avg.clamp(-1.0, 1.0) * 32767.0) as i16);
            pos += resample_step;

            if pending.len() >= CAPTURE_CHUNK_SAMPLES {
                let mut chunk = Vec::with_capacity(pending.len() * 2);
                for sample in pending.drain(..) {
                    chunk.extend_from_slice(&sample.to_le_bytes());
                }
                // Dropping a chunk is better than blocking the WASAPI thread,
                // which would overrun the capture buffer.
                if pcm_tx.try_send(chunk).is_err() {
                    dropped_chunks += 1;
                    if dropped_chunks % 20 == 1 {
                        let _ = app.emit(
                            "dubbing_log",
                            format!(
                                "[WASAPI] Upload queue full - dropped {} capture chunk(s)",
                                dropped_chunks
                            ),
                        );
                    }
                }
            }
        }

        let consumed = (pos as usize).min(src.len());
        carry_phase = (pos - consumed as f64).max(0.0);
        carry.clear();
        carry.extend_from_slice(&src[consumed..]);
    }

    audio_client.Stop()?;

    if pid != 0 {
        // Free the heap allocation made above; GetMixFormat memory is not ours.
        let _ = Box::from_raw(format_ptr as *mut WAVEFORMATEXTENSIBLE);
    } else {
        CoTaskMemFree(Some(format_ptr as *mut _));
    }

    let _ = app.emit("dubbing_log", "[WASAPI] Loopback capture stopped");

    Ok(())
}

pub fn start_cpal_playback(
    playback_rx: std::sync::mpsc::Receiver<Vec<u8>>,
    stop: Arc<AtomicBool>,
    dubbed_volume: Arc<std::sync::Mutex<f32>>,
    is_dubbed_muted: Arc<std::sync::Mutex<bool>>,
    output_device_idx: Option<usize>,
    app: tauri::AppHandle,
    gate: Arc<FeedbackGate>,
) {
    std::thread::spawn(move || {
        if let Err(e) = cpal_playback_inner(
            playback_rx,
            stop,
            dubbed_volume,
            is_dubbed_muted,
            output_device_idx,
            &app,
            gate,
        ) {
            let _ = app.emit("dubbing_log", format!("[ERROR] Playback: {}", e));
            let _ = app.emit("dubbing_status", "error");
            let _ = app.emit("dubbing_error", e.to_string());
        }
    });
}

fn cpal_playback_inner(
    playback_rx: std::sync::mpsc::Receiver<Vec<u8>>,
    stop: Arc<AtomicBool>,
    dubbed_volume: Arc<std::sync::Mutex<f32>>,
    is_dubbed_muted: Arc<std::sync::Mutex<bool>>,
    output_device_idx: Option<usize>,
    app: &tauri::AppHandle,
    gate: Arc<FeedbackGate>,
) -> Result<(), DublyError> {
    let host = cpal::default_host();

    let device = output_device_idx
        .and_then(|idx| host.output_devices().ok().and_then(|mut it| it.nth(idx)))
        .or_else(|| host.default_output_device())
        .ok_or_else(|| DublyError::AudioDeviceError("No audio output device available".into()))?;

    let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
    let _ = app.emit("dubbing_log", format!("[PLAYBACK] Output: {}", device_name));

    // Whole-device capture taps the default render endpoint. If the dub is
    // going somewhere else - headphones, a virtual cable - it can never come
    // back round, so capture can stay full duplex.
    if gate.is_enabled() {
        let default_name = host.default_output_device().and_then(|d| d.name().ok());
        let shares_endpoint = default_name.as_deref() == Some(device_name.as_str());
        if shares_endpoint {
            let _ = app.emit(
                "dubbing_log",
                "[PLAYBACK] Dub shares the captured endpoint - capture pauses while it speaks. \
                 Pick the source application, or send the dub to another output device, to avoid this.",
            );
        } else {
            gate.set_enabled(false);
            let _ = app.emit(
                "dubbing_log",
                "[PLAYBACK] Dub goes to a separate device - echo guard off, capture stays full duplex",
            );
        }
    }

    let config = device
        .default_output_config()
        .map_err(|e| DublyError::AudioDeviceError(format!("No output config: {}", e)))?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    let max_buffered_samples = (sample_rate as usize / 1000) * MAX_PLAYBACK_BUFFER_MS * channels;

    let audio_buf: Arc<std::sync::Mutex<std::collections::VecDeque<f32>>> =
        Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    let audio_buf_cb = audio_buf.clone();
    let vol_cb = dubbed_volume.clone();
    let mute_cb = is_dubbed_muted.clone();

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut buf = audio_buf_cb.lock().unwrap();
                let vol = *vol_cb.lock().unwrap();
                let muted = *mute_cb.lock().unwrap();
                for sample in data.iter_mut() {
                    let raw = buf.pop_front().unwrap_or(0.0);
                    *sample = if muted { 0.0 } else { raw * vol };
                }
            },
            |err| eprintln!("[CPAL] Output error: {:?}", err),
            None,
        )
        .map_err(|e| DublyError::AudioDeviceError(format!("Failed to build output stream: {}", e)))?;

    stream
        .play()
        .map_err(|e| DublyError::AudioDeviceError(format!("Failed to start playback: {}", e)))?;
    let _ = app.emit("dubbing_log", "[PLAYBACK] CPAL output stream started");

    let resample_step = PLAYBACK_SAMPLE_RATE as f64 / sample_rate as f64;
    // Carried across chunks so the read phase stays continuous.
    let mut resample_pos: f64 = 0.0;
    let mut trimmed: u64 = 0;

    while !stop.load(Ordering::Relaxed) {
        match playback_rx.try_recv() {
            Ok(pcm_bytes) => {
                let mut buf = audio_buf.lock().unwrap();

                // An empty frame is the interruption signal: the server threw
                // away the turn this audio belonged to, so drop it.
                if pcm_bytes.is_empty() {
                    buf.clear();
                    drop(buf);
                    resample_pos = 0.0;
                    gate.set_queued(0);
                    continue;
                }

                let in_samples = pcm_bytes.len() / 2;
                while (resample_pos as usize) < in_samples {
                    let byte = (resample_pos as usize) * 2;
                    let s16 = i16::from_le_bytes([pcm_bytes[byte], pcm_bytes[byte + 1]]);
                    let f = s16 as f32 / 32768.0;
                    for _ in 0..channels {
                        buf.push_back(f);
                    }
                    resample_pos += resample_step;
                }
                resample_pos -= in_samples as f64;

                if buf.len() > max_buffered_samples {
                    let excess = buf.len() - max_buffered_samples;
                    buf.drain(..excess);
                    trimmed += 1;
                    if trimmed % 10 == 1 {
                        let _ = app.emit(
                            "dubbing_log",
                            format!(
                                "[PLAYBACK] Output running behind - trimmed the queue to {} ms ({}x)",
                                MAX_PLAYBACK_BUFFER_MS, trimmed
                            ),
                        );
                    }
                }

                let queued_ms = (buf.len() / channels.max(1)) as u64 * 1000 / sample_rate as u64;
                drop(buf);

                // Muted output cannot feed back, so it must not gate capture.
                let muted = *is_dubbed_muted.lock().unwrap();
                gate.set_queued(if muted { 0 } else { queued_ms });
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
        }
    }

    drop(stream);
    gate.set_queued(0);
    let _ = app.emit("dubbing_log", "[PLAYBACK] CPAL output stream stopped");

    Ok(())
}
