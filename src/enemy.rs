use bevy::prelude::*;
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

//query_enemy: Query<(&Transform, &mut Velocity), With<Enemy>, Without<self????>>
//Not sure how to consider other enemies because add the above query creates a query conflict? Two queries getting the same thing = sad
fn enemy_ai_system(
    mut cmds: Commands,
    mut self_query: Query<(&mut Velocity, &Transform)>, 
    query_player: Query<(&Transform), With<Player>>
    
) {
    let player_tf = query_player.get_single().unwrap();

    for (mut self_vel, mut enemy_tf) in self_query.iter_mut() {
        let new_vel = Vec2::new(
            player_tf.translation.x - enemy_tf.translation.x,
            player_tf.translation.y - enemy_tf.translation.y
        );
        *self_vel = Velocity(normalize_vec2(new_vel));
    }
}