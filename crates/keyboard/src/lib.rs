#![no_std]

// https://en.wikipedia.org/wiki/List_of_Unicode_characters
// https://www.scs.stanford.edu/10wi-cs140/pintos/specs/kbd/scancodes-1.html
// https://wiki.osdev.org/PS/2_Keyboard

extern crate alloc;

use alloc::collections::BTreeSet;
use lazy_static::lazy_static;
use spin::Mutex;

mod key_code;

pub use key_code::*;


lazy_static! {
    static ref KEYS_DOWN: Mutex<BTreeSet<KeyCode>> = Mutex::new(BTreeSet::default());
    static ref INFO: Mutex<KeyboardInfo> = Mutex::new(KeyboardInfo::default());
}

#[derive(Default)]
struct KeyboardInfo {
    is_caps_lock_enabled: bool,
    is_shift_down: bool,
    is_ctrl_down: bool,
    is_alt_down: bool,

    ext: ScanCodeExtension,
}

const START_EXPANDED_CODE_SPACE: u8 = 0xE0;

// Only care about < 0x58 for now.

pub fn handle_next_scan_code(value: u8) -> Option<KeyEvent> {
    let mut info = INFO.lock();

    // Extended Code Space (224/0xE0)
    if value == START_EXPANDED_CODE_SPACE {
        info.ext = ScanCodeExtension::Extended;

        None
    } else if value > 128 {
        // Release
        let key_code = KeyCode::from_scan_code(info.ext, value - 128);

        KEYS_DOWN.lock().remove(&key_code);

        match key_code {
            KeyCode::LeftShift | KeyCode::RightShift => {
                info.is_shift_down = false;
            }

            KeyCode::LeftControl => {
                info.is_ctrl_down = false;
            }

            _ => ()
        }

        Some(KeyEvent::Up(KeyInfo {
            is_caps_lock_enabled: info.is_caps_lock_enabled,
            is_shift_down: info.is_shift_down,
            is_ctrl_down: info.is_ctrl_down,
            is_alt_down: info.is_alt_down,

            is_held: false,

            char: key_code.to_byte_unicode(info.is_caps_lock_enabled ^ info.is_shift_down)? as char,
            code: key_code,
        }))
    } else {
        let key_code = KeyCode::from_scan_code(info.ext, value);

        // KeyCode::Escape => return None,
        // KeyCode::LeftCommand => return None,
        // KeyCode::LeftTab => return None,

        // KeyCode::F1 => return None,
        // KeyCode::F2 => return None,
        // KeyCode::F3 => return None,
        // KeyCode::F4 => return None,
        // KeyCode::F5 => return None,
        // KeyCode::F6 => return None,
        // KeyCode::F7 => return None,
        // KeyCode::F8 => return None,
        // KeyCode::F9 => return None,
        // KeyCode::F10 => return None,
        // KeyCode::F11 => return None,
        // KeyCode::F12 => return None,

        let is_held = !KEYS_DOWN.lock().insert(key_code);

        match key_code {
            KeyCode::LeftShift | KeyCode::RightShift => {
                info.is_shift_down = true;
            }

            KeyCode::LeftControl => {
                info.is_ctrl_down = true;
            }

            KeyCode::CapsLock => if !is_held {
                info.is_caps_lock_enabled = !info.is_caps_lock_enabled;
            }

            _ => ()
        }

        info.ext = ScanCodeExtension::Default;

        Some(KeyEvent::Down(KeyInfo {
            is_caps_lock_enabled: info.is_caps_lock_enabled,
            is_shift_down: info.is_shift_down,
            is_ctrl_down: info.is_ctrl_down,
            is_alt_down: info.is_alt_down,

            is_held,

            char: key_code.to_byte_unicode(info.is_caps_lock_enabled ^ info.is_shift_down)? as char,
            code: key_code,
        }))
    }
}

#[derive(Debug)]
pub enum KeyEvent {
    Up(KeyInfo),
    Down(KeyInfo),
}


#[derive(Debug)]
pub struct KeyInfo {
    pub is_caps_lock_enabled: bool,
    pub is_shift_down: bool,
    pub is_ctrl_down: bool,
    pub is_alt_down: bool,

    pub is_held: bool,
    pub char: char,
    pub code: KeyCode,
}