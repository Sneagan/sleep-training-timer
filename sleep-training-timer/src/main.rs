extern crate futures;
use std::{thread, time};

mod timer;

fn main() {
    let mut timer_manager = timer::TimerManager::new(1);
    let timer = timer_manager.start_timer_sequence();
    println!("{:?}", timer.time_passed());
    thread::sleep(time::Duration::from_millis(5000));
    println!("{:?}", timer.time_passed());
}
