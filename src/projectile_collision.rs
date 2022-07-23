use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Enemy, FromPlayer, Moveable, Player, Projectile, Size, Velocity};
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(projectile_collision_and_score_system);
    }
}
pub fn projectile_collision_and_score_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &Size), With<Enemy>>,
    mut player_query: Query<(Entity, &Transform, &Size), With<Player>>,
    mut blaster_query: Query<(Entity, &Transform, &Size, &Projectile)>,
) {
    // check collision with objects
    for (projectile_entity, projectile_tf, projectile_size, projectile) in blaster_query.iter() {
        if projectile.from_player {
            check_collision_with_enemy(
                &mut commands,
                &enemy_query,
                projectile_entity,
                projectile_tf.translation,
                projectile_size.0,
            );
        }
        // } else {
        //     check_collision_with_player(
        //         &mut commands,
        //         &mut player_query,
        //         projectile,
        //         &collider_entity,
        //     );
        // }
    }
}

fn check_collision_with_enemy(
    cmds: &mut Commands,
    enemy_query: &Query<(Entity, &Transform, &Size), With<Enemy>>,
    project_entity: bevy::prelude::Entity,
    projectile_tf: Vec3,
    projectile_size: Vec2,
) {
    for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
        let collision = collide(
            enemy_tf.translation,
            enemy_size.0,
            projectile_tf,
            projectile_size,
        );
        if collision.is_some() {
            cmds.entity(enemy_entity).despawn_recursive();
            cmds.entity(project_entity).despawn_recursive();
            break;
        }
    }
}

// fn check_collision_with_player(
//     mut commands: Commands,
//     player_query: player_query: Query< (Entity, &Transform, &Size), With<Player> >,
//     projectile,
//     &collider_entity,
// ) {

// }

// fn enemy_despawn_system(
//     mut cmds: Commands,
//     enemy_query: Query<(Entity, &Transform, &Size), With<Enemy>>,
//     blaster_query: Query<(Entity, &Transform, &Size), (With<FromPlayer>, With<Projectile>)>,
//     mut score: ResMut<PlayerScore>,
// ) {
//     //I want to breakout this out into a plugin I think so it is easily usable for the player? Not sure but I don't want to leave this here
//     for (blaster_entity, blaster_tf, blaster_size) in blaster_query.iter() {
//         for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
//             let collision = collide(
//                 enemy_tf.translation,
//                 enemy_size.0,
//                 blaster_tf.translation,
//                 blaster_size.0,
//             );
//             if collision.is_some() {
//                 score.0 += 3;
//                 cmds.entity(enemy_entity).despawn_recursive();
//                 cmds.entity(blaster_entity).despawn_recursive();
//                 break;
//             }
//         }
//     }
// }
