use std::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_A, VK_B, VK_C, VK_D,
    VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M, VK_N, VK_O, VK_OEM_1, VK_OEM_2,
    VK_OEM_COMMA, VK_OEM_PERIOD, VK_P, VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z,
};

use crate::keyboard::KeyCode;

pub fn generate_key_stroke(code: KeyCode) {
    let virtual_keycode = match code {
        KeyCode::Q => VK_Q,
        KeyCode::W => VK_W,
        KeyCode::E => VK_E,
        KeyCode::R => VK_R,
        KeyCode::T => VK_T,
        KeyCode::Y => VK_Y,
        KeyCode::U => VK_U,
        KeyCode::I => VK_I,
        KeyCode::O => VK_O,
        KeyCode::P => VK_P,
        KeyCode::A => VK_A,
        KeyCode::S => VK_S,
        KeyCode::D => VK_D,
        KeyCode::F => VK_F,
        KeyCode::G => VK_G,
        KeyCode::H => VK_H,
        KeyCode::J => VK_J,
        KeyCode::K => VK_K,
        KeyCode::L => VK_L,
        KeyCode::Semicolon => VK_OEM_1,
        KeyCode::Z => VK_Z,
        KeyCode::X => VK_X,
        KeyCode::C => VK_C,
        KeyCode::V => VK_V,
        KeyCode::B => VK_B,
        KeyCode::N => VK_N,
        KeyCode::M => VK_M,
        KeyCode::Comma => VK_OEM_COMMA,
        KeyCode::Dot => VK_OEM_PERIOD,
        KeyCode::Slash => VK_OEM_2,
    };
    unsafe {
        SendInput(
            &[
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: virtual_keycode,
                            ..Default::default()
                        },
                    },
                },
            ],
            size_of::<INPUT>() as i32,
        );
        SendInput(
            &[
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: virtual_keycode,
                            dwFlags: KEYEVENTF_KEYUP,
                            ..Default::default()
                        },
                    },
                },
            ],
            size_of::<INPUT>() as i32,
        );
    }
}
