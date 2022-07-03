use bevy::prelude::*;
use rand::Rng;

use crate::components::{Civilian, Moveable, Player, Velocity};
use crate::resources::WindowSize;

pub struct CivilianPlugin;

impl Plugin for CivilianPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_civilian_system)
            .add_system(civilian_ai_system);
    }
}

fn spawn_civilian_system(mut cmds: Commands, win_size: Res<WindowSize>) {
    let mut rng = rand::thread_rng();
    let num_civilians = 5;

    for idx in 0..num_civilians {
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(40, 240, 40),
                custom_size: Some(Vec2::new(20., 20.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                    rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
                    1.9,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Civilian)
        .insert(Velocity(Vec2::new(0., 0.)))
        .insert(Moveable {
            solid: true,
            speed_multiplier: 0.25,
        });
    }
}

fn civilian_ai_system(
    mut civilian_query: Query<(&mut Velocity, &Transform), With<Civilian>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_tf = player_query.get_single().unwrap();

    for (mut civ_velocity, civ_tf) in civilian_query.iter_mut() {
        let position_diff = Vec2::new(
            player_tf.translation.x - civ_tf.translation.x,
            player_tf.translation.y - civ_tf.translation.y,
        );
        *civ_velocity = Velocity(position_diff);
    }
}
