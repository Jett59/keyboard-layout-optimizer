use std::error::Error;

use eframe::egui;
use eframe::NativeOptions;
use trace::Tracer;

mod keyboard;

#[cfg_attr(windows, path = "windows/trace.rs")]
mod trace;

fn main() -> Result<(), Box<dyn Error>> {
    let tracer = Tracer::new(Box::new(|key_code| {
        println!("{:?}", key_code);
    }));
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
