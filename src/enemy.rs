use bevy::prelude::*;
use rand::Rng;

use crate::components::{Despawnable, Moveable, Enemy, Velocity};
use crate::constants::{BASE_SPEED, SPRITE_SCALE, TIME_STEP};
use crate::resources::{GameTextures, WindowSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
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
        speed_multiplier: 1.,
        ..Default::default()
    });
}