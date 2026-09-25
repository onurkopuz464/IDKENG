use std::mem::size_of;
use std::thread;
use std::time::Duration;

use windows_sys::Win32::System::DataExchange::GetClipboardSequenceNumber;
use windows_sys::Win32::System::Threading::AttachThreadInput;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_C, VK_CONTROL,
    VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT, VK_V,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow,
};

use crate::settings;

pub fn paste_to_hwnd(hwnd: isize) {
    if hwnd == 0 {
        return;
    }

    focus_hwnd(hwnd);
    thread::sleep(Duration::from_millis(40));

    let mut inputs = [
        key(VK_CONTROL, 0),
        key(VK_V, 0),
        key(VK_V, KEYEVENTF_KEYUP),
        key(VK_CONTROL, KEYEVENTF_KEYUP),
    ];
    send(&mut inputs);
}

pub fn copy_from_hwnd(hwnd: isize) -> bool {
    if hwnd == 0 {
        return false;
    }

    let seq = unsafe { GetClipboardSequenceNumber() };
    focus_hwnd(hwnd);
    thread::sleep(Duration::from_millis(40));
    send_copy_chord();

    for _ in 0..20 {
        thread::sleep(Duration::from_millis(25));
        if unsafe { GetClipboardSequenceNumber() } != seq {
            return true;
        }
    }
    false
}

fn send_copy_chord() {
    let hotkey = settings::hotkey();
    let mut inputs = Vec::new();

    if hotkey.vk != u32::from(VK_C) {
        inputs.push(key(hotkey.vk as u16, KEYEVENTF_KEYUP));
    }
    if hotkey.shift {
        inputs.push(key(VK_SHIFT, KEYEVENTF_KEYUP));
    }
    if hotkey.alt {
        inputs.push(key(VK_MENU, KEYEVENTF_KEYUP));
    }
    if hotkey.win {
        inputs.push(key(VK_LWIN, KEYEVENTF_KEYUP));
        inputs.push(key(VK_RWIN, KEYEVENTF_KEYUP));
    }

    inputs.push(key(VK_CONTROL, 0));
    inputs.push(key(VK_C, 0));
    inputs.push(key(VK_C, KEYEVENTF_KEYUP));
    inputs.push(key(VK_CONTROL, KEYEVENTF_KEYUP));
    send(&mut inputs);
}

fn focus_hwnd(hwnd: isize) {
    unsafe {
        let target = hwnd as *mut core::ffi::c_void;
        let current = GetForegroundWindow();
        let mut unused_pid = 0u32;
        let current_thread = GetWindowThreadProcessId(current, &mut unused_pid);
        let target_thread = GetWindowThreadProcessId(target, &mut unused_pid);

        if current_thread != target_thread {
            AttachThreadInput(current_thread, target_thread, 1);
        }
        SetForegroundWindow(target);
        if current_thread != target_thread {
            AttachThreadInput(current_thread, target_thread, 0);
        }
    }
}

fn send(inputs: &mut [INPUT]) {
    if inputs.is_empty() {
        return;
    }
    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        );
    }
}

fn key(vk: u16, flags: u32) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}
