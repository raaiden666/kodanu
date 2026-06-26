use std::time::{Duration, Instant};

pub struct Time {
    startup: Instant,
    last: Instant,
    delta: Duration,
    elapsed: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            startup: Instant::now(),
            last: Instant::now(),
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta = now.duration_since(self.last);
        self.elapsed = now.duration_since(self.startup);

        self.last = now;
    }

    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
