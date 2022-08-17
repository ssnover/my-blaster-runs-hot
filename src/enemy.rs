use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::blaster::BlasterFiredEvent;
use crate::components::{AreaOfEffect, Enemy, FromPlayer, Health, Player, Projectile, WeaponData};
use crate::constants::{
    ENEMY_REPULSION_FORCE, ENEMY_REPULSION_RADIUS, ENEMY_SPRITE_SCALE, PLAYER_ATTRACTION_FORCE,
    TIME_STEP,
};
use crate::projectile_collision::LivingBeing;
use crate::resources::{GameTextures, WindowSize};
use crate::states::GameState;
use crate::utils::{normalize_vec2, CooldownTimer};
use crate::{blaster, PlayerScore};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
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

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::new(ENEMY_SPRITE_SCALE, ENEMY_SPRITE_SCALE, 1.),
            //translation: Vec3::new( 200., 200., 0.),
            translation: Vec3::new(0.0, 0.0, 0.1),
            ..Default::default()
        },
        ..Default::default()
    };

    cmds.spawn_bundle(sprite)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LivingBeing)
        .insert(Enemy);
}

fn enemy_spawn_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    win_size: Res<WindowSize>,
) {
    let mut rng = rand::thread_rng();

    let texture_handle = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(126.0, 39.0), 1, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the enemy
    for i in 0..3 {
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
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let mut entity_counter = 0.;

    let mut enemy_position = HashMap::default();

    for (entity, _, enemy_tf) in enemy_query.iter() {
        enemy_position.insert(entity, enemy_tf.translation);
    }

    for (entity, mut enemy_vel, enemy_tf) in enemy_query.iter_mut() {
        //These random constants need to be changed and we need some way to know that the enemy cannot be the position of other enemies
        //But it all diverges to player eventually
        for (enemy, tf) in &enemy_position {
            if entity != *enemy {
                if (tf.x - enemy_tf.translation.x).abs() < ENEMY_REPULSION_RADIUS {
                    x_offset += (enemy_tf.translation.x / enemy_tf.translation.x.abs())
                        / (tf.x - enemy_tf.translation.x);
                }
                if (tf.y - enemy_tf.translation.y).abs() < ENEMY_REPULSION_RADIUS {
                    y_offset += enemy_tf.translation.y
                        / enemy_tf.translation.y.abs()
                        / (tf.y - enemy_tf.translation.y);
                }
            }
        }

        let player_vel = Vec2::new(
            player_tf.translation.x - enemy_tf.translation.x,
            player_tf.translation.y - enemy_tf.translation.y,
        );
        let mut total_vel = PLAYER_ATTRACTION_FORCE * normalize_vec2(player_vel)
            - ENEMY_REPULSION_FORCE * Vec2::new(x_offset, y_offset);
        x_offset = 0.0;
        y_offset = 0.0;
        enemy_vel.linvel = total_vel;
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

    for (entity, enemy_tf, mut enemy_weapon) in enemy_query.iter_mut() {
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
            };
            send_fire_event.send(event);
        }
    }
}
