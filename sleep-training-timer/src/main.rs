extern crate timer;
extern crate chrono;
#[macro_use] extern crate dbg;

use std::{thread, time};

mod sleep_timer;

fn main() {
    let mut timer_manager = sleep_timer::TimerManager::new(1);
    let timer = timer_manager.start_timer_sequence();
    println!("{:?}", timer.time_passed());
    thread::sleep(time::Duration::from_millis(5000));
    println!("{:?}", timer.time_passed());
}
