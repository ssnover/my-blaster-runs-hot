use crate::utils::CooldownTimer;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Civilian;

#[derive(Component)]
pub struct Blaster;

#[derive(Component)]
pub struct Projectile {
    pub from_player: bool,
}

#[derive(Component)]
pub struct Lives {
    pub lives_num: u8,
}

#[derive(Component)]
pub struct WeaponData {
    pub aim_direction: Vec2,
    pub firing: bool,
    pub fire_rate_timer: CooldownTimer,
}

//Default is qwark's ranged weapon
impl Default for WeaponData {
    fn default() -> Self {
        Self {
            aim_direction: Default::default(),
            firing: Default::default(),
            fire_rate_timer: CooldownTimer::from_seconds(1.),
        }
    }
}

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct AreaOfEffect(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
