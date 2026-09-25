const DOUBLE_TAP_MS: u64 = 500;
const COOLDOWN_MS: u64 = 600;
const LLKHF_INJECTED: u32 = 0x10;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::AppHandle;
use windows_sys::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetMessageW, SetWindowsHookExW,
    TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::session;
use crate::settings;

enum HookEvent {
    Translate(isize),
}

static TX: OnceLock<Sender<HookEvent>> = OnceLock::new();
static LAST_TAP_MS: AtomicU64 = AtomicU64::new(0);
static LAST_TRIGGER_MS: AtomicU64 = AtomicU64::new(0);
static TRIGGER_HELD: AtomicBool = AtomicBool::new(false);
static CTRL: AtomicBool = AtomicBool::new(false);
static ALT: AtomicBool = AtomicBool::new(false);
static SHIFT: AtomicBool = AtomicBool::new(false);
static WIN: AtomicBool = AtomicBool::new(false);

pub fn start(app: AppHandle) {
    let (tx, rx) = mpsc::channel::<HookEvent>();
    let _ = TX.set(tx);

    std::thread::Builder::new()
        .name("idkeng-translate".into())
        .spawn(move || {
            for event in rx {
                match event {
                    HookEvent::Translate(hwnd) => session::on_shortcut(&app, hwnd),
                }
            }
        })
        .expect("çeviri iş parçacığı açılamadı");

    std::thread::Builder::new()
        .name("idkeng-hook".into())
        .spawn(|| unsafe { message_loop() })
        .expect("klavye kancası açılamadı");
}

pub fn reset_pending() {
    LAST_TAP_MS.store(0, Ordering::Relaxed);
    TRIGGER_HELD.store(false, Ordering::Relaxed);
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn set_modifier(vk: u32, down: bool) -> bool {
    match vk {
        0x11 | 0xA2 | 0xA3 => {
            CTRL.store(down, Ordering::Relaxed);
            true
        }
        0x12 | 0xA4 | 0xA5 => {
            ALT.store(down, Ordering::Relaxed);
            true
        }
        0x10 | 0xA0 | 0xA1 => {
            SHIFT.store(down, Ordering::Relaxed);
            true
        }
        0x5B | 0x5C => {
            WIN.store(down, Ordering::Relaxed);
            true
        }
        _ => false,
    }
}

fn mods_match() -> bool {
    let hotkey = settings::hotkey();
    CTRL.load(Ordering::Relaxed) == hotkey.ctrl
        && ALT.load(Ordering::Relaxed) == hotkey.alt
        && SHIFT.load(Ordering::Relaxed) == hotkey.shift
        && WIN.load(Ordering::Relaxed) == hotkey.win
}

fn is_down_msg(message: u32) -> bool {
    message == WM_KEYDOWN || message == WM_SYSKEYDOWN
}

fn is_up_msg(message: u32) -> bool {
    message == WM_KEYUP || message == WM_SYSKEYUP
}

fn send_translate() {
    let hwnd = unsafe { GetForegroundWindow() as isize };
    if let Some(tx) = TX.get() {
        let _ = tx.send(HookEvent::Translate(hwnd));
    }
}

fn handle_trigger(down: bool, up: bool, vk: u32) -> bool {
    let hotkey = settings::hotkey();
    if up {
        if vk == hotkey.vk {
            TRIGGER_HELD.store(false, Ordering::Relaxed);
        }
        return false;
    }
    if !down {
        return false;
    }
    if vk != hotkey.vk {
        LAST_TAP_MS.store(0, Ordering::Relaxed);
        return false;
    }
    if TRIGGER_HELD.swap(true, Ordering::Relaxed) {
        return false;
    }
    if !mods_match() {
        LAST_TAP_MS.store(0, Ordering::Relaxed);
        return false;
    }

    let t = now_ms();
    let last_trigger = LAST_TRIGGER_MS.load(Ordering::Relaxed);
    if t.saturating_sub(last_trigger) < COOLDOWN_MS {
        return false;
    }

    if hotkey.taps <= 1 {
        LAST_TRIGGER_MS.store(t, Ordering::Relaxed);
        LAST_TAP_MS.store(0, Ordering::Relaxed);
        send_translate();
        return true;
    }

    let last = LAST_TAP_MS.load(Ordering::Relaxed);
    if last != 0 && t.saturating_sub(last) <= DOUBLE_TAP_MS {
        LAST_TAP_MS.store(0, Ordering::Relaxed);
        LAST_TRIGGER_MS.store(t, Ordering::Relaxed);
        send_translate();
        return false;
    }

    LAST_TAP_MS.store(t, Ordering::Relaxed);
    false
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb = &*(lparam as *const KBDLLHOOKSTRUCT);
        if kb.flags & LLKHF_INJECTED == 0 {
            let message = wparam as u32;
            let down = is_down_msg(message);
            let up = is_up_msg(message);
            if down || up {
                if !set_modifier(kb.vkCode, down) && handle_trigger(down, up, kb.vkCode) {
                    return 1;
                }
            }
        }
    }

    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

unsafe fn message_loop() {
    let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), std::ptr::null_mut(), 0);
    if hook.is_null() {
        eprintln!("IDKENG: klavye kancası kurulamadı");
        return;
    }

    let mut msg = MSG {
        hwnd: std::ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    UnhookWindowsHookEx(hook);
}
