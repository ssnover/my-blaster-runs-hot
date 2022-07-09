use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

use crate::civilian::spawn_civilian;
use crate::components::{Civilian, Enemy};
use crate::enemy::spawn_crab;
use crate::resources::{GameTextures, SpawnQueue, SpawnType, WindowSize};
use crate::rounds::RoundTracker;

pub struct SpawnManagerPlugin;

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_startup_system)
            .add_system(spawn_manager_system);
    }
}

fn spawn_startup_system(mut cmds: Commands) {
    cmds.insert_resource(SpawnQueue(VecDeque::new()));
}

fn spawn_manager_system(
    mut cmds: Commands,
    round_tracker: Res<RoundTracker>,
    mut spawn_queue: ResMut<SpawnQueue>,
    win_size: Res<WindowSize>,
    game_textures: Res<GameTextures>,
    query: Query<(), Or<(With<Civilian>, With<Enemy>)>>,
) {
    let current_max_spawns = round_tracker.current_round_data().unwrap().max_spawns as usize;
    let number_of_spawns = query.iter().count();

    if current_max_spawns > number_of_spawns {
        let mut rng = rand::thread_rng();
        let diff = current_max_spawns - number_of_spawns;
        let spawn_position = Vec2::new(
            rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
            rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
        );
        for _ in 0..diff {
            match spawn_queue.pop_front() {
                Some(SpawnType::Civilian) => spawn_civilian(&mut cmds, spawn_position),
                Some(SpawnType::Crab) => {
                    spawn_crab(&mut cmds, spawn_position, game_textures.enemy.clone())
                }
                _ => {}
            }
        }
    }
}
