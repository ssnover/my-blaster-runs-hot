use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashMap;
use rand::Rng;

use crate::components::{
    AreaOfEffect, Enemy, FromPlayer, Moveable, Player, Projectile, Size, Velocity, RangedWeapon,
};
use crate::constants::{
    BASE_SPEED, ENEMY_REPULSION_FORCE, ENEMY_REPULSION_RADIUS, PLAYER_ATTRACTION_FORCE,
    SPRITE_SCALE, TIME_STEP,
};
use crate::resources::{GameTextures, WindowSize};
use crate::utils::{normalize_vec2, CooldownTimer};
use crate::{PlayerScore, blaster};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system)
            .add_system(enemy_ai_system)
            .add_system(enemy_blaster_system);
    }
}

fn enemy_spawn_system(
    mut cmds: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
) {
    let mut rng = rand::thread_rng();

    // Add the enemy
    for i in 0..3 {
        cmds.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                //translation: Vec3::new( 200., 200., 0.),
                translation: Vec3::new(
                    rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                    rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
                    0.,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Size(Vec2::new(50., 50.)))
        .insert(Velocity::from(Vec2::new(0., 0.)))
        .insert(RangedWeapon {
            aim_direction: Vec2::new(1., 0.),
            fire_rate_timer: CooldownTimer::from_seconds(0.5),
            firing: true,
        })
        .insert(Moveable {
            //Slower than player
            solid: true,
            speed_multiplier: 0.5,
            ..Default::default()
        });
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
        *enemy_vel = Velocity(total_vel);

    }
}

fn enemy_blaster_system(
    mut cmds: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut RangedWeapon), With<Enemy>>,
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

            blaster::create_blaster_shot(
                &mut cmds,
                enemy_tf.translation,
                enemy_weapon.aim_direction,
                Color::rgb_u8(0, 0, 240),
                false,
            );
        }
    }   
}