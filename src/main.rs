use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Instant,
};

use digram_timing::DigramTimingHint;
use gui::launch_gui;
use layout_creator::LayoutCreator;
use trace::Tracer;

use crate::layout_creator::LayoutHint;

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
    let layout_creator2 = layout_creator.clone();
    let _tracer = Tracer::new(move |key_code| {
        let mut layout_creator = layout_creator2.lock().unwrap();
        println!("{:?}", key_code);
        layout_creator.receive_key_press(key_code, Instant::now());
    });
    launch_gui(Box::new(move || {
        let layout_creator = layout_creator.lock().unwrap();
        layout_creator.create_layout()
    }), Box::new(move |layout| {
        println!("Enabling!");
    }), Box::new(move || {
        println!("Disabling!");
    }))
}
