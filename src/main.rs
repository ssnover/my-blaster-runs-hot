#![allow(unused)]

use bevy::prelude::*;

const QWARK_SPRITE: &str = "qwark.png";
const QWARK_SIZE: (f32, f32) = (500., 500.);

mod blaster;
mod civilian;
mod components;
mod constants;
mod gamepad;
mod movement;
mod player;
mod resources;
mod utils;
use resources::{GameTextures, PlayerScore, WindowSize, BlasterHeat};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "My Blaster Runs Hot".to_string(),
            width: 1024.,
            height: 768.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(civilian::CivilianPlugin)
        .add_plugin(gamepad::GamepadPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut cmds: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    // Add the camera
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add WinSize resource
    let window = windows.get_primary().unwrap();
    cmds.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });

    let game_textures = GameTextures {
        player: asset_server.load(QWARK_SPRITE),
    };
    cmds.insert_resource(game_textures);

    cmds.insert_resource(PlayerScore(0));
    cmds.insert_resource(BlasterHeat(0.));
}
