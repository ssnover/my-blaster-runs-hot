use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::blaster;
use crate::components::{Moveable, Player, Projectile, RangedWeapon, Size, Velocity};
use crate::constants::*;
use crate::debug;
use crate::resources::{BlasterHeat, Controller, GameTextures, WindowSize};
use crate::states::GameState;
use crate::utils::CooldownTimer;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<BulletFiredEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame).with_system(player_spawn_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(player_control_system)
                    .with_system(player_fire_blaster_system),
            );
    }
}

fn player_spawn_system(
    mut cmds: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0., 0).into(),
        activation: RigidBodyActivation::cannot_sleep(),
        forces: RigidBodyForce {
            gravity_scale: 0.,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::rectangle(50., 50., 0.1),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    };

    // Add the player sprite
    let sprite = SpriteBundle {
        material: materials.bullet_material.clone(),
        sprite: Sprite::new(Vec2::new(10., 10.)),
        ..Default::default()
    };

    cmds.spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player { speed: 1.5 })
}

fn player_move_system(
    mut players: Query<(&mut Player, &mut RigidBodyVelocity)>,

    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,

    keys: Res<Input<KeyCode>>,
) {
    let mut player_vel = Vec2::new(0.0, 0.0);

    if let Some(controller) = controller {
        let axis_lx = GamepadAxis(controller.0, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(controller.0, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            player_vel.x = x;
            player_vel.y = y;
        }
    } else {
        if keys.pressed(KeyCode::W) {
            player_vel.y = 1.;
        } else if keys.pressed(KeyCode::S) {
            player_vel.y = -1.;
        } else {
            player_vel.y = 0.;
        }
        if keys.pressed(KeyCode::D) {
            player_vel.x = 1.;
        } else if keys.pressed(KeyCode::A) {
            player_vel.x = -1.;
        } else {
            player_vel.x = 0.;
        }
    }

    for (mut player, mut velocity, mut position) in players.iter_mut() {
        velocity.linvel = player_vel.into();
    }
}

fn player_fire_aim_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut blaster_heat: ResMut<BlasterHeat>,

    //Why the with player? Check the tutorial
    mut players: Query<(&mut Player, &mut RigidBodyPosition), With<Player>>,

    mut send_fire_event: EventWriter<BulletFiredEvent>,

    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,

    mouse_buttons: Res<Input<MouseButton>>,
    win_size: Res<WindowSize>,
    windows: Res<Windows>,
) {
    let mut weapon_dir = Vec2::new(0.0, 0.0);

    let window = windows.get_primary().unwrap();

    if let Some(cursor) = window.cursor_position() {
        weapon_dir = Vec2::new(
            cursor.x - win_size.w / 2.0 - player_tf.translation.x,
            cursor.y - win_size.h / 2.0 - player_tf.translation.y,
        );
    }
    if let Some(controller) = controller {
        let normal_fire_button = GamepadButton(controller.0, GamepadButtonType::LeftTrigger);
        weapon_data.firing = buttons.pressed(normal_fire_button);

        let axis_rx = GamepadAxis(controller.0, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis(controller.0, GamepadAxisType::RightStickY);
        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            if x.abs() > 0.2 || y.abs() > 0.2 {
                weapon_dir = Vec2::new(x, y);
            }
        }
    }

    weapon_data.fire_rate_timer.tick(time.delta());
    blaster_heat.overheat_cooldown_timer.tick(time.delta());
    blaster_heat.value =
        0f32.max(blaster_heat.value - (time.delta_seconds() * BLASTER_COOLOFF_MULTIPLIER));

    if blaster_heat.value >= MAX_BLASTER_HEAT && !debug::is_overheat_disabled() {
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
