use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::clipboard;
use crate::paste;
use crate::settings;
use crate::AppState;

const WINDOW: &str = "main";

#[derive(Clone, Serialize)]
#[serde(tag = "kind")]
pub enum TranslationState {
    #[serde(rename = "loading")]
    Loading { source: String },
    #[serde(rename = "ready")]
    Ready {
        source: String,
        translation: String,
    },
    #[serde(rename = "error")]
    Error {
        source: Option<String>,
        message: String,
    },
}

pub fn bind_close_to_hide(app: &AppHandle) {
    let Some(window) = app.get_webview_window(WINDOW) else {
        return;
    };
    let hidden = window.clone();
    let handle = app.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            if settings::close_to_tray() {
                api.prevent_close();
                let _ = hidden.hide();
            } else {
                api.prevent_close();
                handle.exit(0);
            }
        }
    });
}

pub fn show_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(WINDOW) {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

pub fn open_settings(app: &AppHandle) {
    show_main(app);
    if let Some(window) = app.get_webview_window(WINDOW) {
        let _ = window.emit("open-settings", ());
    }
}

pub fn show_loading(app: &AppHandle, source: &str) {
    emit(
        app,
        &TranslationState::Loading {
            source: source.to_string(),
        },
    );
    show_main(app);
}

pub fn show_ready(app: &AppHandle, source: &str, translation: &str) {
    emit(
        app,
        &TranslationState::Ready {
            source: source.to_string(),
            translation: translation.to_string(),
        },
    );
    show_main(app);
}

pub fn show_error(app: &AppHandle, source: Option<&str>, message: &str) {
    emit(
        app,
        &TranslationState::Error {
            source: source.map(str::to_string),
            message: message.to_string(),
        },
    );
    show_main(app);
}

fn emit(app: &AppHandle, state: &TranslationState) {
    if let Some(window) = app.get_webview_window(WINDOW) {
        let _ = window.emit("translation-state", state);
    }
}

#[tauri::command]
pub fn hide_main(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(WINDOW) {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn copy_translation(state: State<AppState>) -> Result<(), String> {
    let text = state
        .last_translation
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "Çeviri yok.".to_string())?;
    clipboard::write_text(&text)
}

#[tauri::command]
pub fn paste_translation(state: State<AppState>) -> Result<(), String> {
    let text = state
        .last_translation
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "Çeviri yok.".to_string())?;
    clipboard::write_text(&text)?;

    let hwnd = *state.previous_hwnd.lock().map_err(|e| e.to_string())?;
    paste::paste_to_hwnd(hwnd);
    Ok(())
}
