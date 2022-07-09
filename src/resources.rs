use bevy::prelude::*;
use std::collections::VecDeque;

use crate::utils::CooldownTimer;

pub enum SpawnType {
    Civilian,
    Crab,
    Bot,
    Tank,
    Exploder,
}

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
}

pub struct GameFont(pub Handle<Font>);

#[derive(Deref, DerefMut)]
pub struct Controller(pub Gamepad);

#[derive(Deref, DerefMut)]
pub struct PlayerScore(pub usize);

pub struct BlasterHeat {
    pub value: f32,
    pub overheat_cooldown_timer: CooldownTimer,
}

#[derive(Deref, DerefMut)]
pub struct SpawnQueue(pub VecDeque<SpawnType>);
