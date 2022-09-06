use std::sync::{Arc, Mutex};

use bevy::{
    prelude::Component,
    reflect::{TypeData, TypeInfo},
};
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
    fn location(&self) -> (usize, usize);
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
    fn location(&self) -> (usize, usize) {
        match &self.state {
            PlayerState::Death => (0 * 8, 8),
            PlayerState::Run => (1 * 8, 6),
            PlayerState::Jump => (2 * 8, 2),
            PlayerState::Crouch => (3 * 8, 4),
            PlayerState::Idle => (4 * 8, 5),
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
    fn location(&self) -> (usize, usize) {
        match &self.state {
            EnemyState::Death => (0 * 8, 8),
            EnemyState::Run => (1 * 8, 6),
            EnemyState::Idle => (4 * 8, 5),
            _ => (0, 0),
        }
    }
}
