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

    let texture_handle_crab = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    let texture_atlas_crab =
        TextureAtlas::from_grid(texture_handle_crab, Vec2::new(126.0, 39.0), 1, 8);
    let texture_atlas_handle_crab = texture_atlases.add(texture_atlas_crab);

    let texture_handle_civ = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    let texture_atlas_civ =
        TextureAtlas::from_grid(texture_handle_civ, Vec2::new(126.0, 39.0), 1, 8);
    let texture_atlas_handle_civ = texture_atlases.add(texture_atlas_civ);

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
                    spawn_civilian(&mut cmds, spawn_position, &texture_atlas_handle_crab)
                }
                Some(SpawnType::Crab) => {
                    spawn_crab(&mut cmds, spawn_position, &texture_atlas_handle_civ)
                }
                _ => {}
            }
        }
    }
}
