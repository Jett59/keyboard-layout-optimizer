use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Instant,
};

use digram_timing::DigramTimingHint;
use gui::launch_gui;

use keyboard::KeyboardLayout;
use layout_creator::LayoutCreator;
use trace::Tracer;

use crate::layout_creator::LayoutHint;

mod digram_timing;
mod gui;
mod keyboard;
mod layout_creator;

#[cfg_attr(windows, path = "windows/input.rs")]
mod input;
#[cfg_attr(windows, path = "windows/trace.rs")]
mod trace;

fn main() -> Result<(), Box<dyn Error>> {
    let layout_creator = Arc::new(Mutex::new(LayoutCreator::new(vec![Box::new(
        DigramTimingHint::default(),
    )])));
    let active_keyboard_layout: Arc<Mutex<Option<KeyboardLayout>>> = Arc::new(Mutex::new(None));
    let layout_creator2 = layout_creator.clone();
    let active_keyboard_layout2 = active_keyboard_layout.clone();
    let active_keyboard_layout3 = active_keyboard_layout.clone();
    let _tracer = Tracer::new(move |context, key_code| {
        let mut layout_creator = layout_creator2.lock().unwrap();
        let active_keyboard_layout = active_keyboard_layout3.lock().unwrap();
        layout_creator.receive_key_press(key_code, Instant::now());
        if let Some(active_keyboard_layout) = *active_keyboard_layout {
            context.suppress();
            let translated_keystroke = active_keyboard_layout
                .key_at(KeyboardLayout::QWERTY.position_of(key_code).unwrap());
            context.send_keystroke(translated_keystroke);
        }
    });
    launch_gui(
        Box::new(move || {
            let layout_creator = layout_creator.lock().unwrap();
            layout_creator.create_layout()
        }),
        Box::new(move |layout| {
            let mut active_keyboard_layout = active_keyboard_layout.lock().unwrap();
            *active_keyboard_layout = Some(layout.clone());
        }),
        Box::new(move || {
            let mut active_keyboard_layout = active_keyboard_layout2.lock().unwrap();
            *active_keyboard_layout = None;
        }),
    )
}
