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
    mut query: Query<(Entity, &Velocity, &Moveable, &mut Transform, &Size)>,
) {
    for (entity, velocity, moveable, mut tf, size) in query.iter_mut() {
        
        let velocity = normalize_vec2(velocity.0);

        let x_position_delta = velocity.x * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;
        let y_position_delta = velocity.y * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;

        let x_position =
        (tf.translation.x + x_position_delta).min(win_size.w / 2. - MOVEMENT_BOUND_MARGIN);

        let y_position =
        (tf.translation.y + y_position_delta).min(win_size.h / 2. - MOVEMENT_BOUND_MARGIN);

        let mut target = tf.translation.clone();
        target.x = x_position.max(-win_size.w / 2. + MOVEMENT_BOUND_MARGIN);
        target.y = y_position.max(-win_size.h / 2. + MOVEMENT_BOUND_MARGIN);

        if (moveable.solid) {

            for (entity2, velocity2, moveable2, tf2, size2) in query.iter() {
                if (entity != entity2){
                    if(collision_check(&target, size, &tf2.translation, size2)) {
                        tf.translation.x = x_position.max(-win_size.w / 2. + MOVEMENT_BOUND_MARGIN);
                    }
                    if(collision_check(&target, size, &tf2.translation, size2)) {
                        tf.translation.y = y_position.max(-win_size.h / 2. + MOVEMENT_BOUND_MARGIN);
                    }
                }
            }
        }
        else {
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
    target_pos: &Vec3,
    target_size: &Size,
    different_pos: &Vec3,
    different_size: &Size,
) -> bool {
        let collision = collide(
            *target_pos,
            target_size.0,
            *different_pos,
            different_size.0
        );

        if collision.is_some() {
            return false;
        }
    
    return true;
}