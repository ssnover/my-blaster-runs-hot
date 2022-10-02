#![allow(unused)]

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_rapier2d::prelude::*;

const QWARK_SPRITE: &str = "qwark.png";
const QWARK_SIZE: (f32, f32) = (500., 500.);
const ENEMY_SPRITE: &str = "tux.png";
const ENEMY_SIZE: (f32, f32) = (500., 500.);

mod blaster;
mod camera;
mod civilian;
mod components;
mod constants;
mod debug;
mod enemy;
mod game_over;
mod gamepad;
mod graphics;
mod main_menu;
mod player;
mod projectile_collision;
mod resources;
mod rounds;
mod spawn_manager;
mod states;
mod ui;
mod utils;

use constants::*;
use game_over::GameOverMenuPlugin;
use main_menu::MainMenuPlugin;
use projectile_collision::CollisionPlugin;
use resources::{BlasterHeat, GameFont, GameTextures, PlayerLives, PlayerScore, WindowSize};
use states::GameState;
use utils::CooldownTimer;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "My Blaster Runs Hot".to_string(),
            width: 1024.,
            height: 768.,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest()) //Prevents blurry images apparently
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(states::GameState::MainMenu)
        //start plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(civilian::CivilianPlugin)
        //.add_plugin(gamepad::GamepadPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(graphics::AnimationPlugin)
        .add_plugin(blaster::BlasterPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(projectile_collision::CollisionPlugin)
        .add_plugin(rounds::RoundManagerPlugin)
        .add_plugin(spawn_manager::SpawnManagerPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(GameOverMenuPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        //startup system
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut state: ResMut<State<GameState>>,
) {
    // Add the camera
    //cmds.spawn_bundle(Camera2dBundle::default());

    // Add WinSize resource
    let window = windows.get_primary().unwrap();
    cmds.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });

    let game_textures = GameTextures {
        player: asset_server.load(QWARK_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
    };
    cmds.insert_resource(game_textures);

    cmds.insert_resource(PlayerScore(0));
    cmds.insert_resource(PlayerLives(0));
    cmds.insert_resource(BlasterHeat {
        value: 0.,
        overheat_cooldown_timer: CooldownTimer::from_seconds(COOLDOWN_TIME_SECONDS),
    });

    let game_font = GameFont(asset_server.load("FiraSans-Bold.ttf"));
    cmds.insert_resource(game_font);
}
