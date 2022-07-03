use bevy::prelude::*;

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
}

#[derive(Deref, DerefMut)]
pub struct Controller(pub Gamepad);

#[derive(Deref, DerefMut)]
pub struct PlayerScore(pub usize);
