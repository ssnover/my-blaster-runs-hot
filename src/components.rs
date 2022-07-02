use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}

#[derive(Component)]
pub struct Moveable {
    pub speed_multiplier: f32,
    pub solid: bool,
}

impl Default for Moveable {
    fn default() -> Self {
        Moveable {
            speed_multiplier: 1.0,
            solid: true,
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Civilian;

#[derive(Component)]
pub struct NormalBlasterFire;

#[derive(Component, Default)]
pub struct RangedWeapon {
    pub aim_direction: Vec2,
    pub firing: bool,
}

#[derive(Component)]
pub struct Despawnable;
