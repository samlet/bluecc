use parking_lot::Mutex;
use std::{
    sync::{atomic, Arc},
    thread, time,
};

pub struct Stopwatch {
    time: Mutex<time::Instant>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch { time: Mutex::new(time::Instant::now()),}
    }

    pub fn ms(&self) -> u64 {
        fn as_millis(dur: time::Duration) -> u64 {
            dur.as_secs() * 1_000 + dur.subsec_nanos() as u64 / 1_000_000
        }

        let mut time = self.time.lock();
        let elapsed = as_millis(time.elapsed());

        *time = time::Instant::now();

        elapsed
    }
}
