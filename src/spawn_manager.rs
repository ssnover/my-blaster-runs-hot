use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

use crate::civilian::spawn_civilian;
use crate::components::{Civilian, Enemy};
use crate::enemy::spawn_crab;
use crate::resources::{GameTextures, SpawnQueue, SpawnType, WindowSize};
use crate::rounds::RoundTracker;
use crate::states::GameState;

pub struct SpawnManagerPlugin;

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainGame).with_system(spawn_startup_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainGame).with_system(spawn_manager_system),
        );
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

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let current_max_spawns = round_tracker.current_round_data().unwrap().max_spawns as usize;
    let number_of_spawns = query.iter().count();

    if current_max_spawns > number_of_spawns {
        let mut rng = rand::thread_rng();
        let diff = current_max_spawns - number_of_spawns;
        for _ in 0..diff {
            let spawn_position = Vec2::new(
                rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
            );
            match spawn_queue.pop_front() {
                Some(SpawnType::Civilian) => {
                    spawn_civilian(&mut cmds, spawn_position, asset_server, texture_atlases)
                }
                Some(SpawnType::Crab) => {
                    spawn_crab(&mut cmds, spawn_position, asset_server, texture_atlases)
                }
                _ => {}
            }
        }
    }
}
