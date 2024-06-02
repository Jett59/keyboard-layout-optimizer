use std::error::Error;

use eframe::egui;
use eframe::NativeOptions;

use crate::keyboard::KeyboardLayout;

struct KeyboardLayoutOptimizerGui {
    create_layout: Box<dyn FnMut() -> KeyboardLayout>,
    enable_layout: Box<dyn FnMut(KeyboardLayout) -> ()>,
}

impl eframe::App for KeyboardLayoutOptimizerGui {
    fn update(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            let create_button = ui.button("Create layout");
            if create_button.clicked() {
                println!("{:?}", (self.create_layout)());
            }
            let create_and_enable_button = ui.button("Create and enable layout");
            if create_and_enable_button.clicked() {
                let layout = (self.create_layout)();
                (self.enable_layout)(layout);
            }
        });
    }
}

pub fn launch_gui(
    create_layout: Box<dyn FnMut() -> KeyboardLayout>,
    enable_layout: Box<dyn FnMut(KeyboardLayout) -> ()>,
) -> Result<(), Box<dyn Error>> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Keyboard Layout Optimizer",
        native_options,
        Box::new(|_creation_context| {
            Box::new(KeyboardLayoutOptimizerGui {
                create_layout,
                enable_layout,
            })
        }),
    )?;
    Ok(())
}
