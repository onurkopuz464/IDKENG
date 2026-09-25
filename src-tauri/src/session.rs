use crate::gemini;
use crate::overlay;
use crate::settings;
use crate::AppState;
use tauri::{AppHandle, Manager};

const MAX_CHARS: usize = 8000;

pub fn on_shortcut(app: &AppHandle, hwnd: isize) {
    if let Ok(mut previous) = app.state::<AppState>().previous_hwnd.lock() {
        *previous = hwnd;
    }

    let copied = crate::paste::copy_from_hwnd(hwnd);
    let text = if copied {
        crate::clipboard::read_text_retry()
    } else if settings::hotkey_copies_selection() {
        crate::clipboard::read_text_retry()
    } else {
        None
    };

    let Some(text) = text else {
        overlay::show_error(app, None, "err:no_text");
        return;
    };

    start_translation(app, text);
}

#[tauri::command]
pub fn translate_text(app: AppHandle, source: String) -> Result<(), String> {
    start_translation(&app, source);
    Ok(())
}

pub fn start_translation(app: &AppHandle, text: String) {
    if text.trim().is_empty() {
        overlay::show_error(app, None, "err:no_text");
        return;
    }

    if text.chars().count() > MAX_CHARS {
        overlay::show_error(app, Some(&text), "err:too_long");
        return;
    }

    if !settings::has_api_key() {
        overlay::show_error(app, Some(&text), "err:no_key");
        overlay::open_settings(app);
        return;
    }

    overlay::show_loading(app, &text);

    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        match gemini::translate(&text).await {
            Ok(translation) => {
                if let Ok(mut last) = app_handle.state::<AppState>().last_translation.lock() {
                    *last = Some(translation.clone());
                }
                overlay::show_ready(&app_handle, &text, &translation);
            }
            Err(message) => overlay::show_error(&app_handle, Some(&text), &message),
        }
    });
}
