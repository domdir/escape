#![windows_subsystem = "windows"]

use std::{mem, ptr};
use winapi::{
    ctypes::c_int,
    shared::minwindef::{DWORD, LPARAM, LRESULT, WORD, WPARAM},
    um::winuser::{
        CallNextHookEx, GetMessageA, INPUT_u, SendInput, SetWindowsHookExA, INPUT, INPUT_KEYBOARD,
        KBDLLHOOKSTRUCT, KEYEVENTF_KEYUP, MSG, VK_CAPITAL, VK_ESCAPE, WH_KEYBOARD_LL, WM_KEYUP,
        WM_SYSKEYUP,
    },
};

fn main() {
    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), ptr::null_mut(), 0);
        assert_ne!(hook, ptr::null_mut(), "Unable to set hook");
        let mut msg = MSG::default();
        GetMessageA(&mut msg, ptr::null_mut(), 0, 0);
    }
}

unsafe extern "system" fn callback(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let param = l_param as *const KBDLLHOOKSTRUCT;

    if (*param).vkCode == VK_CAPITAL as DWORD {
        let mut u = INPUT_u::default();

        let mut keybdinput = u.ki_mut();
        keybdinput.wVk = VK_ESCAPE as WORD;
        keybdinput.dwFlags = if w_param == WM_KEYUP as WPARAM || w_param == WM_SYSKEYUP as WPARAM {
            KEYEVENTF_KEYUP
        } else {
            0
        };

        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u,
        };

        SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);

        1
    } else {
        CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
    }
}
