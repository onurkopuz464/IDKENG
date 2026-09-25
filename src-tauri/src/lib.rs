mod clipboard;
mod gemini;
mod overlay;
mod paste;
mod session;
mod settings;
mod tray;

#[cfg(windows)]
mod hook;

use std::sync::Mutex;

pub struct AppState {
    pub last_translation: Mutex<Option<String>>,
    pub previous_hwnd: Mutex<isize>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            overlay::show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(AppState {
            last_translation: Mutex::new(None),
            previous_hwnd: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            overlay::hide_main,
            overlay::copy_translation,
            overlay::paste_translation,
            session::translate_text,
            settings::add_api_key,
            settings::select_api_key,
            settings::remove_api_key,
            settings::get_settings_state,
            settings::set_autostart,
            settings::set_close_to_tray,
            settings::set_start_minimized,
            settings::set_hotkey_binding,
            settings::set_app_prefs,
            settings::preview_prompt_rules,
            settings::get_traffic,
            settings::clear_traffic,
        ])
        .setup(|app| {
            settings::init(app.handle());
            tray::setup(app.handle())?;
            overlay::bind_close_to_hide(app.handle());

            #[cfg(windows)]
            hook::start(app.handle().clone());

            if settings::start_minimized() {
                let _ = overlay::hide_main(app.handle().clone());
            } else {
                overlay::show_main(app.handle());
            }

            if !settings::has_api_key() {
                overlay::open_settings(app.handle());
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("IDKENG başlatılamadı");
}
