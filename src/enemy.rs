use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::parry::either::Either::Right;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::blaster::BlasterFiredEvent;
use crate::components::{
    AnimationTimer, AreaOfEffect, Enemy, FromPlayer, Health, Lives, LivingBeing, Player, WeaponData,
};
use crate::constants::{
    ENEMY_GROUP, ENEMY_REPULSION_FORCE, ENEMY_REPULSION_RADIUS, ENEMY_SPRITE_SCALE,
    PLAYER_ATTRACTION_FORCE, PLAYER_GROUP, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_SPRITE_SCALE,
    PLAYER_WIDTH, TIME_STEP,
};
use crate::projectile_collision::{LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::resources::{GameTextures, WindowSize};
use crate::states::GameState;
use crate::utils::{normalize_vec2, CooldownTimer};
use crate::{blaster, PlayerScore};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<BlasterFiredEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame).with_system(enemy_spawn_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(enemy_ai_system)
                    .with_system(enemy_blaster_system),
            );
    }
}

pub fn spawn_crab(
    cmds: &mut Commands,
    position: Vec2,
    texture_atlas_handle: &Handle<TextureAtlas>,
) {
    //Ripped my own code from the animation branch
    // Add the enemy sprites I think I want to break this out into a component? With a bunch of parts that we can call in different systems even at startup

    let transform = Transform {
        translation: Vec3::new(position.x, position.y, 0.0),
        scale: Vec3::splat(PLAYER_SPRITE_SCALE),
        ..default()
    };

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: transform,
        ..default()
    };

    cmds.spawn_bundle(sprite)
        //Rigid Body
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        //Collider
        .insert(Collider::cuboid(PLAYER_WIDTH, PLAYER_HEIGHT))
        .insert(ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(ENEMY_GROUP, ENEMY_GROUP))
        //Custom functionality
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(LivingBeing)
        .insert(Health { health: 1 })
        .insert(Lives { lives_num: 1 })
        .insert(Enemy)
        .insert(WeaponData {
            firing: true,
            ..Default::default()
        });
}

fn enemy_spawn_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    win_size: Res<WindowSize>,
) {
    let mut rng = rand::thread_rng();

    let texture_handle =
        asset_server.load("darians-assets/TeamGunner/CHARACTER_SPRITES/Red/Gunner_Red_Run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the enemy
    for i in 0..2 {
        spawn_crab(
            &mut cmds,
            Vec2::new(
                rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
            ),
            &texture_atlas_handle,
        );
    }
}

fn enemy_ai_system(
    mut cmds: Commands,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    player_query: Query<(&Transform), With<Player>>,
) {
    let player_tf = player_query.get_single().unwrap();

    for (enemy, mut enemy_velocity, enemy_tf) in enemy_query.iter_mut() {
        let position_diff = Vec2::new(
            player_tf.translation.x - enemy_tf.translation.x,
            player_tf.translation.y - enemy_tf.translation.y,
        );

        enemy_velocity.linvel = position_diff.normalize() * 0.0;
    }
}

fn enemy_blaster_system(
    mut cmds: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut WeaponData), With<Enemy>>,
    mut send_fire_event: EventWriter<BlasterFiredEvent>,
    player_query: Query<(&Transform), With<Player>>,
    time: Res<Time>,
) {
    let player_tf = player_query.get_single().unwrap();

    for (enemy, enemy_tf, mut enemy_weapon) in enemy_query.iter_mut() {
        enemy_weapon.fire_rate_timer.tick(time.delta());

        if enemy_weapon.firing && enemy_weapon.fire_rate_timer.ready() {
            enemy_weapon.fire_rate_timer.trigger();

            enemy_weapon.aim_direction = Vec2::new(
                player_tf.translation.x - enemy_tf.translation.x,
                player_tf.translation.y - enemy_tf.translation.y,
            );

            let event = BlasterFiredEvent {
                position: Vec2::new(enemy_tf.translation.x, enemy_tf.translation.y),
                direction: enemy_weapon.aim_direction,
                from_player: false,
                memberships: PLAYER_GROUP,
                filter: PLAYER_GROUP,
                color: Color::rgb(1.0, 0.0, 0.0),
            };
            send_fire_event.send(event);
        }
    }
}
