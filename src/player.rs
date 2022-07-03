use bevy::prelude::*;

use crate::blaster;
use crate::components::{Moveable, NormalBlasterFire, Player, RangedWeapon, Size, Velocity};
use crate::constants::*;
use crate::resources::{BlasterHeat, Controller, GameTextures, WindowSize};
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
    mut blaster_heat: ResMut<BlasterHeat>,
    mut query: Query<(&Transform, &mut RangedWeapon), With<Player>>,
) {
    let (tf, mut weapon_data) = query.get_single_mut().unwrap();
    weapon_data.fire_rate_timer.tick(time.delta());
    blaster_heat.overheat_cooldown_timer.tick(time.delta());
    blaster_heat.value =
        0f32.max(blaster_heat.value - (time.delta_seconds() * BLASTER_COOLOFF_MULTIPLIER));

    if blaster_heat.value >= MAX_BLASTER_HEAT {
        blaster_heat.overheat_cooldown_timer.trigger();
    }

    if weapon_data.firing
        && weapon_data.fire_rate_timer.ready()
        && blaster_heat.overheat_cooldown_timer.ready()
    {
        weapon_data.fire_rate_timer.trigger();
        blaster_heat.value += BLASTER_SHOT_HEAT_ADDITION;
        println!("Blaster Temp: {} C", blaster_heat.value);
        blaster::create_blaster_shot(
            &mut cmds,
            tf.translation,
            weapon_data.aim_direction,
            Color::rgb_u8(240, 0, 15),
            true,
        );
    }
}
