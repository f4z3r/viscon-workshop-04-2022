#[macro_use]
extern crate simple_log;
use ctrlc;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    // handle interrupts
    let stop_cp = stop.clone();
    ctrlc::set_handler(move || {
        println!("received interrupt, stopping");
        stop_cp.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");


    // main application
    simple_log::quick!("info");
    let period = std::time::Duration::from_millis(666);
    while ! stop.load(Ordering::SeqCst) {
        info!("Hello, world!");
        std::thread::sleep(period);
    }
}
