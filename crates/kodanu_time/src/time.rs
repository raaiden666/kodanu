use std::time::{Duration, Instant};

pub struct Time {
    startup: Instant,
    last: Instant,
    delta: Duration,
    elapsed: Duration,
    max_delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            startup: now,
            last: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            max_delta: Duration::from_millis(100),
        }
    }
}

impl Time {
    #[inline]
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last);

        self.delta = delta.min(self.max_delta);
        self.elapsed = now.duration_since(self.startup);

        self.last = now;
    }

    #[inline]
    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
