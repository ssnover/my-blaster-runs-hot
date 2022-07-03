use bevy::prelude::*;
use bevy::scene::serde::ENTITY_FIELD_ENTITY;
use bevy::utils::HashMap;
use rand::Rng;

use crate::components::{Despawnable, Moveable, Enemy, Velocity, Player};
use crate::constants::{BASE_SPEED, SPRITE_SCALE, TIME_STEP};
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
    for i in 0..5 {
        cmds.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform{
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                //translation: Vec3::new( 200., 200., 0.),
                translation: Vec3::new( rng.gen_range(-win_size.w/2.0 ..win_size.w/2.0), rng.gen_range(-win_size.h/2.0 ..win_size.h/2.0), 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Velocity::from(Vec2::new(0.,0.)))
        .insert(Moveable {
            //Slower than player
            solid: true,
            speed_multiplier: 1.,
            ..Default::default()
        });
    }
    
}

//query_enemy: Query<(&Transform, &mut Velocity), With<Enemy>, Without<self????>>
//Not sure how to consider other enemies because add the above query creates a query conflict? Two queries getting the same thing = sad
fn enemy_ai_system(
    mut cmds: Commands,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    query_player: Query<(&Transform), With<Player>>
    
) {
    let player_tf = query_player.get_single().unwrap();
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut entity_counter = 0.;

    let mut enemy_position = HashMap::default();

    for (entity, _, enemy_tf) in enemy_query.iter() {
        enemy_position.insert(entity, enemy_tf.clone());
        x_sum += enemy_tf.translation.x.clone();
        y_sum += enemy_tf.translation.y.clone();
        entity_counter += 1.;
    }

    for (entity, mut enemy_vel, enemy_tf) in enemy_query.iter_mut() {
        //This approach has to be naive
        
        // let new_vel = Vec2::new(
        //     player_tf.translation.x - enemy_tf.translation.x ,
        //     player_tf.translation.y - enemy_tf.translation.y
        // );
        let new_vel = Vec2::new(
            player_tf.translation.x - enemy_tf.translation.x - x_sum/entity_counter,
            player_tf.translation.y - enemy_tf.translation.y - y_sum/entity_counter
        );
        *enemy_vel = Velocity(normalize_vec2(new_vel));
    }

}