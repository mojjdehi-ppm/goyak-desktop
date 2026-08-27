pub mod audio;
pub mod commands;
pub mod errors;
pub mod gemini;
pub mod platform;
pub mod state;

use state::AppState;
use tauri::{Listener, Manager};

/// Append one timestamped line to the on-disk log. Every failure here is
/// swallowed on purpose - logging must never take the pipeline down with it.
fn append_log_line(path: &std::path::Path, line: &str) {
    use std::io::Write;
    let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let epoch_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (h, m, s) = ((epoch_secs / 3600) % 24, (epoch_secs / 60) % 60, epoch_secs % 60);
    let _ = writeln!(file, "[{:02}:{:02}:{:02} UTC] {}", h, m, s, line);
}

/// Event payloads arrive JSON-encoded (a bare string comes as `"..."`).
fn decode_payload(raw: &str) -> String {
    serde_json::from_str::<String>(raw).unwrap_or_else(|_| raw.to_string())
}

/// Mirror every `dubbing_*` event into a log file so failures are diagnosable
/// after the fact. The UI shows the same events live, but a session that dies
/// and drops back to the setup screen used to leave no trace anywhere.
fn setup_file_logging(app: &tauri::App) {
    let Ok(dir) = app.path().app_log_dir() else {
        return;
    };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let path = dir.join("goyak.log");

    append_log_line(
        &path,
        &format!("=== Goyak {} started (log: {}) ===", app.package_info().version, path.display()),
    );

    let p = path.clone();
    app.listen_any("dubbing_log", move |event| {
        append_log_line(&p, &decode_payload(event.payload()));
    });

    let p = path.clone();
    app.listen_any("dubbing_status", move |event| {
        append_log_line(&p, &format!("[STATUS] {}", decode_payload(event.payload())));
    });

    app.listen_any("dubbing_error", move |event| {
        append_log_line(&path, &format!("[ERROR] {}", decode_payload(event.payload())));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::default())
        .setup(|app| {
            setup_file_logging(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_audio_applications,
            commands::get_output_devices,
            commands::start_dubbing,
            commands::stop_dubbing,
            commands::update_audio_volumes,
            commands::get_gemini_model,
            commands::get_system_platform,
            commands::test_gemini_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running dubly tauri application");
}
