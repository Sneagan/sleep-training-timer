extern crate time;
extern crate futures;
extern crate tokio;
extern crate tokio_timer;
extern crate clap;
extern crate indicatif;
extern crate console;

use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App};
use tokio_timer::*;
use time::{SteadyTime, Duration};
use futures::{Future, Stream};


fn main() {
    let matches = App::new("Infant Sleep Training Timer")
        .version("0.1.0")
        .author("Jackson Egan")
        .about("I am not a doctor. Use for sleep training at your own risk.")
        .arg(Arg::with_name("night")
             .short("n")
             .long("night")
             .value_name("NIGHT")
             .help("Tonight is night X of sleep training. [i.e. 1]")
             .required(true))
        .get_matches();
    let night = matches.value_of("night").unwrap_or("1");
    timer_sequence_for_day(night.parse::<i32>().unwrap());

    pub fn timer_sequence_for_day(day_number: i32) -> i32 {
        let durations: Vec<u64> = match day_number {
            1 => vec![3, 1, 5, 1, 10, 1, 10, 1, 10, 1, 10],
            2 => vec![5, 1, 10, 1, 12, 1, 12, 1, 12, 1, 12],
            3 => vec![10, 1, 12, 1, 15, 1, 15, 1, 15, 1, 15],
            4 => vec![12, 1, 15, 1, 17, 1, 17, 1, 17, 1, 17],
            5 => vec![15, 1, 17, 1, 20, 1, 20, 1, 20, 1, 20],
            6 => vec![17, 1, 20, 1, 25, 1, 25, 1, 25, 1, 25],
            7 => vec![20, 1, 25, 1, 30, 1, 30, 1, 30, 1, 30],
            _ => vec![0],
        };

        let mut interval = 1;
        
        for (i, duration) in durations.iter().enumerate() {
            let duration_fmt: i64 = duration.to_string().parse::<i64>().unwrap();
            let start_time_plus = SteadyTime::now() + Duration::minutes(duration_fmt) + Duration::seconds(2);

            if i.to_string().parse::<i32>().unwrap() % 2 == 0 {
                println!("Interval {}. Allow baby to cry.", interval);
                interval = interval + 1;
            }
            else {
                println!("Feel free to comfort baby if needed until timer completes.")
            }

            let bar = ProgressBar::new(duration*60);
            bar.set_style(ProgressStyle::default_bar()
                .template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"));
            Timer::default()
                .interval(Duration::seconds(1).to_std().unwrap())
                .take(duration*60)
                .for_each(|val| {
                    let secs = (start_time_plus - SteadyTime::now()).num_seconds();
                    bar.inc(1);
                    Ok(val)
                })
                .wait();
                bar.finish();
        }
        0
    }
}

