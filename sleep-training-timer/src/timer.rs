use futures::Future;
use std::time::{Duration, Instant};

struct SleepTimer {
  duration: Duration,
  timer: Option<Instant>,
  start_time: Option<Instant>,
}

impl SleepTimer {
    fn start(&mut self) {
        if let Some(ref mut x) = self.start_time {
            *x = Instant::now();
        }
    }

    fn time_passed(self) -> Result<u64, &'static str> {
        match self.start_time {
            Some(start_time) => Ok(
                start_time
                    .elapsed()
                    .as_secs()
            ),
            None => Err("The timer has not been started"),
        }
    }
}

pub struct TimerSequence {
    sleep_timers: Vec<SleepTimer>
}

pub struct TimerManager {
    timer_collection: TimerSequence,
    current_timer: i32,
}

impl TimerManager {
    pub fn new() -> TimerManager {
        TimerManager{
            timer_collection: timer_sequence_for_day(1),
            current_timer: 0,
        }
    }

    fn start_timer_sequence(&mut self) {
        self.timer_collection.sleep_timers[0].start();
    }
}

pub fn timer_sequence_for_day(day_number: i32) -> TimerSequence {
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

    fn int_to_timer(duration: u64) -> SleepTimer {
        SleepTimer{
            duration: Duration::new(duration*60, 0),
            timer: None,
            start_time: None,
        }
    }

    TimerSequence{
        sleep_timers: durations.into_iter()
        .map(int_to_timer)
        .collect()
    }
} 
