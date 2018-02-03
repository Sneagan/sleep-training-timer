extern crate futures;

mod timer;

fn main() {
    let mut timer_manager = timer::TimerManager::new(1);
    let timer = timer_manager.start_timer_sequence();
    println!("{:?}", timer.time_passed());
}
