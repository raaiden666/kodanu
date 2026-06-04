use std::time::{Duration, Instant};

pub struct Time {
    startup_time: Instant,
    last_frame_time: Instant,
    delta_time: Duration,
    elapsed_time: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            startup_time: Instant::now(),
            last_frame_time: Instant::now(),
            delta_time: Duration::ZERO,
            elapsed_time: Duration::ZERO,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta_time = now.duration_since(self.last_frame_time);
        self.elapsed_time = now.duration_since(self.startup_time);

        self.last_frame_time = now;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time.as_secs_f32()
    }
}
