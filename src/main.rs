use std::error::Error;

use gui::launch_gui;
use trace::Tracer;

mod gui;
mod keyboard;
mod layout_creator;

#[cfg_attr(windows, path = "windows/trace.rs")]
mod trace;

fn main() -> Result<(), Box<dyn Error>> {
    let _tracer = Tracer::new(|key_code| {
        println!("{:?}", key_code);
    });
    launch_gui()
}
