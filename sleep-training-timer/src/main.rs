extern crate chrono;
#[macro_use]
extern crate futures;
extern crate tokio;
extern crate tokio_timer;

use tokio_timer::*;
use tokio::executor::current_thread;
use std::time::*;
use futures::{Future, Stream, Async};


fn main() {
    let seconds_passed = timer_sequence_for_day(1);
    println!("{:?}", seconds_passed);

    pub fn timer_sequence_for_day(day_number: i32) -> i32 {
        let mut seconds_passed_this_loop = 0;
        let mut seconds_passed_total = 0;
        let durations = match day_number {
            1 => vec![3, 5, 10, 10, 10, 10],
            2 => vec![5, 10, 12, 12, 12, 12],
            3 => vec![10, 12, 15, 15, 15, 15],
            4 => vec![12, 15, 17, 17, 17, 17],
            5 => vec![15, 17, 20, 20, 20, 20],
            6 => vec![17, 20, 25, 25, 25, 25],
            7 => vec![20, 25, 30, 30, 30, 30],
            _ => vec![0],
        };
        
        for duration in durations {
            Timer::default()
                .interval(Duration::new(1, 0))
                .take(5)
                .for_each(|val| {
                    seconds_passed_this_loop = seconds_passed_this_loop + 1;
                    seconds_passed_total = seconds_passed_total + 1;
                    println!("{:?}", seconds_passed_this_loop);
                    Ok(val)
                })
                .wait();
            seconds_passed_this_loop = 0;
        }

        seconds_passed_total
    }
}

