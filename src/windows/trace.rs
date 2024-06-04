use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Mutex,
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{
            VIRTUAL_KEY, VK_A, VK_B, VK_C, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L,
            VK_M, VK_N, VK_O, VK_OEM_1, VK_OEM_2, VK_OEM_COMMA, VK_OEM_PERIOD, VK_P, VK_Q, VK_R,
            VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z,
        },
        WindowsAndMessaging::{
            CallNextHookEx, SetWindowsHookExA, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN,
            WM_SYSKEYDOWN,
        },
    },
};

use crate::keyboard::KeyCode;

type BoxedCallbackFunction = Box<dyn FnMut(KeyCode) + Send>;

struct CallbackEntry {
    callback: BoxedCallbackFunction,
    id: usize,
}

static CALLBACK_HANDLERS: Mutex<Option<Vec<CallbackEntry>>> = Mutex::new(None);

unsafe extern "system" fn keyboard_hook_callback(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let event_type = wparam.0 as u32;
        if event_type == WM_KEYDOWN || event_type == WM_SYSKEYDOWN {
            let event_info = &mut *(lparam.0 as *mut KBDLLHOOKSTRUCT);
            if let Some(key_code) = match VIRTUAL_KEY(event_info.vkCode as u16) {
                VK_Q => Some(KeyCode::Q),
                VK_W => Some(KeyCode::W),
                VK_E => Some(KeyCode::E),
                VK_R => Some(KeyCode::R),
                VK_T => Some(KeyCode::T),
                VK_Y => Some(KeyCode::Y),
                VK_U => Some(KeyCode::U),
                VK_I => Some(KeyCode::I),
                VK_O => Some(KeyCode::O),
                VK_P => Some(KeyCode::P),
                VK_A => Some(KeyCode::A),
                VK_S => Some(KeyCode::S),
                VK_D => Some(KeyCode::D),
                VK_F => Some(KeyCode::F),
                VK_G => Some(KeyCode::G),
                VK_H => Some(KeyCode::H),
                VK_J => Some(KeyCode::J),
                VK_K => Some(KeyCode::K),
                VK_L => Some(KeyCode::L),
                // On standard US QWERTY, the ';' key is mapped to VK_OEM_1 for some reason.
                VK_OEM_1 => Some(KeyCode::Semicolon),
                VK_Z => Some(KeyCode::Z),
                VK_X => Some(KeyCode::X),
                VK_C => Some(KeyCode::C),
                VK_V => Some(KeyCode::V),
                VK_B => Some(KeyCode::B),
                VK_N => Some(KeyCode::N),
                VK_M => Some(KeyCode::M),
                VK_OEM_COMMA => Some(KeyCode::Comma),
                VK_OEM_PERIOD => Some(KeyCode::Dot),
                // '/' is also OEM_2.
                VK_OEM_2 => Some(KeyCode::Slash),
                _ => None,
            } {
                let mut callbacks = CALLBACK_HANDLERS.lock().unwrap();
                if let Some(callbacks) = &mut *callbacks {
                    for callback in callbacks {
                        (callback.callback)(key_code);
                    }
                }
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub struct Tracer(usize);

impl Tracer {
    pub fn new<CallbackFunction>(callback: CallbackFunction) -> Self
    where
        CallbackFunction: FnMut(KeyCode) + Send + 'static,
    {
        static ADDED_HOOK: AtomicBool = AtomicBool::new(false);
        // If we haven't already registered the global key hook, do it here.
        if ADDED_HOOK
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            unsafe {
                SetWindowsHookExA(
                    WH_KEYBOARD_LL,
                    Some(keyboard_hook_callback),
                    HINSTANCE::default(),
                    0,
                )
                .unwrap()
            };
        }
        let mut handlers = CALLBACK_HANDLERS.lock().unwrap();
        if handlers.is_none() {
            *handlers = Some(Vec::new());
        }
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        handlers.as_mut().unwrap().push(CallbackEntry {
            callback: Box::new(callback),
            id,
        });
        Self(id)
    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        let mut handlers = CALLBACK_HANDLERS.lock().unwrap();
        if let Some(handlers) = &mut *handlers {
            handlers.retain(|entry| entry.id != self.0);
        }
    }
}
