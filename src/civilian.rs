use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

use crate::components::{Civilian, Moveable, Player, Size, Velocity};
use crate::resources::{PlayerScore, WindowSize};
use crate::states::GameState;

pub struct CivilianPlugin;

impl Plugin for CivilianPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::MainGame)
            .with_system(civilian_ai_system)
            .with_system(civilian_despawn_system));
    }
}

pub fn spawn_civilian(cmds: &mut Commands, position: Vec2) {
    cmds.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(40, 240, 40),
            custom_size: Some(Vec2::new(20., 20.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(position.x, position.y, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Civilian)
    .insert(Velocity(Vec2::new(0., 0.)))
    .insert(Moveable {
        solid: true,
        speed_multiplier: 0.25,
    })
    .insert(Size(Vec2::new(20., 20.)));
}

fn spawn_civilian_system(mut cmds: Commands, win_size: Res<WindowSize>) {
    let mut rng = rand::thread_rng();
    let num_civilians = 5;

    for _ in 0..num_civilians {
        spawn_civilian(
            &mut cmds,
            Vec2::new(
                rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
            ),
        );
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

fn civilian_despawn_system(
    mut cmds: Commands,
    civilian_query: Query<(Entity, &Transform, &Size), With<Civilian>>,
    player_query: Query<(&Transform, &Size), With<Player>>,
    mut score: ResMut<PlayerScore>,
) {
    let (player_tf, player_size) = player_query.get_single().unwrap();

    for (entity, tf, civilian_size) in civilian_query.iter() {
        let collision = collide(
            player_tf.translation,
            player_size.0,
            tf.translation,
            civilian_size.0,
        );
        if collision.is_some() {
            // Rescued this civilian!
            score.0 += 100;
            cmds.entity(entity).despawn_recursive();
            println!("Current Score: {}", score.0);
        }
    }
}
