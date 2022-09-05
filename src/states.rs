use std::sync::{Arc, Mutex};

use bevy::prelude::Component;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::components::Player;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    MainGame,
    ControlMenu,
    GameOver,
}

pub trait SpriteLocation {
    fn location(&self) -> (u8, u8);
}

pub enum PlayerState {
    Death,
    Run,
    Jump,
    Crouch,
    Idle,
}

#[derive(Component)]
pub struct PlayerAnimationInfo {
    pub state: PlayerState,
}

impl SpriteLocation for PlayerAnimationInfo {
    fn location(&self) -> (u8, u8) {
        match self.state {
            Death => (0, 8),
            Run => (1, 6),
            Jump => (2, 2),
            Crouch => (3, 4),
            Idle => (4, 5),
            _ => (0, 0),
        }
    }
}

pub enum EnemyState {
    Death,
    Run,
    Idle,
}

#[derive(Component)]
pub struct EnemyAnimationInfo {
    pub state: EnemyState,
}

impl SpriteLocation for EnemyAnimationInfo {
    fn location(&self) -> (u8, u8) {
        match self.state {
            Death => (0, 8),
            Run => (1, 6),
            Idle => (4, 5),
            _ => (0, 0),
        }
    }
}
