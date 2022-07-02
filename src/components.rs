use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl From<Vec2> for Velocity {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}

#[derive(Component)]
pub struct Player;
