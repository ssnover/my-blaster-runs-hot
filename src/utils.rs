use std::time::Duration;

use bevy::math::Vec2;

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

pub fn normalize_vec2(vec: Vec2) -> Vec2 {
    if vec.x.abs() > std::f32::EPSILON || vec.y.abs() > std::f32::EPSILON {
        let magnitude = (vec.x.powf(2.) + vec.y.powf(2.)).sqrt();
        let scalar = 1. / magnitude;
        Vec2::new(vec.x * scalar, vec.y * scalar)
    } else {
        Vec2::new(0., 0.)
    }
}

//Vec2.angle_between(Vec2::new(1.0,0.0));