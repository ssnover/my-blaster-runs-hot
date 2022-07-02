use bevy::prelude::*;

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
}

#[derive(Deref, DerefMut)]
pub struct Controller(pub Gamepad);
