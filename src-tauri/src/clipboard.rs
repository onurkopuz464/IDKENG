use std::thread;
use std::time::Duration;

pub fn read_text() -> Option<String> {
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard
        .get_text()
        .ok()
        .map(|s| s.replace("\r\n", "\n"))
        .filter(|s| !s.trim().is_empty())
}

pub fn read_text_retry() -> Option<String> {
    for _ in 0..8 {
        thread::sleep(Duration::from_millis(25));
        if let Some(text) = read_text() {
            return Some(text);
        }
    }
    None
}

pub fn write_text(text: &str) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())
}
