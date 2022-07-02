use std::time::Duration;

pub struct CooldownTimer {
    pub duration: Duration,
    pub elapsed: Option<Duration>,
}

impl CooldownTimer {
    pub fn from_seconds(secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(secs),
            elapsed: None,
        }
    }

    pub fn trigger(&mut self) {
        self.elapsed = Some(Duration::from_secs(0));
    }

    pub fn ready(&self) -> bool {
        self.elapsed.is_none()
    }

    pub fn tick(&mut self, delta: Duration) {
        if let Some(ref mut elapsed) = self.elapsed {
            *elapsed += delta;
            if *elapsed > self.duration {
                self.elapsed = None;
            }
        }
    }
}
