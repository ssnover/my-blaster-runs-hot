use crate::utils::CooldownTimer;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AreaOfEffect(pub bool);

#[derive(Component)]
pub struct Blaster {
    pub damage: u32,
}

#[derive(Component)]
pub struct Civilian;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Health {
    pub health: u32,
}

#[derive(Component, Deref, DerefMut)]
pub struct Lives {
    pub lives_num: u32,
}

#[derive(Component)]
pub struct LivingBeing;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
#[derive(Component)]
pub struct PlayerLivesUI;

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct WeaponData {
    pub aim_direction: Vec2, //This seems fucking useless now
    pub firing: bool,
    pub fire_rate_timer: CooldownTimer,
    pub damage: u32,
}

//Default is qwark's ranged weapon
impl Default for WeaponData {
    fn default() -> Self {
        Self {
            aim_direction: Default::default(),
            firing: false,
            fire_rate_timer: CooldownTimer::from_seconds(1.0),
            damage: 1,
        }
    }
}
