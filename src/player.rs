use bevy::prelude::*;

use crate::components::{Player, Velocity};
use crate::constants::{BASE_SPEED, SPRITE_SCALE, TIME_STEP};
use crate::resources::{Controller, GameTextures, WindowSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_velocity_control_gamepad_system)
            .add_system(player_velocity_control_keyboard_system)
            .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut cmds: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
) {
    // Add the player
    cmds.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity::from(Vec2::new(0., 0.)));
}

fn player_velocity_control_gamepad_system(
    mut query: Query<&mut Velocity, With<Player>>,
    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
) {
    let mut velocity = query.get_single_mut().unwrap();
    if let Some(controller) = controller {
        let axis_lx = GamepadAxis(controller.0, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(controller.0, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            velocity.x = x;
            velocity.y = y;
        }
    }
}

fn player_velocity_control_keyboard_system(
    mut query: Query<&mut Velocity, With<Player>>,
    controller: Option<Res<Controller>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut velocity = query.get_single_mut().unwrap();
    if controller.is_none() {
        if keys.pressed(KeyCode::W) {
            velocity.y = 1.;
        } else if keys.pressed(KeyCode::S) {
            velocity.y = -1.;
        } else {
            velocity.y = 0.;
        }
        if keys.pressed(KeyCode::D) {
            velocity.x = 1.;
        } else if keys.pressed(KeyCode::A) {
            velocity.x = -1.;
        } else {
            velocity.x = 0.;
        }
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut tf) in query.iter_mut() {
        tf.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        tf.translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
