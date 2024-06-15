use std::error::Error;

use eframe::egui::{self, Button, Direction, Vec2};
use eframe::egui::{Layout, Pos2, Rect, Ui};
use eframe::NativeOptions;

use crate::keyboard::KeyboardLayout;

struct KeyboardLayoutOptimizerGui {
    create_layout: Box<dyn FnMut() -> KeyboardLayout>,
    custom_keyboard_layout: Option<KeyboardLayout>,
}

impl eframe::App for KeyboardLayoutOptimizerGui {
    fn update(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            if let Some(layout) = &self.custom_keyboard_layout {
                Self::render_keyboard(ui, layout);
            } else {
                let create_button = ui.button("Create layout");
                if create_button.clicked() {
                    let layout = (self.create_layout)();
                    self.custom_keyboard_layout = Some(layout);
                }
            }
            Self::render_keyboard(ui, &KeyboardLayout::QWERTY);
        });
    }
}

impl KeyboardLayoutOptimizerGui {
    fn render_keyboard(ui: &mut Ui, layout: &KeyboardLayout) {
        let Vec2 {
            x: available_width,
            y: available_height,
        } = ui.available_size();
        // The keyboard should take up at most 2/3 of the available width and 2/3 of the available height
        let key_size = f32::min(
            2.0 / 3.0 * available_width / 11.0,
            2.0 / 3.0 * available_height / 3.0,
        );
        let min_y = available_height - key_size * 3.0;
        let height = 3.0 * key_size;
        let mut keyboard_region = ui.child_ui(
            Rect::from_min_size(Pos2::new(0.0, min_y), Vec2::new(available_width, height)),
            Layout::centered_and_justified(Direction::TopDown),
        );
        // Unfortunately there is no easy way to get the keyboard to be horizontally centered
        let offset = (available_width - 11.0 * key_size) / 2.0;
        keyboard_region.vertical(|rows| {
            let row_offsets = [0.0, key_size / 2.0, key_size];
            for row_index in 0..3 {
                rows.horizontal(|row| {
                    row.add_space(offset);
                    row.add_space(row_offsets[row_index]);
                    for column_index in 0..10 {
                        row.add(
                            Button::new(layout.key_at((row_index, column_index)).to_string())
                                .min_size(Vec2::new(key_size, key_size)),
                        );
                    }
                    row.add_space(key_size - row_offsets[row_index]);
                    row.add_space(offset)
                });
            }
        });
    }
}

pub fn launch_gui(create_layout: Box<dyn FnMut() -> KeyboardLayout>) -> Result<(), Box<dyn Error>> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Keyboard Layout Optimizer",
        native_options,
        Box::new(|_creation_context| {
            Box::new(KeyboardLayoutOptimizerGui {
                create_layout,
                custom_keyboard_layout: None,
            })
        }),
    )?;
    Ok(())
}
