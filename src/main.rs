use std::error::Error;

use eframe::egui;
use eframe::NativeOptions;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::WindowsAndMessaging::CallNextHookEx;
use windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExA;
use windows::Win32::UI::WindowsAndMessaging::KBDLLHOOKSTRUCT;
use windows::Win32::UI::WindowsAndMessaging::WH_KEYBOARD_LL;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_SYSKEYDOWN;

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        if wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN {
            println!("Key pressed");
            let key_info = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            println!("Scan code: {}", key_info.scanCode);
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), HMODULE::default(), 0)? };
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Keyboard Layout Optimizer",
        native_options,
        Box::new(|_creation_context| Box::new(KeyboardLayoutOptimizerGui::default())),
    )?;
    Ok(())
}

#[derive(Default)]
struct KeyboardLayoutOptimizerGui {}

impl eframe::App for KeyboardLayoutOptimizerGui {
    fn update(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            let button1 = ui.button("Button 1");
            if button1.clicked() {
                println!("Button 1 clicked");
            }
        });
    }
}
