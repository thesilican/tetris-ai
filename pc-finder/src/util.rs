use std::collections::VecDeque;
use std::lazy::SyncLazy;
use std::time::{Duration, Instant};

/// A simple struct to create a stopwatch
/// that gives the average duration over N periods
#[derive(Debug)]
pub struct AvgStopwatch {
    window: u32,
    start: Option<Instant>,
    times: VecDeque<Duration>,
}
impl AvgStopwatch {
    pub fn new(window: u32) -> Self {
        AvgStopwatch {
            window,
            start: None,
            times: VecDeque::new(),
        }
    }
    pub fn start(&mut self) {
        self.start = Some(Instant::now())
    }
    pub fn stop(&mut self) {
        match self.start.take() {
            Some(start) => {
                let elapsed = start.elapsed();
                self.times.push_back(elapsed);
                if self.times.len() > self.window as usize {
                    self.times.pop_front();
                }
            }
            None => {
                // Stopwatch wasn't started, do nothing
            }
        }
    }
    pub fn reading(&self) -> Duration {
        self.times.iter().sum::<Duration>() / self.times.len() as u32
    }
}

static REDIS_CLIENT: SyncLazy<redis::Client> =
    SyncLazy::new(|| redis::Client::open("redis://127.0.0.1/").unwrap());

pub fn get_redis_con() -> redis::Connection {
    REDIS_CLIENT.get_connection().unwrap()
}
