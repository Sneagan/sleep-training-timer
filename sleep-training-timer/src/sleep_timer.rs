use tokio_timer::*;
use futures::*;
use std::time::*;
use tokio_core::reactor::Core;

pub struct SleepTimer {
  duration: Duration,
  timer: Timer,
  start_time: Option<Instant>,
  ms_passed: u64,
}

impl SleepTimer {
    fn start(&mut self) -> &mut SleepTimer {
        self.start_time = Some(Instant::now());
        let mut core = Core::new().unwrap();
        let ms_sleeper = self.timer.interval(Duration::from_millis(1));
        let ms_duration = self.duration.as_secs() * 60;
        let mut passed = 0;
        let logger = ms_sleeper
            .take(ms_duration)
            .map(|()| {
                passed += 1;
                println!("{:?}", Duration::seconds(Duration::from_millis(passed)));
                passed
            })
            .collect();

        let result = core.run(logger).unwrap();
        self
    }

    pub fn time_passed(&self) -> u64 {
        self.ms_passed
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

fn int_to_timer(duration: u64) -> SleepTimer {
    SleepTimer{
        duration: Duration::new(duration*60, 0),
        start_time: None,
        timer: Timer::default(),
        ms_passed: 0,
    }
}

