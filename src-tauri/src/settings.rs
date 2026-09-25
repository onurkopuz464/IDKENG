use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;

static KEY_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
static KEY_STORE: Mutex<KeyStore> = Mutex::new(KeyStore {
    active_id: None,
    keys: Vec::new(),
});
static PREFS_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
static PREFS: Mutex<Prefs> = Mutex::new(Prefs {
    close_to_tray: true,
    start_minimized: false,
    hotkey: Hotkey {
        ctrl: true,
        alt: false,
        shift: false,
        win: false,
        vk: 0x43,
        taps: 2,
    },
    ui_language: String::new(),
    native_language: String::new(),
    foreign_language: String::new(),
    custom_prompt: String::new(),
    prompt_rules: Vec::new(),
    rules_customized: false,
    timeout_secs: 20,
    max_output_tokens: 2048,
    fallback_models: true,
});
static TRAFFIC_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
static TRAFFIC: Mutex<Vec<TrafficEntry>> = Mutex::new(Vec::new());

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct KeyStore {
    active_id: Option<String>,
    keys: Vec<KeyEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct KeyEntry {
    id: String,
    name: String,
    key: String,
    #[serde(default = "default_provider")]
    provider: String,
    #[serde(default)]
    model: String,
    #[serde(default)]
    base_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Prefs {
    #[serde(default = "default_true")]
    close_to_tray: bool,
    #[serde(default)]
    start_minimized: bool,
    #[serde(default = "default_hotkey")]
    hotkey: Hotkey,
    #[serde(default = "default_ui_lang")]
    ui_language: String,
    #[serde(default = "default_native")]
    native_language: String,
    #[serde(default = "default_foreign")]
    foreign_language: String,
    #[serde(default)]
    custom_prompt: String,
    #[serde(default)]
    prompt_rules: Vec<String>,
    #[serde(default)]
    rules_customized: bool,
    #[serde(default = "default_timeout")]
    timeout_secs: u64,
    #[serde(default = "default_tokens")]
    max_output_tokens: u32,
    #[serde(default = "default_true")]
    fallback_models: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Hotkey {
    #[serde(default = "default_true")]
    pub ctrl: bool,
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub shift: bool,
    #[serde(default)]
    pub win: bool,
    #[serde(default = "default_vk_c")]
    pub vk: u32,
    #[serde(default = "default_taps")]
    pub taps: u8,
}

fn default_true() -> bool {
    true
}

fn default_vk_c() -> u32 {
    0x43
}

fn default_taps() -> u8 {
    2
}

fn default_hotkey() -> Hotkey {
    Hotkey {
        ctrl: true,
        alt: false,
        shift: false,
        win: false,
        vk: default_vk_c(),
        taps: default_taps(),
    }
}

fn default_provider() -> String {
    "gemini".into()
}

fn default_ui_lang() -> String {
    "tr".into()
}

fn default_native() -> String {
    "tr".into()
}

fn default_foreign() -> String {
    "en".into()
}

fn default_timeout() -> u64 {
    20
}

fn default_tokens() -> u32 {
    2048
}

fn empty_prefs() -> Prefs {
    Prefs {
        close_to_tray: true,
        start_minimized: false,
        hotkey: default_hotkey(),
        ui_language: default_ui_lang(),
        native_language: default_native(),
        foreign_language: default_foreign(),
        custom_prompt: String::new(),
        prompt_rules: Vec::new(),
        rules_customized: false,
        timeout_secs: default_timeout(),
        max_output_tokens: default_tokens(),
        fallback_models: true,
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    id: String,
    name: String,
    provider: String,
    model: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    has_api_key: bool,
    active_key_id: Option<String>,
    api_keys: Vec<ApiKeyInfo>,
    autostart: bool,
    close_to_tray: bool,
    start_minimized: bool,
    hotkey_label: String,
    can_autostart: bool,
    ui_language: String,
    native_language: String,
    foreign_language: String,
    prompt_rules: Vec<String>,
    rules_customized: bool,
    timeout_secs: u64,
    max_output_tokens: u32,
    fallback_models: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrafficEntry {
    pub id: String,
    pub at_ms: u64,
    pub provider: String,
    pub model: String,
    pub profile_name: String,
    pub elapsed_ms: u64,
    pub source: String,
    pub system_prompt: String,
    pub user_message: String,
    pub response: String,
    pub error: String,
}

fn new_id() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| format!("{:x}", d.as_nanos()))
        .unwrap_or_else(|_| "key".into())
}

pub fn init(app: &AppHandle) {
    let Ok(dir) = app.path().app_data_dir() else {
        return;
    };
    let _ = fs::create_dir_all(&dir);
    let key_path = dir.join("api-keys.bin");
    let legacy_path = dir.join("api-key.bin");
    let prefs_path = dir.join("prefs.json");

    if key_path.exists() {
        if let Ok(store) = read_store(&key_path) {
            if let Ok(mut slot) = KEY_STORE.lock() {
                *slot = store;
            }
        }
    } else if legacy_path.exists() {
        if let Ok(key) = read_protected_string(&legacy_path) {
            let id = new_id();
            let store = KeyStore {
                active_id: Some(id.clone()),
                keys: vec![KeyEntry {
                    id,
                    name: "Varsayılan".into(),
                    key,
                    provider: default_provider(),
                    model: String::new(),
                    base_url: String::new(),
                }],
            };
            if let Ok(mut slot) = KEY_STORE.lock() {
                *slot = store.clone();
            }
            if let Ok(mut slot) = KEY_FILE.lock() {
                *slot = Some(key_path.clone());
            }
            let _ = persist_keys();
        }
    }

    if let Ok(mut slot) = KEY_FILE.lock() {
        *slot = Some(key_path);
    }

    if prefs_path.exists() {
        if let Ok(bytes) = fs::read(&prefs_path) {
            if let Ok(mut loaded) = serde_json::from_slice::<Prefs>(&bytes) {
                if validate_hotkey(&loaded.hotkey).is_err() {
                    loaded.hotkey = default_hotkey();
                }
                normalize_prefs(&mut loaded);
                if let Ok(mut prefs) = PREFS.lock() {
                    *prefs = loaded;
                }
            }
        }
    } else if let Ok(mut prefs) = PREFS.lock() {
        *prefs = empty_prefs();
    }

    if let Ok(mut slot) = PREFS_FILE.lock() {
        *slot = Some(prefs_path);
    }

    let traffic_path = dir.join("traffic.json");
    if traffic_path.exists() {
        if let Ok(bytes) = fs::read(&traffic_path) {
            if let Ok(loaded) = serde_json::from_slice::<Vec<TrafficEntry>>(&bytes) {
                if let Ok(mut slot) = TRAFFIC.lock() {
                    *slot = loaded;
                }
            }
        }
    }
    if let Ok(mut slot) = TRAFFIC_FILE.lock() {
        *slot = Some(traffic_path);
    }
}

pub fn close_to_tray() -> bool {
    PREFS.lock().map(|p| p.close_to_tray).unwrap_or(true)
}

pub fn start_minimized() -> bool {
    PREFS.lock().map(|p| p.start_minimized).unwrap_or(false)
}

pub fn hotkey() -> Hotkey {
    PREFS
        .lock()
        .map(|prefs| prefs.hotkey)
        .unwrap_or_else(|_| default_hotkey())
}

pub fn hotkey_label() -> String {
    format_hotkey(hotkey())
}

pub fn hotkey_copies_selection() -> bool {
    let hotkey = hotkey();
    hotkey.ctrl && !hotkey.alt && !hotkey.shift && !hotkey.win && hotkey.vk == 0x43 && hotkey.taps >= 2
}

pub fn validate_hotkey(hotkey: &Hotkey) -> Result<(), String> {
    if hotkey.taps != 1 && hotkey.taps != 2 {
        return Err("Kısayol 1 veya 2 vuruş olmalı.".into());
    }
    if is_modifier_vk(hotkey.vk) {
        return Err("Yalnızca Ctrl, Alt, Shift veya Win yetmez.".into());
    }
    if hotkey.vk == 0x1B {
        return Err("Esc kısayol olarak kullanılamaz.".into());
    }
    let has_mod = hotkey.ctrl || hotkey.alt || hotkey.shift || hotkey.win;
    if !has_mod && !is_function_key(hotkey.vk) {
        return Err("Harf tuşu için Ctrl, Alt, Shift veya Win kullan.".into());
    }
    Ok(())
}

pub fn set_hotkey(hotkey: Hotkey) -> Result<(), String> {
    validate_hotkey(&hotkey)?;
    {
        let mut prefs = PREFS.lock().map_err(|e| e.to_string())?;
        prefs.hotkey = hotkey;
    }
    persist_prefs()?;
    #[cfg(windows)]
    crate::hook::reset_pending();
    Ok(())
}

#[tauri::command]
pub fn set_hotkey_binding(hotkey: Hotkey) -> Result<String, String> {
    set_hotkey(hotkey)?;
    Ok(hotkey_label())
}

fn is_modifier_vk(vk: u32) -> bool {
    matches!(
        vk,
        0x10 | 0x11 | 0x12 | 0x5B | 0x5C | 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5
    )
}

fn is_function_key(vk: u32) -> bool {
    (0x70..=0x87).contains(&vk)
}

fn format_hotkey(hotkey: Hotkey) -> String {
    let mut parts = Vec::new();
    if hotkey.ctrl {
        parts.push("Ctrl".to_string());
    }
    if hotkey.alt {
        parts.push("Alt".to_string());
    }
    if hotkey.shift {
        parts.push("Shift".to_string());
    }
    if hotkey.win {
        parts.push("Win".to_string());
    }
    let key = vk_name(hotkey.vk);
    parts.push(key.clone());
    if hotkey.taps == 2 {
        parts.push(key);
    }
    parts.join("+")
}

fn vk_name(vk: u32) -> String {
    match vk {
        0x08 => "Backspace".into(),
        0x09 => "Tab".into(),
        0x0D => "Enter".into(),
        0x1B => "Esc".into(),
        0x20 => "Space".into(),
        0x21 => "PageUp".into(),
        0x22 => "PageDown".into(),
        0x23 => "End".into(),
        0x24 => "Home".into(),
        0x25 => "Left".into(),
        0x26 => "Up".into(),
        0x27 => "Right".into(),
        0x28 => "Down".into(),
        0x2C => "PrintScreen".into(),
        0x2D => "Insert".into(),
        0x2E => "Delete".into(),
        0x30..=0x39 => ((b'0' + (vk - 0x30) as u8) as char).to_string(),
        0x41..=0x5A => ((b'A' + (vk - 0x41) as u8) as char).to_string(),
        0x60..=0x69 => format!("Num{}", vk - 0x60),
        0x6A => "Num*".into(),
        0x6B => "Num+".into(),
        0x6D => "Num-".into(),
        0x6E => "Num.".into(),
        0x6F => "Num/".into(),
        0x70..=0x87 => format!("F{}", vk - 0x6F),
        0xBA => ";".into(),
        0xBB => "=".into(),
        0xBC => ",".into(),
        0xBD => "-".into(),
        0xBE => ".".into(),
        0xBF => "/".into(),
        0xC0 => "`".into(),
        0xDB => "[".into(),
        0xDC => "\\".into(),
        0xDD => "]".into(),
        0xDE => "'".into(),
        _ => format!("Vk{vk:02X}"),
    }
}

fn persist_prefs() -> Result<(), String> {
    let prefs = PREFS.lock().map_err(|e| e.to_string())?.clone();
    let path = PREFS_FILE
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "Ayar dosyası yok.".to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_vec_pretty(&prefs).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn persist_keys() -> Result<(), String> {
    let store = KEY_STORE.lock().map_err(|e| e.to_string())?.clone();
    let path = KEY_FILE
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "Anahtar dosyası yolu yok.".to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_vec(&store).map_err(|e| e.to_string())?;
    let protected = protect(&json)?;
    fs::write(path, protected).map_err(|e| format!("Anahtarlar yazılamadı: {e}"))
}

fn read_store(path: &Path) -> Result<KeyStore, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let plain = unprotect(&bytes)?;
    serde_json::from_slice(&plain).map_err(|_| "Anahtar deposu bozuk.".to_string())
}

fn key_summaries(store: &KeyStore) -> Vec<ApiKeyInfo> {
    store
        .keys
        .iter()
        .map(|entry| ApiKeyInfo {
            id: entry.id.clone(),
            name: entry.name.clone(),
            provider: entry.provider.clone(),
            model: display_model(entry),
        })
        .collect()
}

pub fn load_api_key() -> Result<Option<String>, String> {
    let store = KEY_STORE.lock().map_err(|e| e.to_string())?;
    let Some(active_id) = store.active_id.as_ref() else {
        return Ok(None);
    };
    Ok(store
        .keys
        .iter()
        .find(|entry| &entry.id == active_id)
        .map(|entry| entry.key.clone())
        .filter(|key| !key.trim().is_empty()))
}

pub fn has_api_key() -> bool {
    matches!(load_api_key(), Ok(Some(key)) if !key.trim().is_empty())
}

#[tauri::command]
pub fn add_api_key(
    name: String,
    api_key: String,
    provider: String,
    model: String,
    base_url: String,
) -> Result<SettingsStateCore, String> {
    let name = name.trim();
    let key = api_key.trim();
    let provider = provider.trim().to_ascii_lowercase();
    let model = model.trim().to_string();
    let base_url = base_url.trim().trim_end_matches('/').to_string();
    if name.is_empty() {
        return Err("err:name".into());
    }
    if key.len() < 8 {
        return Err("err:key".into());
    }
    if !is_provider(&provider) {
        return Err("err:provider".into());
    }
    if provider == "custom" && !base_url.starts_with("https://") {
        return Err("err:base_url".into());
    }
    if model.is_empty() && provider != "gemini" {
        return Err("err:model".into());
    }

    {
        let mut store = KEY_STORE.lock().map_err(|e| e.to_string())?;
        if store
            .keys
            .iter()
            .any(|entry| entry.name.eq_ignore_ascii_case(name))
        {
            return Err("err:duplicate".into());
        }
        let id = new_id();
        store.keys.push(KeyEntry {
            id: id.clone(),
            name: name.to_string(),
            key: key.to_string(),
            provider,
            model,
            base_url,
        });
        if store.active_id.is_none() {
            store.active_id = Some(id);
        }
    }

    persist_keys()?;
    snapshot_keys()
}

#[tauri::command]
pub fn select_api_key(id: String) -> Result<SettingsStateCore, String> {
    {
        let mut store = KEY_STORE.lock().map_err(|e| e.to_string())?;
        if !store.keys.iter().any(|entry| entry.id == id) {
            return Err("err:not_found".into());
        }
        store.active_id = Some(id);
    }
    persist_keys()?;
    snapshot_keys()
}

#[tauri::command]
pub fn remove_api_key(id: String) -> Result<SettingsStateCore, String> {
    {
        let mut store = KEY_STORE.lock().map_err(|e| e.to_string())?;
        store.keys.retain(|entry| entry.id != id);
        if store.active_id.as_deref() == Some(id.as_str()) {
            store.active_id = store.keys.first().map(|entry| entry.id.clone());
        }
    }
    persist_keys()?;
    snapshot_keys()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsStateCore {
    has_api_key: bool,
    active_key_id: Option<String>,
    api_keys: Vec<ApiKeyInfo>,
}

fn snapshot_keys() -> Result<SettingsStateCore, String> {
    let store = KEY_STORE.lock().map_err(|e| e.to_string())?;
    Ok(SettingsStateCore {
        has_api_key: store.active_id.as_ref().is_some_and(|id| {
            store
                .keys
                .iter()
                .any(|entry| &entry.id == id && !entry.key.trim().is_empty())
        }),
        active_key_id: store.active_id.clone(),
        api_keys: key_summaries(&store),
    })
}

#[tauri::command]
pub fn get_settings_state(app: AppHandle) -> Result<SettingsState, String> {
    let prefs = PREFS.lock().map_err(|e| e.to_string())?.clone();
    let keys = snapshot_keys()?;
    Ok(SettingsState {
        has_api_key: keys.has_api_key,
        active_key_id: keys.active_key_id,
        api_keys: keys.api_keys,
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        close_to_tray: prefs.close_to_tray,
        start_minimized: prefs.start_minimized,
        hotkey_label: format_hotkey(prefs.hotkey),
        can_autostart: !is_dev_executable(),
        ui_language: prefs.ui_language.clone(),
        native_language: prefs.native_language.clone(),
        foreign_language: prefs.foreign_language.clone(),
        prompt_rules: visible_rules(&prefs),
        rules_customized: prefs.rules_customized,
        timeout_secs: prefs.timeout_secs,
        max_output_tokens: prefs.max_output_tokens,
        fallback_models: prefs.fallback_models,
    })
}

fn is_dev_executable() -> bool {
    if cfg!(dev) {
        return true;
    }
    std::env::current_exe()
        .ok()
        .and_then(|path| path.to_str().map(str::to_owned))
        .is_some_and(|path| {
            path.replace('/', "\\")
                .to_ascii_lowercase()
                .contains("\\target\\debug\\")
        })
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let autolaunch = app.autolaunch();
    if enabled {
        if is_dev_executable() {
            return Err(
                "Geliştirme sürümü Windows açılışında çalışmaz. Kurulu uygulamadan aç."
                    .into(),
            );
        }
        autolaunch.enable().map_err(|e| e.to_string())
    } else {
        autolaunch.disable().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn set_close_to_tray(enabled: bool) -> Result<(), String> {
    {
        let mut prefs = PREFS.lock().map_err(|e| e.to_string())?;
        prefs.close_to_tray = enabled;
    }
    persist_prefs()
}

#[tauri::command]
pub fn set_start_minimized(enabled: bool) -> Result<(), String> {
    {
        let mut prefs = PREFS.lock().map_err(|e| e.to_string())?;
        prefs.start_minimized = enabled;
    }
    persist_prefs()
}

pub fn timeout_secs() -> u64 {
    PREFS
        .lock()
        .map(|prefs| prefs.timeout_secs.clamp(5, 120))
        .unwrap_or(20)
}

pub fn max_output_tokens() -> u32 {
    PREFS
        .lock()
        .map(|prefs| prefs.max_output_tokens.clamp(128, 8192))
        .unwrap_or(2048)
}

pub fn ui_language() -> String {
    PREFS
        .lock()
        .map(|prefs| prefs.ui_language.clone())
        .unwrap_or_else(|_| default_ui_lang())
}

#[derive(Clone)]
pub struct ActiveProfile {
    pub name: String,
    pub provider: String,
    pub model: String,
    pub base_url: String,
    pub key: String,
    pub fallback_models: bool,
}

pub fn active_profile() -> Result<ActiveProfile, String> {
    let store = KEY_STORE.lock().map_err(|e| e.to_string())?;
    let prefs = PREFS.lock().map_err(|e| e.to_string())?;
    let Some(active_id) = store.active_id.as_ref() else {
        return Err("err:no_key".into());
    };
    let Some(entry) = store.keys.iter().find(|entry| &entry.id == active_id) else {
        return Err("err:no_key".into());
    };
    if entry.key.trim().is_empty() {
        return Err("err:no_key".into());
    }
    let provider = canonical_provider(&entry.provider);
    Ok(ActiveProfile {
        name: entry.name.clone(),
        model: resolved_model(entry),
        base_url: resolved_base_url(&provider, &entry.base_url),
        key: entry.key.clone(),
        provider,
        fallback_models: prefs.fallback_models,
    })
}

pub fn prompts_for(source: &str) -> (String, String) {
    let prefs = PREFS.lock().map(|prefs| prefs.clone()).unwrap_or_else(|_| empty_prefs());
    let system = apply_language_tokens(&join_rules(&active_rules(&prefs)), &prefs);
    let user = format!(
        "Translate. If the source is {native}, write {foreign}. If the source is not {native}, write {native}. Keep person (you/I/he). Output only the translation.\n\n{source}",
        native = language_name(&prefs.native_language),
        foreign = language_name(&prefs.foreign_language),
    );
    (system, user)
}

pub fn record_traffic(entry: TrafficEntry) {
    let mut entry = entry;
    entry.id = new_id();
    entry.source = clip(&entry.source, 8000);
    entry.system_prompt = clip(&entry.system_prompt, 12000);
    entry.user_message = clip(&entry.user_message, 12000);
    entry.response = clip(&entry.response, 12000);
    entry.error = clip(&entry.error, 2000);
    if let Ok(mut traffic) = TRAFFIC.lock() {
        traffic.insert(0, entry);
        traffic.truncate(40);
    }
    let _ = persist_traffic();
}

#[tauri::command]
pub fn get_traffic() -> Result<Vec<TrafficEntry>, String> {
    TRAFFIC.lock().map(|traffic| traffic.clone()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_traffic() -> Result<(), String> {
    if let Ok(mut traffic) = TRAFFIC.lock() {
        traffic.clear();
    }
    persist_traffic()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPrefsUpdate {
    ui_language: String,
    native_language: String,
    foreign_language: String,
    prompt_rules: Vec<String>,
    rules_customized: bool,
    timeout_secs: u64,
    max_output_tokens: u32,
    fallback_models: bool,
}

#[tauri::command]
pub fn set_app_prefs(app: AppHandle, update: AppPrefsUpdate) -> Result<(), String> {
    if !is_lang(&update.ui_language) || !is_lang(&update.native_language) || !is_lang(&update.foreign_language)
    {
        return Err("err:language".into());
    }
    if update.native_language == update.foreign_language {
        return Err("err:same_language".into());
    }
    if !(5..=120).contains(&update.timeout_secs) {
        return Err("err:timeout_range".into());
    }
    if !(128..=8192).contains(&update.max_output_tokens) {
        return Err("err:tokens".into());
    }
    {
        let mut prefs = PREFS.lock().map_err(|e| e.to_string())?;
        prefs.ui_language = update.ui_language;
        prefs.native_language = update.native_language;
        prefs.foreign_language = update.foreign_language;
        prefs.rules_customized = update.rules_customized;
        prefs.prompt_rules = if update.rules_customized {
            clean_rules(&update.prompt_rules)
        } else {
            Vec::new()
        };
        prefs.custom_prompt.clear();
        prefs.timeout_secs = update.timeout_secs;
        prefs.max_output_tokens = update.max_output_tokens;
        prefs.fallback_models = update.fallback_models;
    }
    persist_prefs()?;
    crate::tray::refresh(&app);
    Ok(())
}

fn persist_traffic() -> Result<(), String> {
    let traffic = TRAFFIC.lock().map_err(|e| e.to_string())?.clone();
    let path = TRAFFIC_FILE
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "err:failed".to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_vec_pretty(&traffic).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn normalize_prefs(prefs: &mut Prefs) {
    if !is_lang(&prefs.ui_language) {
        prefs.ui_language = default_ui_lang();
    }
    if !is_lang(&prefs.native_language) {
        prefs.native_language = default_native();
    }
    if !is_lang(&prefs.foreign_language) {
        prefs.foreign_language = default_foreign();
    }
    if prefs.native_language == prefs.foreign_language {
        prefs.foreign_language = if prefs.native_language == "en" {
            "tr".into()
        } else {
            "en".into()
        };
    }
    if prefs.timeout_secs == 0 {
        prefs.timeout_secs = default_timeout();
    }
    prefs.timeout_secs = prefs.timeout_secs.clamp(5, 120);
    if prefs.max_output_tokens == 0 {
        prefs.max_output_tokens = default_tokens();
    }
    prefs.max_output_tokens = prefs.max_output_tokens.clamp(128, 8192);
    if !prefs.rules_customized && prefs.prompt_rules.is_empty() && !prefs.custom_prompt.trim().is_empty() {
        prefs.prompt_rules = split_legacy_prompt(&prefs.custom_prompt);
        prefs.rules_customized = true;
        prefs.custom_prompt.clear();
    }
    prefs.prompt_rules = clean_rules(&prefs.prompt_rules);
    prefs.custom_prompt = clip(&prefs.custom_prompt, 12000);
}

fn is_lang(code: &str) -> bool {
    matches!(
        code,
        "tr" | "en" | "de" | "fr" | "es" | "it" | "pt" | "ru" | "ar" | "ja" | "zh" | "ko"
    )
}

fn is_provider(code: &str) -> bool {
    matches!(code, "gemini" | "openai" | "groq" | "openrouter" | "custom")
}

fn canonical_provider(code: &str) -> String {
    let code = code.trim().to_ascii_lowercase();
    if is_provider(&code) {
        code
    } else {
        default_provider()
    }
}

fn display_model(entry: &KeyEntry) -> String {
    resolved_model(entry)
}

fn resolved_model(entry: &KeyEntry) -> String {
    if !entry.model.trim().is_empty() {
        return entry.model.trim().to_string();
    }
    match canonical_provider(&entry.provider).as_str() {
        "openai" => "gpt-4o-mini".into(),
        "groq" => "llama-3.3-70b-versatile".into(),
        "openrouter" => "google/gemini-2.5-flash".into(),
        "custom" => String::new(),
        _ => "gemini-3.1-flash-lite".into(),
    }
}

fn resolved_base_url(provider: &str, base_url: &str) -> String {
    if !base_url.trim().is_empty() {
        return base_url.trim().trim_end_matches('/').to_string();
    }
    match provider {
        "openai" => "https://api.openai.com/v1".into(),
        "groq" => "https://api.groq.com/openai/v1".into(),
        "openrouter" => "https://openrouter.ai/api/v1".into(),
        _ => String::new(),
    }
}

pub fn language_name(code: &str) -> &'static str {
    match code {
        "tr" => "Turkish",
        "en" => "English",
        "de" => "German",
        "fr" => "French",
        "es" => "Spanish",
        "it" => "Italian",
        "pt" => "Portuguese",
        "ru" => "Russian",
        "ar" => "Arabic",
        "ja" => "Japanese",
        "zh" => "Chinese",
        "ko" => "Korean",
        _ => "Turkish",
    }
}

fn apply_language_tokens(template: &str, prefs: &Prefs) -> String {
    template
        .replace(
            "{native}",
            localized_name(&prefs.native_language, &prefs.ui_language),
        )
        .replace(
            "{foreign}",
            localized_name(&prefs.foreign_language, &prefs.ui_language),
        )
}

fn active_rules(prefs: &Prefs) -> Vec<String> {
    let defaults = default_rules(
        &prefs.ui_language,
        &prefs.native_language,
        &prefs.foreign_language,
    );
    let head = defaults
        .first()
        .cloned()
        .unwrap_or_else(|| "Translate into the selected language.".into());
    let mut rules = vec![head];
    if prefs.rules_customized {
        rules.extend(without_target_rule(&clean_rules(&prefs.prompt_rules)));
    } else {
        rules.extend(defaults.into_iter().skip(1));
    }
    rules
}

fn without_target_rule(rules: &[String]) -> Vec<String> {
    let mut rules = rules.to_vec();
    if rules.first().is_some_and(|rule| is_target_rule(rule)) {
        rules.remove(0);
    }
    rules
}

fn is_target_rule(text: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "Hedef dil:",
        "Zielsprache:",
        "Langue cible",
        "Idioma de destino:",
        "Lingua di arrivo:",
        "Língua de destino:",
        "Язык перевода:",
        "لغة الهدف:",
        "訳し先:",
        "目标语言：",
        "목표 언어:",
        "Target language:",
    ];
    let text = text.trim();
    PREFIXES.iter().any(|prefix| text.starts_with(prefix))
}

fn visible_rules(prefs: &Prefs) -> Vec<String> {
    active_rules(prefs)
}

fn clean_rules(rules: &[String]) -> Vec<String> {
    rules
        .iter()
        .map(|rule| clip(rule.trim(), 2000))
        .filter(|rule| !rule.is_empty())
        .take(80)
        .collect()
}

fn join_rules(rules: &[String]) -> String {
    rules
        .iter()
        .enumerate()
        .map(|(index, rule)| format!("{}. {}", index + 1, rule.trim()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn split_legacy_prompt(text: &str) -> Vec<String> {
    let mut rules = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(body) = numbered_body(trimmed) {
            if let Some(prev) = current.take() {
                if !prev.trim().is_empty() {
                    rules.push(prev.trim().to_string());
                }
            }
            current = Some(body);
        } else if let Some(cur) = current.as_mut() {
            let extra = trimmed.trim_start_matches(['-', '•']).trim();
            if !extra.is_empty() {
                cur.push(' ');
                cur.push_str(extra);
            }
        }
    }
    if let Some(prev) = current {
        if !prev.trim().is_empty() {
            rules.push(prev.trim().to_string());
        }
    }
    if rules.is_empty() {
        vec![text.trim().to_string()]
    } else {
        rules
    }
}

fn numbered_body(line: &str) -> Option<String> {
    let mut chars = line.chars().peekable();
    let mut saw_digit = false;
    while chars.peek().is_some_and(|c| c.is_ascii_digit()) {
        saw_digit = true;
        chars.next();
    }
    if !saw_digit || chars.next() != Some('.') {
        return None;
    }
    let rest: String = chars.collect();
    let rest = rest.trim();
    if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    }
}

#[tauri::command]
pub fn preview_prompt_rules(
    ui_language: String,
    native_language: String,
    foreign_language: String,
) -> Result<Vec<String>, String> {
    if !is_lang(&ui_language) || !is_lang(&native_language) || !is_lang(&foreign_language) {
        return Err("err:language".into());
    }
    Ok(default_rules(&ui_language, &native_language, &foreign_language))
}

fn default_rules(ui: &str, native_code: &str, foreign_code: &str) -> Vec<String> {
    let native = localized_name(native_code, ui);
    let foreign = localized_name(foreign_code, ui);
    let english_person = native_code == "en" || foreign_code == "en";
    rule_templates(ui, english_person)
        .into_iter()
        .map(|rule| rule.replace("{native}", native).replace("{foreign}", foreign))
        .collect()
}

fn localized_name(code: &str, ui: &str) -> &'static str {
    const ORDER: [&str; 12] = [
        "tr", "en", "de", "fr", "es", "it", "pt", "ru", "ar", "ja", "zh", "ko",
    ];
    let names = match ui {
        "tr" => [
            "Türkçe", "İngilizce", "Almanca", "Fransızca", "İspanyolca", "İtalyanca", "Portekizce",
            "Rusça", "Arapça", "Japonca", "Çince", "Korece",
        ],
        "de" => [
            "Türkisch", "Englisch", "Deutsch", "Französisch", "Spanisch", "Italienisch",
            "Portugiesisch", "Russisch", "Arabisch", "Japanisch", "Chinesisch", "Koreanisch",
        ],
        "fr" => [
            "turc", "anglais", "allemand", "français", "espagnol", "italien", "portugais", "russe",
            "arabe", "japonais", "chinois", "coréen",
        ],
        "es" => [
            "turco", "inglés", "alemán", "francés", "español", "italiano", "portugués", "ruso",
            "árabe", "japonés", "chino", "coreano",
        ],
        "it" => [
            "turco", "inglese", "tedesco", "francese", "spagnolo", "italiano", "portoghese", "russo",
            "arabo", "giapponese", "cinese", "coreano",
        ],
        "pt" => [
            "turco", "inglês", "alemão", "francês", "espanhol", "italiano", "português", "russo",
            "árabe", "japonês", "chinês", "coreano",
        ],
        "ru" => [
            "турецкий", "английский", "немецкий", "французский", "испанский", "итальянский",
            "португальский", "русский", "арабский", "японский", "китайский", "корейский",
        ],
        "ar" => [
            "التركية", "الإنجليزية", "الألمانية", "الفرنسية", "الإسبانية", "الإيطالية",
            "البرتغالية", "الروسية", "العربية", "اليابانية", "الصينية", "الكورية",
        ],
        "ja" => [
            "トルコ語", "英語", "ドイツ語", "フランス語", "スペイン語", "イタリア語", "ポルトガル語",
            "ロシア語", "アラビア語", "日本語", "中国語", "韓国語",
        ],
        "zh" => [
            "土耳其语", "英语", "德语", "法语", "西班牙语", "意大利语", "葡萄牙语", "俄语", "阿拉伯语",
            "日语", "中文", "韩语",
        ],
        "ko" => [
            "터키어", "영어", "독일어", "프랑스어", "스페인어", "이탈리아어", "포르투갈어", "러시아어",
            "아랍어", "일본어", "중국어", "한국어",
        ],
        _ => [
            "Turkish", "English", "German", "French", "Spanish", "Italian", "Portuguese", "Russian",
            "Arabic", "Japanese", "Chinese", "Korean",
        ],
    };
    ORDER
        .iter()
        .position(|item| *item == code)
        .map(|index| names[index])
        .unwrap_or(names[0])
}

fn rule_templates(ui: &str, english_person: bool) -> Vec<String> {
    let person = person_rule(ui, english_person);
    let (target, only, natural, quotes, quote_kinds, structure, slang, final_only) = match ui {
        "tr" => (
            "Hedef dil: Kaynak {native} ise çıktı {foreign} olur. Kaynak {native} değilse çıktı her zaman {native} olur. Başka hiçbir dile çevirme.",
            "Sadece çeviriyi yaz. Açıklama, yorum, not, alternatif çeviri veya gereksiz hiçbir şey ekleme.",
            "Çeviri gündelik, doğal ve sade insan konuşması gibi olsun. Kelimesi kelimesine mekanik çeviri yapma; anlamı ve konuşma tarzını koru.",
            "Çeviri içinde tırnak içine alınmış terimleri ASLA çevirme. Aynı şekilde bırak.",
            "Tırnak içindeki ifade özel bir isim, teknik terim, değişken adı veya özellik adı olsa bile değiştirme.",
            "Çevirinin doğal olması için gerektiğinde cümle yapısını değiştirebilirsin, ancak anlamı ve şahsı değiştirme.",
            "Küfür, argo, günlük konuşma veya samimi ifadeler varsa bunları sansürleme; hedef dile doğal şekilde aktar.",
            "Bana yalnızca nihai çeviriyi gönder.",
        ),
        "de" => (
            "Zielsprache: Ist die Quelle {native}, ist die Ausgabe {foreign}. Ist die Quelle nicht {native}, ist die Ausgabe immer {native}. Übersetze in keine andere Sprache.",
            "Schreib nur die Übersetzung. Keine Erklärung, keinen Kommentar, keine Alternative.",
            "Die Übersetzung soll wie alltägliche gesprochene Sprache klingen. Nicht Wort für Wort. Bedeutung und Ton bleiben.",
            "Begriffe in Anführungszeichen niemals übersetzen. Sie bleiben gleich.",
            "Auch Namen, Fachbegriffe, Variablen oder Funktionsnamen in Anführungszeichen bleiben unverändert.",
            "Du darfst den Satzbau ändern, damit es natürlich klingt, aber nicht die Bedeutung oder die Person.",
            "Flüche, Slang und lockere Rede nicht zensieren. Natürlich in die Zielsprache übertragen.",
            "Schick nur die fertige Übersetzung.",
        ),
        "fr" => (
            "Langue cible : si la source est en {native}, la sortie est en {foreign}. Si la source n'est pas en {native}, la sortie est toujours en {native}. Ne traduis vers aucune autre langue.",
            "Écris seulement la traduction. Pas d'explication, de note ou d'alternative.",
            "La traduction doit sonner comme une parole quotidienne. Pas de mot à mot. Garde le sens et le ton.",
            "Ne traduis jamais les termes entre guillemets. Laisse-les tels quels.",
            "Même un nom, un terme technique, une variable ou un nom de fonction entre guillemets reste inchangé.",
            "Tu peux changer la structure de la phrase pour que ce soit naturel, sans changer le sens ni la personne.",
            "S'il y a des jurons, de l'argot ou un ton familier, ne censure pas. Transmets-les naturellement.",
            "Envoie seulement la traduction finale.",
        ),
        "es" => (
            "Idioma de destino: si el texto está en {native}, la salida es {foreign}. Si no está en {native}, la salida siempre es {native}. No traduzcas a ningún otro idioma.",
            "Escribe solo la traducción. Sin explicaciones, notas ni alternativas.",
            "La traducción debe sonar a habla cotidiana. No traduzcas palabra por palabra. Conserva el sentido y el tono.",
            "No traduzcas nunca los términos entre comillas. Déjalos igual.",
            "Aunque lo entrecomillado sea un nombre, un término técnico, una variable o una función, no lo cambies.",
            "Puedes cambiar la estructura para que suene natural, sin cambiar el sentido ni la persona.",
            "Si hay insultos, jerga o un tono cercano, no los censures. Pásalos con naturalidad.",
            "Envía solo la traducción final.",
        ),
        "it" => (
            "Lingua di arrivo: se la fonte è {native}, l'uscita è {foreign}. Se la fonte non è {native}, l'uscita è sempre {native}. Non tradurre in nessun'altra lingua.",
            "Scrivi solo la traduzione. Niente spiegazioni, note o alternative.",
            "La traduzione deve suonare come il parlato di tutti i giorni. Non parola per parola. Tieni senso e tono.",
            "Non tradurre mai i termini tra virgolette. Lasciali uguali.",
            "Anche un nome, un termine tecnico, una variabile o una funzione tra virgolette resta invariato.",
            "Puoi cambiare la struttura della frase perché suoni naturale, senza cambiare senso o persona.",
            "Se ci sono parolacce, gergo o un tono informale, non censurare. Rendili in modo naturale.",
            "Manda solo la traduzione finale.",
        ),
        "pt" => (
            "Língua de destino: se a origem está em {native}, a saída é {foreign}. Se a origem não está em {native}, a saída é sempre {native}. Não traduzas para nenhuma outra língua.",
            "Escreve só a tradução. Sem explicações, notas ou alternativas.",
            "A tradução deve soar a fala do dia a dia. Não traduzas palavra a palavra. Mantém o sentido e o tom.",
            "Nunca traduzas termos entre aspas. Deixa-os iguais.",
            "Mesmo que o texto entre aspas seja um nome, um termo técnico, uma variável ou uma função, não o mudes.",
            "Podes mudar a estrutura da frase para soar natural, sem mudar o sentido nem a pessoa.",
            "Se houver palavrões, calão ou um tom informal, não censures. Passa-os com naturalidade.",
            "Envia só a tradução final.",
        ),
        "ru" => (
            "Язык перевода: если исходный текст на языке «{native}», ответ на языке «{foreign}». Если исходный текст не на языке «{native}», ответ всегда на языке «{native}». Не переводи ни на какой другой язык.",
            "Пиши только перевод. Без пояснений, заметок и вариантов.",
            "Перевод должен звучать как обычная речь. Не переводи слово в слово. Сохраняй смысл и тон.",
            "Термины в кавычках никогда не переводи. Оставляй как есть.",
            "Даже имя, термин, переменная или название функции в кавычках остаются без изменений.",
            "Можно менять строй фразы, чтобы звучало естественно, но не смысл и не лицо.",
            "Мат, сленг и разговорный тон не цензурируй. Передавай естественно.",
            "Пришли только готовый перевод.",
        ),
        "ar" => (
            "لغة الهدف: إذا كان المصدر بـ{native} فالناتج بـ{foreign}. إذا لم يكن المصدر بـ{native} فالناتج دائمًا بـ{native}. لا تترجم إلى أي لغة أخرى.",
            "اكتب الترجمة فقط. بلا شرح أو ملاحظة أو بديل.",
            "لتكن الترجمة كلامًا يوميًا طبيعيًا. لا تترجم كلمة بكلمة. حافظ على المعنى والأسلوب.",
            "لا تترجم أبدًا ما بين علامتي التنصيص. اتركه كما هو.",
            "حتى لو كان ما بين التنصيص اسمًا أو مصطلحًا أو متغيرًا أو اسم ميزة، لا تغيّره.",
            "يمكنك تغيير تركيب الجملة ليبدو طبيعيًا، دون تغيير المعنى أو الضمير.",
            "إذا وُجد سباب أو عامية أو حديث حميمي فلا تراقبه. انقله بطبيعته إلى اللغة الهدف.",
            "أرسل الترجمة النهائية فقط.",
        ),
        "ja" => (
            "訳し先: 原文が{native}なら出力は{foreign}。原文が{native}でなければ出力は常に{native}。それ以外の言語には訳さない。",
            "翻訳だけを書く。説明、注、別案は入れない。",
            "日常の話し言葉にする。一語一語の機械訳にしない。意味と口調は保つ。",
            "引用符の中の語は絶対に訳さない。そのまま残す。",
            "引用符の中が固有名詞、専門用語、変数名、機能名でも変えない。",
            "自然にするために文の形は変えてよい。意味と人称は変えない。",
            "罵り、俗語、くだけた言い方は伏せない。訳し先で自然に伝える。",
            "最終的な翻訳だけを返す。",
        ),
        "zh" => (
            "目标语言：原文是{native}时，输出为{foreign}。原文不是{native}时，输出始终为{native}。不要译成任何其他语言。",
            "只写译文。不要解释、注释或另一种译法。",
            "译文要像日常说话。不要逐字硬译。保留意思和口气。",
            "引号里的词一律不译，原样保留。",
            "引号里即使是名字、术语、变量名或功能名，也不要改。",
            "为了自然可以调整句式，但不要改意思和人称。",
            "有脏话、俚语或随便的说法时不要删掉，用目标语言自然地说出来。",
            "只发送最终译文。",
        ),
        "ko" => (
            "목표 언어: 원문이 {native}이면 결과는 {foreign}이다. 원문이 {native}가 아니면 결과는 항상 {native}이다. 다른 언어로는 번역하지 않는다.",
            "번역만 쓴다. 설명, 주석, 다른 번역은 넣지 않는다.",
            "일상 말투로 옮긴다. 단어를 하나씩 기계적으로 옮기지 않는다. 뜻과 말투는 유지한다.",
            "따옴표 안의 말은 절대 번역하지 않는다. 그대로 둔다.",
            "따옴표 안이 이름, 용어, 변수, 기능 이름이어도 바꾸지 않는다.",
            "자연스럽게 하려고 문장 구조는 바꿔도 된다. 뜻과 인칭은 바꾸지 않는다.",
            "욕, 속어, 편한 말투가 있으면 지우지 말고 목표 언어로 자연스럽게 옮긴다.",
            "최종 번역만 보낸다.",
        ),
        _ => (
            "Target language: if the source is {native}, the output is {foreign}. If the source is not {native}, the output is always {native}. Do not translate into any other language.",
            "Write only the translation. Do not add an explanation, a note, or an alternative.",
            "Sound like everyday speech. Do not translate word for word. Keep the meaning and the tone.",
            "Never translate terms inside quotation marks. Leave them as they are.",
            "Even if the quoted text is a name, a technical term, a variable, or a feature name, leave it unchanged.",
            "You may change the sentence structure so it sounds natural, but do not change the meaning or the person.",
            "If there is swearing, slang, or casual talk, do not censor it. Carry it naturally into the target language.",
            "Send only the final translation.",
        ),
    };
    vec![
        target.into(),
        only.into(),
        natural.into(),
        person,
        quotes.into(),
        quote_kinds.into(),
        structure.into(),
        slang.into(),
        final_only.into(),
    ]
}

fn person_rule(ui: &str, english_person: bool) -> String {
    match (ui, english_person) {
        ("tr", true) => "Şahıs aynen kalsın. sen = you, ben = I, o = he. Çıktı İngilizceyse they/them veya he/she yazma.".into(),
        ("tr", false) => "Şahıs aynen kalsın. Konuşulan kişi, konuşan kişi ve üçüncü kişi yer değiştirmesin.".into(),
        ("de", true) => "Die Person bleibt. du = you, ich = I, er = he. Wenn die Ausgabe Englisch ist, schreibe nicht they/them oder he/she.".into(),
        ("de", false) => "Die Person bleibt. Angesprochene Person, sprechende Person und eine dritte Person tauschen nicht die Rolle.".into(),
        ("fr", true) => "La personne reste. tu = you, je = I, il = he. Si la sortie est en anglais, n'écris pas they/them ni he/she.".into(),
        ("fr", false) => "La personne reste. La personne à qui l'on parle, celle qui parle et une troisième personne ne changent pas de rôle.".into(),
        ("es", true) => "La persona se mantiene. tú = you, yo = I, él = he. Si la salida es inglés, no escribas they/them ni he/she.".into(),
        ("es", false) => "La persona se mantiene. Quien escucha, quien habla y una tercera persona no cambian de papel.".into(),
        ("it", true) => "La persona resta. tu = you, io = I, lui = he. Se l'uscita è inglese, non scrivere they/them o he/she.".into(),
        ("it", false) => "La persona resta. Chi ascolta, chi parla e una terza persona non si scambiano.".into(),
        ("pt", true) => "A pessoa mantém-se. tu = you, eu = I, ele = he. Se a saída for inglês, não escrevas they/them nem he/she.".into(),
        ("pt", false) => "A pessoa mantém-se. Quem ouve, quem fala e uma terceira pessoa não trocam de papel.".into(),
        ("ru", true) => "Лицо сохраняется. ты = you, я = I, он = he. Если ответ на английском, не пиши they/them или he/she.".into(),
        ("ru", false) => "Лицо сохраняется. Собеседник, говорящий и третье лицо не меняются местами.".into(),
        ("ar", true) => "الضمير يبقى. أنت = you، أنا = I، هو = he. إذا كان الناتج إنجليزيًا فلا تكتب they/them أو he/she.".into(),
        ("ar", false) => "الضمير يبقى. المخاطَب والمتكلم والغائب لا يتبادلون الأدوار.".into(),
        ("ja", true) => "人称はそのまま。あなた = you、私 = I、彼 = he。出力が英語なら they/them や he/she は書かない。".into(),
        ("ja", false) => "人称はそのまま。話しかけられている人、話している人、第三者を入れ替えない。".into(),
        ("zh", true) => "人称保持不变。你 = you，我 = I，他 = he。如果输出是英语，不要写 they/them 或 he/she。".into(),
        ("zh", false) => "人称保持不变。听话的人、说话的人和第三者不要对调。".into(),
        ("ko", true) => "인칭은 그대로 둔다. 너 = you, 나 = I, 그 = he. 결과가 영어면 they/them 이나 he/she 를 쓰지 않는다.".into(),
        ("ko", false) => "인칭은 그대로 둔다. 듣는 사람, 말하는 사람, 제삼자를 바꾸지 않는다.".into(),
        (_, true) => "Keep the person. you stays you, I stays I, he stays he. If the output is English, do not write they/them or he/she.".into(),
        (_, false) => "Keep the person. Do not swap the listener, the speaker, or a third person.".into(),
    }
}

fn clip(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}

fn protect(data: &[u8]) -> Result<Vec<u8>, String> {
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Cryptography::{
        CryptProtectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptProtectData(
            &input,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if ok == 0 {
        return Err("Anahtar şifrelenemedi.".into());
    }

    let bytes = unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }
        .to_vec();
    unsafe {
        LocalFree(output.pbData as *mut core::ffi::c_void);
    }
    Ok(bytes)
}

fn read_protected_string(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let plain = unprotect(&bytes)?;
    String::from_utf8(plain).map_err(|_| "Anahtar dosyası bozuk.".to_string())
}

fn unprotect(data: &[u8]) -> Result<Vec<u8>, String> {
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Cryptography::{
        CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptUnprotectData(
            &input,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if ok == 0 {
        return Err("Anahtar okunamadı.".into());
    }

    let bytes = unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }
        .to_vec();
    unsafe {
        LocalFree(output.pbData as *mut core::ffi::c_void);
    }
    Ok(bytes)
}
