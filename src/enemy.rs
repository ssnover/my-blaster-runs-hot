use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;

use crate::components::{Enemy, Moveable, Player, Velocity};
use crate::constants::{BASE_SPEED, SPRITE_SCALE, TIME_STEP, ENEMY_REPULSION_FORCE, PLAYER_ATTRACTION_FORCE, ENEMY_REPULSION_RADIUS};
use crate::resources::{GameTextures, WindowSize};
use crate::utils::normalize_vec2;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system)
            .add_system(enemy_ai_system);
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
        .insert(Velocity::from(Vec2::new(0., 0.)))
        .insert(Moveable {
            //Slower than player
            solid: true,
            speed_multiplier: 1.,
            ..Default::default()
        });
    }
}

fn enemy_ai_system(
    mut cmds: Commands,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    query_player: Query<(&Transform), With<Player>>,
) {
    let player_tf = query_player.get_single().unwrap();
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
                if (tf.x - enemy_tf.translation.x).abs() < ENEMY_REPULSION_RADIUS{
                    x_offset += enemy_tf.translation.x/enemy_tf.translation.x.abs();
                }
                if (tf.y - enemy_tf.translation.y).abs() < ENEMY_REPULSION_RADIUS {
                    y_offset += enemy_tf.translation.x/enemy_tf.translation.x.abs();
                }
            }
        }

        let player_vel = Vec2::new(
            player_tf.translation.x - enemy_tf.translation.x,
            player_tf.translation.y - enemy_tf.translation.y,
        );
        let mut total_vel = PLAYER_ATTRACTION_FORCE*normalize_vec2(player_vel) - ENEMY_REPULSION_FORCE*Vec2::new(x_offset, y_offset);
        x_offset = 0.0;
        y_offset = 0.0;
        *enemy_vel = Velocity(total_vel);
    }
}
