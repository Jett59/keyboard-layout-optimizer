use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Instant,
};

use digram_timing::DigramTimingHint;
use gui::launch_gui;
use layout_creator::LayoutCreator;
use trace::Tracer;

use crate::{keyboard::KeyboardLayout, layout_creator::LayoutHint};

mod digram_timing;
mod gui;
mod keyboard;
mod layout_creator;

#[cfg_attr(windows, path = "windows/trace.rs")]
mod trace;

fn main() -> Result<(), Box<dyn Error>> {
    let layout_creator = Arc::new(Mutex::new(LayoutCreator::new(vec![Box::new(
        DigramTimingHint::default(),
    )])));
    let enabled_keyboard: Arc<Mutex<Option<KeyboardLayout>>> = Arc::new(Mutex::new(None));
    let layout_creator2 = layout_creator.clone();
    let enabled_keyboard2 = enabled_keyboard.clone();
    let _tracer = Tracer::new(move |key_code| {
        let mut layout_creator = layout_creator2.lock().unwrap();
        println!("{:?}", key_code);
        layout_creator.receive_key_press(key_code, Instant::now());
        let enabled_keyboard = enabled_keyboard2.lock().unwrap();
        if enabled_keyboard.is_some() {
            let enabled_keyboard = enabled_keyboard.as_ref().unwrap();
            println!("{:?}", enabled_keyboard.key_at(KeyboardLayout::QWERTY.position_of(key_code).unwrap()));
            Some(enabled_keyboard.key_at(KeyboardLayout::QWERTY.position_of(key_code).unwrap()))
        } else {
            None
        }
    });
    launch_gui(
        Box::new(move || {
            let layout_creator = layout_creator.lock().unwrap();
            layout_creator.create_layout()
        }),
        Box::new(move |layout| {
            let mut enabled_keyboard = enabled_keyboard.lock().unwrap();
            *enabled_keyboard = Some(layout);
        }),
    )
}
