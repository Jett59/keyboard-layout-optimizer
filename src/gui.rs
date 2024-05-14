use std::error::Error;

use eframe::egui;
use eframe::NativeOptions;

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

pub fn launch_gui() -> Result<(), Box<dyn Error>> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Keyboard Layout Optimizer",
        native_options,
        Box::new(|_creation_context| Box::new(KeyboardLayoutOptimizerGui::default())),
    )?;
    Ok(())
}
