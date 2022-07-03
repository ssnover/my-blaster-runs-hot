use bevy::prelude::*;

use crate::components::{
    Despawnable, Moveable, NormalBlasterFire, Player, RangedWeapon, Size, Velocity,
};
use crate::constants::{BASE_SPEED, SPRITE_SCALE, TIME_STEP};
use crate::resources::{Controller, GameTextures, WindowSize};
use crate::utils::CooldownTimer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_control_system)
            .add_system(player_fire_blaster_system);
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
    .insert(Velocity::from(Vec2::new(0., 0.)))
    .insert(Moveable {
        speed_multiplier: 1.,
        ..Default::default()
    })
    .insert(Size(Vec2::new(50., 50.)))
    .insert(RangedWeapon {
        aim_direction: Vec2::new(1., 0.),
        fire_rate_timer: CooldownTimer::from_seconds(0.5),
        ..Default::default()
    });
}

fn player_control_system(
    mut query: Query<(&mut Velocity, &mut RangedWeapon), With<Player>>,
    controller: Option<Res<Controller>>,
    keys: Res<Input<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
) {
    let (mut velocity, mut weapon_data) = query.get_single_mut().unwrap();
    if let Some(controller) = controller {
        let axis_lx = GamepadAxis(controller.0, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(controller.0, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            velocity.x = x;
            velocity.y = y;
        }
        let normal_fire_button = GamepadButton(controller.0, GamepadButtonType::LeftTrigger);
        weapon_data.firing = buttons.pressed(normal_fire_button);

        let axis_rx = GamepadAxis(controller.0, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis(controller.0, GamepadAxisType::RightStickY);
        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            if x.abs() > 0.2 || y.abs() > 0.2 {
                weapon_data.aim_direction = Vec2::new(x, y);
            }
        }
    } else {
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

        weapon_data.firing = keys.pressed(KeyCode::RShift);
        if keys.pressed(KeyCode::Up) {
            weapon_data.aim_direction.y = 1.;
        }
        if keys.pressed(KeyCode::Down) {
            weapon_data.aim_direction.y = -1.;
        }
        if keys.pressed(KeyCode::Left) {
            weapon_data.aim_direction.x = -1.;
        }
        if keys.pressed(KeyCode::Right) {
            weapon_data.aim_direction.x = 1.;
        }
    }
}

fn player_fire_blaster_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut RangedWeapon), With<Player>>,
) {
    let (tf, mut weapon_data) = query.get_single_mut().unwrap();
    weapon_data.fire_rate_timer.tick(time.delta());

    if weapon_data.firing && weapon_data.fire_rate_timer.ready() {
        weapon_data.fire_rate_timer.trigger();
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(240, 0, 15),
                custom_size: Some(Vec2::new(20., 20.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(tf.translation.x, tf.translation.y, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NormalBlasterFire)
        .insert(Despawnable)
        .insert(Velocity::from(weapon_data.aim_direction))
        .insert(Moveable {
            solid: false,
            speed_multiplier: 1.5,
        });
    }
}
