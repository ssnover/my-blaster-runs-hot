use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

use crate::civilian::spawn_civilian;
use crate::components::{Civilian, Enemy};
use crate::enemy::spawn_crab;
use crate::resources::{GameTextures, SpawnQueue, SpawnType, WindowSize};
use crate::rounds::{PopulateQueueEvent, RoundTracker};
use crate::states::GameState;

pub struct NewRoundEvent {}

pub struct SpawnManagerPlugin;

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PopulateQueueEvent>().add_system_set(
            SystemSet::on_update(GameState::MainGame).with_system(spawn_manager_system),
        );
    }
}

fn spawn_manager_system(
    mut cmds: Commands,
    mut round_tracker: ResMut<RoundTracker>,
    mut spawn_queue: ResMut<SpawnQueue>,
    win_size: Res<WindowSize>,
    game_textures: Res<GameTextures>,
    query: Query<(), Or<(With<Civilian>, With<Enemy>)>>,
    mut state: ResMut<State<GameState>>,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut send_populate_queue: EventWriter<PopulateQueueEvent>,
) {
    let current_max_spawns = round_tracker.current_round_data().unwrap().max_spawns as usize;
    let number_of_spawns = query.iter().count();

    if spawn_queue.len() == 0 && number_of_spawns == 0 {
        if (!round_tracker.next_round()) {
            state.push(GameState::GameOver).unwrap();
        }
        send_populate_queue.send(PopulateQueueEvent {});
    }

    let texture_handle_crab =
        asset_server.load("darians-assets/TeamGunner/CHARACTER_SPRITES/Red/Red_Soldier.png");
    let texture_atlas_crab =
        TextureAtlas::from_grid(texture_handle_crab, Vec2::new(50.0, 50.0), 8, 5);
    let texture_atlas_handle_crab = texture_atlases.add(texture_atlas_crab);

    let texture_handle_civ =
        asset_server.load("darians-assets/TeamGunner/CHARACTER_SPRITES/Green/Green_Soldier.png");
    let texture_atlas_civ =
        TextureAtlas::from_grid(texture_handle_civ, Vec2::new(50.0, 50.0), 8, 5);
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
                    spawn_civilian(&mut cmds, spawn_position, &texture_atlas_handle_civ)
                }
                Some(SpawnType::Crab) => {
                    spawn_crab(&mut cmds, spawn_position, &texture_atlas_handle_crab)
                }
                _ => {}
            }
        }
    }
}
