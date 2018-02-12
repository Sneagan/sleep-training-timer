use std::time::Instant;
use chrono::Duration;
use timer::Timer;

pub struct SleepTimer {
  duration: Duration,
  timer: Option<Timer>,
  log_timer: Option<Timer>,
  start_time: Option<Instant>,
  log_time: u64,
}

impl SleepTimer {
    fn start(&mut self) -> &mut SleepTimer {
        self.start_time = Some(Instant::now());
        if let Some(ref timer) = self.timer {
            timer.schedule_repeating(self.duration, || {
                println!("Yo");
            });
        }
        if let Some(ref log_timer) = self.log_timer {
            log_timer.schedule_repeating(self.duration, || {
                self.log_time += 1;
            });
        }
        self
    }

    pub fn time_passed(&self) -> Result<u64, &'static str> {
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
    pub sleep_timers: Vec<SleepTimer>
}

pub struct TimerManager {
    pub timer_collection: TimerSequence,
    pub current_timer: usize,
}

impl TimerManager {
    pub fn new(day: i32) -> TimerManager {
        TimerManager {
            timer_collection: timer_sequence_for_day(day),
            current_timer: 0,
        }
    }

    pub fn start_timer_sequence(&mut self) -> &mut SleepTimer {
        let timer = &mut self.timer_collection.sleep_timers[self.current_timer];
        timer.start()
    }

    fn start_next_timer() {}
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


    TimerSequence{
        sleep_timers: durations.into_iter()
        .map(int_to_timer)
        .collect()
    }
}

fn int_to_timer(duration: i64) -> SleepTimer {
    SleepTimer{
        duration: Duration::seconds(duration),
        start_time: None,
        timer: Some(Timer::new()),
        log_timer:  Some(Timer::new()),
        log_time: 0,
    }
}

