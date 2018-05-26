
use std::time::Instant;

pub struct Stopwatch {
    start: Instant
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start: Instant::now()
        }
    }


    pub fn tick(&mut self) -> f64 {
        let end = Instant::now();
        let duration = end - self.start;

        self.start = end;

        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
    }
}
