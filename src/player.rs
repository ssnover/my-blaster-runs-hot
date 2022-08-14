use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use nalgebra::{vector, Vector2};

use crate::blaster::BlasterFiredEvent;
use crate::components::{AnimationTimer, Enemy, Lives, Player, Projectile, WeaponData};
use crate::constants::*;
use crate::debug;
use crate::projectile_collision::{LivingBeing, LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::resources::{BlasterHeat, Controller, GameTextures, WindowSize};
use crate::states::GameState;
use crate::utils::CooldownTimer;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<BlasterFiredEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame).with_system(player_spawn_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(player_move_system)
                    .with_system(player_fire_aim_system)
                    .with_system(debug_vel_system),
            );
    }
}

fn player_spawn_system(
    mut cmds: Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    let texture_handle = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(126.0, 39.0), 1, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    };

    cmds.spawn()
        .insert_bundle(sprite)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Player { speed: 100.0 })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(LivingBeing)
        .insert(Lives { lives_num: 5 })
        .insert(WeaponData {
            ..Default::default()
        });
}

fn player_move_system(
    mut players: Query<(Entity, &mut Velocity, &Player)>,

    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,

    keys: Res<Input<KeyCode>>,
) {
    let mut player_vel = Vec2::new(0.0, 0.0);

    if let Some(controller) = controller {
        let axis_lx = GamepadAxis::new(controller.0, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis::new(controller.0, GamepadAxisType::LeftStickY);

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

    for (mut player_entity, mut velocity, player) in players.get_single_mut() {
        *velocity = Velocity::linear(player_vel * player.speed);
    }
}

fn debug_vel_system(mut players: Query<(Entity, &mut Velocity, &Player)>) {
    for (mut player_entity, mut velocity, player) in players.get_single_mut() {
        println!("linvel = {}", &velocity.linvel);
    }
}

fn player_fire_aim_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut blaster_heat: ResMut<BlasterHeat>,

    mut player: Query<(Entity, &Transform, &mut WeaponData), With<Player>>,

    mut send_fire_event: EventWriter<BlasterFiredEvent>,

    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,

    mouse_buttons: Res<Input<MouseButton>>,
    win_size: Res<WindowSize>,
    windows: Res<Windows>,
) {
    let (player, player_tf, mut weapon) = player.get_single_mut().unwrap();
    let mut weapon_dir = Vec2::new(0.0, 0.0);
    let window = windows.get_primary().unwrap();

    if let Some(cursor) = window.cursor_position() {
        weapon_dir = Vec2::new(
            cursor.x - win_size.w / 2.0 - player_tf.translation.x,
            cursor.y - win_size.h / 2.0 - player_tf.translation.y,
        );
    }

    if let Some(controller) = controller {
        let normal_fire_button = GamepadButton::new(controller.0, GamepadButtonType::LeftTrigger);
        weapon.firing = buttons.pressed(normal_fire_button);

        let axis_rx = GamepadAxis::new(controller.0, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis::new(controller.0, GamepadAxisType::RightStickY);
        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            if x.abs() > 0.2 || y.abs() > 0.2 {
                weapon_dir = Vec2::new(x, y);
            }
        }
    }

    weapon.fire_rate_timer.tick(time.delta());
    blaster_heat.overheat_cooldown_timer.tick(time.delta());
    blaster_heat.value =
        0f32.max(blaster_heat.value - (time.delta_seconds() * BLASTER_COOLOFF_MULTIPLIER));

    if blaster_heat.value >= MAX_BLASTER_HEAT && !debug::is_overheat_disabled() {
        blaster_heat.overheat_cooldown_timer.trigger();
    }

    if weapon.firing
        && weapon.fire_rate_timer.ready()
        && blaster_heat.overheat_cooldown_timer.ready()
    {
        weapon.fire_rate_timer.trigger();
        blaster_heat.value += BLASTER_SHOT_HEAT_ADDITION;
        println!("Blaster Temp: {} C", blaster_heat.value);

        let event = BlasterFiredEvent {
            position: Vec2::new(player_tf.translation.x, player_tf.translation.y),
            direction: weapon.aim_direction,
            speed: 1.5,
            from_player: true,
        };
        println!("trying to send event");
        send_fire_event.send(event);
    }
}

pub fn collision_with_enemy(
    mut send_player_hit: EventWriter<LivingBeingHitEvent>,
    player_query: Query<(Entity, &Lives), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for (player, lives) in player_query.iter() {
                        for (enemy) in enemy_query.iter() {
                            if ((first == player) ^ (second == player)) {
                                if ((first == enemy) ^ (second == enemy)) {
                                    //send_player_hit.send(LivingBeingHitEvent { entity: player });
                                    lives.lives_num.saturating_sub(1);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
