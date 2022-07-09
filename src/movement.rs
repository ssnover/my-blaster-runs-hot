use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::{Moveable, Velocity, Enemy, Size};
use crate::constants::{BASE_SPEED, TIME_STEP};
use crate::resources::WindowSize;
use crate::utils::normalize_vec2;

const DESPAWN_MARGIN: f32 = 200.;
const MOVEMENT_BOUND_MARGIN: f32 = 50.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system)
            .add_system(despawn_out_of_bounds_system);
    }
}

fn movement_system(
    win_size: Res<WindowSize>,
    mut query: Query<(&Velocity, &Moveable, &mut Transform, &Size)>,
    mut enemy_query: Query<(Entity, &Transform, &Size), With<Enemy>>,
) {
    for (velocity, moveable, mut tf, size) in query.iter_mut() {
        let velocity = normalize_vec2(velocity.0);

        let x_position_delta = velocity.x * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;
        let y_position_delta = velocity.y * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;

        if (moveable.solid) {
            let x_position =
                (tf.translation.x + x_position_delta).min(win_size.w / 2. - MOVEMENT_BOUND_MARGIN);
            let mut target = tf.translation.clone();
            target.x = x_position.max(-win_size.w / 2. + MOVEMENT_BOUND_MARGIN);
            if(collision_check(target, size, &enemy_query)) {
                tf.translation.x = x_position.max(-win_size.w / 2. + MOVEMENT_BOUND_MARGIN);
            }
            
            let y_position =
                (tf.translation.y + y_position_delta).min(win_size.h / 2. - MOVEMENT_BOUND_MARGIN);
            let mut target = tf.translation.clone();
            target.y = y_position.max(-win_size.h / 2. + MOVEMENT_BOUND_MARGIN);
            if(collision_check(target, size, &enemy_query)) {
                tf.translation.y = y_position.max(-win_size.h / 2. + MOVEMENT_BOUND_MARGIN);
            }

        } else {
            tf.translation.x += x_position_delta;
            tf.translation.y += y_position_delta;
        }
    }
}

fn despawn_out_of_bounds_system(
    mut cmds: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Transform), With<Moveable>>,
) {
    for (entity, tf) in query.iter() {
        if tf.translation.x < ((-win_size.w / 2.) - DESPAWN_MARGIN)
            || tf.translation.x > ((win_size.w / 2.) + DESPAWN_MARGIN)
            || tf.translation.y > ((win_size.h / 2.) + DESPAWN_MARGIN)
            || tf.translation.y < ((-win_size.h / 2.) - DESPAWN_MARGIN)
        {
            cmds.entity(entity).despawn_recursive();
        }
    }
}

fn collision_check(
    target_pos: Vec3,
    entity_size: &Size,
    mut enemy_query: &Query<(Entity, &Transform, &Size), With<Enemy>>,
) -> bool {
    for (enemy, enemy_tf, enemy_size) in enemy_query.iter() {
        let collision = collide(
            target_pos,
            entity_size.0,
            enemy_tf.translation,
            enemy_size.0
        );

        if collision.is_some() {
            return false;
        }
    }
    return true;
}