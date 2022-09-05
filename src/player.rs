use std::sync::{Arc, Mutex};

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use nalgebra::{vector, Vector2};

use crate::blaster::BlasterFiredEvent;
use crate::components::{
    AnimationTimer, Direction, Enemy, Health, Lives, LivingBeing, Player, WeaponData,
};
use crate::constants::*; //Should probably fix this, it's a little lazy
use crate::debug;
use crate::projectile_collision::{KnockBackEvent, LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::resources::{BlasterHeat, Controller, GameTextures, PlayerLives, WindowSize};
use crate::states::{AnimationInfo, GameState, PlayerState, SpriteLocation};
use crate::utils::CooldownTimer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<BlasterFiredEvent>()
            .add_event::<KnockBackEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame).with_system(player_spawn_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(player_move_system)
                    .with_system(player_fire_aim_system)
                    .with_system(collision_with_enemy)
                    .with_system(display_lives_ui),
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

    // let texture_handle = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(126.0, 39.00), 1, 8);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_handle =
        asset_server.load("darians-assets/TeamGunner/CHARACTER_SPRITES/Blue/Blue_Soldier.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        ..default()
    };

    cmds.spawn()
        .insert_bundle(sprite)
        //Rigid Body
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        //Collider
        .insert(Collider::cuboid(PLAYER_WIDTH, PLAYER_HEIGHT))
        .insert(ActiveCollisionTypes::all())
        .insert(ColliderMassProperties::Density(1.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            (PLAYER_GROUP | CIVILLIAN_GROUP | PHYSICAL_GROUP),
            (PLAYER_GROUP | CIVILLIAN_GROUP | PHYSICAL_GROUP),
        ))
        //Custom functionality
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(LivingBeing)
        .insert(Player {
            speed: PLAYER_SPEED,
        })
        .insert(Health {
            health: PLAYER_HEALTH,
        })
        .insert(Lives {
            lives_num: PLAYER_LIVES,
        })
        .insert(WeaponData {
            ..Default::default()
        })
        .insert(AnimationInfo {
            state: Arc::new(Mutex::new(PlayerState::Idle)),
        })
        .insert(Direction { is_Right: true });
}

fn player_move_system<T: SpriteLocation + Send + Sync + 'static>(
    mut players: Query<(Entity, &mut Velocity, &Player, &mut AnimationInfo<T>)>,

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

    for (mut player_entity, mut velocity, player, mut player_animation_info) in
        players.get_single_mut()
    {
        *velocity = Velocity::linear(player_vel * player.speed);
        if (velocity.linvel == Vec2 { x: 0.0, y: 0.0 }) {
            *player_animation_info.state.lock().unwrap() = PlayerState::Idle;
        } else {
            player_animation_info.state = ;
        }
    }
}

fn player_fire_aim_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut blaster_heat: ResMut<BlasterHeat>,

    mut player: Query<(Entity, &Transform, &mut WeaponData, &mut Direction), With<Player>>,

    mut send_fire_event: EventWriter<BlasterFiredEvent>,

    controller: Option<Res<Controller>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,

    mouse_buttons: Res<Input<MouseButton>>,
    win_size: Res<WindowSize>,
    windows: Res<Windows>,
) {
    let (player, player_tf, mut weapon, mut player_dir) = player.get_single_mut().unwrap();
    let mut weapon_dir = Vec2::new(0.0, 0.0);
    let window = windows.get_primary().unwrap();

    if let Some(cursor) = window.cursor_position() {
        weapon_dir = Vec2::new(
            cursor.x - win_size.w / 2.0 - player_tf.translation.x,
            cursor.y - win_size.h / 2.0 - player_tf.translation.y,
        );
    }

    weapon.firing = mouse_buttons.pressed(MouseButton::Left);

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
            direction: weapon_dir,
            from_player: true,
            memberships: ENEMY_GROUP,
            filter: ENEMY_GROUP,
            color: Color::rgb(0.0, 0.0, 1.0),
        };
        send_fire_event.send(event);
    }

    if weapon_dir.x > 0.0 {
        player_dir.is_Right = true;
    } else {
        player_dir.is_Right = false;
    }
}

pub fn collision_with_enemy(
    mut send_player_hit: EventWriter<LivingBeingDeathEvent>,
    mut send_knockback_event: EventWriter<KnockBackEvent>,
    player_query: Query<(Entity, &Lives, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for (player, lives, player_tf) in player_query.iter() {
                        for (enemy, enemy_tf) in enemy_query.iter() {
                            if ((first == player) && (second == enemy))
                                || ((first == enemy) && (second == player))
                            {
                                let knock_back_direction = Vec2::new(
                                    player_tf.translation.x - enemy_tf.translation.x,
                                    player_tf.translation.y - enemy_tf.translation.y,
                                );
                                send_player_hit.send(LivingBeingDeathEvent { entity: player });
                                send_knockback_event.send(KnockBackEvent {
                                    entity: player,
                                    direction: knock_back_direction,
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn display_lives_ui(
    player_query: Query<&Lives, With<Player>>,
    mut res_lives: ResMut<PlayerLives>,
) {
    let curr_lives = player_query.get_single().unwrap();
    res_lives.0 = curr_lives.lives_num;
}
