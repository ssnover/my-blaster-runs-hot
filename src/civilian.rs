use bevy::ecs::event::Event;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use rand::Rng;

use crate::components::{AnimationTimer, Civilian, Player};
use crate::constants::PLAYER_SPRITE_SCALE;
use crate::projectile_collision::{LivingBeing, LivingBeingHitEvent};
use crate::resources::{PlayerScore, WindowSize};
use crate::states::GameState;

pub struct CivilianPlugin;

impl Plugin for CivilianPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainGame).with_system(civilian_ai_system),
            //
            //.with_system(civilian_despawn_system),
        );
    }
}

pub fn spawn_civilian(
    cmds: &mut Commands,
    position: Vec2,
    texture_atlas_handle: &Handle<TextureAtlas>,
) {
    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 1.),
            //translation: Vec3::new( 200., 200., 0.),
            translation: Vec3::new(0.0, 0.0, 0.1),
            ..Default::default()
        },
        ..Default::default()
    };

    cmds.spawn()
        .insert_bundle(sprite)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(LivingBeing)
        .insert(Civilian);
}

fn spawn_civilian_system(
    mut cmds: Commands,
    win_size: Res<WindowSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let num_civilians = 5;

    let texture_handle = asset_server.load("darians-assets/Ball and Chain Bot/run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(126.0, 39.0), 1, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..num_civilians {
        spawn_civilian(
            &mut cmds,
            Vec2::new(
                rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0),
                rng.gen_range(-win_size.h / 2.0..win_size.h / 2.0),
            ),
            &texture_atlas_handle,
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
        civ_velocity.linvel = position_diff;
    }
}

fn civilian_despawn_system(
    mut cmds: Commands,
    mut send_civillian_hit: EventWriter<LivingBeingHitEvent>,
    civilian_query: Query<Entity, With<Civilian>>,
    player_query: Query<Entity, With<Player>>,
    mut score: ResMut<PlayerScore>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    let player = player_query.get_single().unwrap();
    for event in contact_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for (civillian) in civilian_query.iter() {
                        //I think this will work because there is only 1 player, I guess this would work with more than 1 player
                        if ((first == player) ^ (second == player)) {
                            if ((first == civillian) ^ (second == civillian)) {
                                cmds.entity(civillian).despawn_recursive();
                                score.0 += 100;
                                println!("Current Score: {}", score.0);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    //     if collision.is_some() {
    //         // Rescued this civilian!
    //         score.0 += 100;
    //         cmds.entity(entity).despawn_recursive();
    //         println!("Current Score: {}", score.0);
    //     }
    // }
}
