use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Enemy, FromPlayer, Moveable, Player, Projectile, Size, Velocity};
use crate::states::GameState;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {

        app.add_system_set(SystemSet::on_update(GameState::MainGame)
            .with_system(projectile_collision_and_score_system));
    }
}
pub fn projectile_collision_and_score_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &Size), With<Enemy>>,
    mut player_query: Query<(Entity, &Transform, &Size), With<Player>>,
    mut blaster_query: Query<(Entity, &Transform, &Size, &Projectile)>,
    mut state: ResMut<State<GameState>>,
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
        } else {
            if(check_collision_with_player(
                &mut commands,
                &mut player_query,
                projectile_entity,
                projectile_tf.translation,
                projectile_size.0,
            )){
                
                state.push(GameState::GameOver).unwrap();
            };
        }
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

fn check_collision_with_player(
    cmds: &mut Commands,
    player_query: &Query<(Entity, &Transform, &Size), With<Player>>,
    project_entity: bevy::prelude::Entity,
    projectile_tf: Vec3,
    projectile_size: Vec2,
) -> bool {
    let (player_entity, player_tf, player_size) = player_query.get_single().unwrap();
    let collision = collide(
        player_tf.translation,
        player_size.0,
        projectile_tf,
        projectile_size,
    );
    if collision.is_some() {
        return true;
    }
    return false;
}
