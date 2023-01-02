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
    fn next_index(&self, curr_index: usize) -> usize;
    fn is_flip(&self) -> bool;
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
    pub is_flip: bool,
}

impl SpriteLocation for PlayerAnimationInfo {
    fn location(&self) -> (usize, usize) {
        match &self.state {
            PlayerState::Death => (0 * 8, 8),
            PlayerState::Run => (1 * 8, 6),
            PlayerState::Jump => (2 * 8, 2),
            PlayerState::Crouch => (3 * 8, 3),
            PlayerState::Idle => (4 * 8, 5),
            _ => (0, 0),
        }
    }

    fn next_index(&self, curr_index: usize) -> usize {
        let (index_offset, col_length) = self.location();
        let mut next_index = curr_index.saturating_sub(index_offset);
        next_index = (next_index + 1) % col_length;
        return next_index + index_offset;
    }

    fn is_flip(&self) -> bool {
        return self.is_flip;
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
    pub is_flip: bool,
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

    fn next_index(&self, curr_index: usize) -> usize {
        let (index_offset, col_length) = self.location();
        let mut next_index = curr_index.saturating_sub(index_offset);
        next_index = (next_index + 1) % col_length;
        return next_index + index_offset;
    }

    fn is_flip(&self) -> bool {
        return self.is_flip;
    }
}

pub enum CivilianState {
    Death,
    Run,
    Idle,
}

#[derive(Component)]
pub struct CivilianAnimationInfo {
    pub state: CivilianState,
    pub is_flip: bool,
}

impl SpriteLocation for CivilianAnimationInfo {
    fn location(&self) -> (usize, usize) {
        match &self.state {
            CivilianState::Death => (0 * 8, 8),
            CivilianState::Run => (1 * 8, 6),
            CivilianState::Idle => (4 * 8, 5),
            _ => (0, 0),
        }
    }

    fn next_index(&self, curr_index: usize) -> usize {
        let (index_offset, col_length) = self.location();
        let mut next_index = curr_index.saturating_sub(index_offset);
        next_index = (next_index + 1) % col_length;
        return next_index + index_offset;
    }

    fn is_flip(&self) -> bool {
        return self.is_flip;
    }
}
