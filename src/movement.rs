use bevy::prelude::*;

use crate::components::{Despawnable, Moveable, Velocity};
use crate::constants::{BASE_SPEED, TIME_STEP};
use crate::resources::WindowSize;

const DESPAWN_MARGIN: f32 = 200.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system).add_system(despawn_system);
    }
}

fn movement_system(mut query: Query<(&Velocity, &Moveable, &mut Transform)>) {
    for (velocity, moveable, mut tf) in query.iter_mut() {
        tf.translation.x += velocity.x * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;
        tf.translation.y += velocity.y * TIME_STEP * BASE_SPEED * moveable.speed_multiplier;
    }
}

fn despawn_system(
    mut cmds: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Transform), With<Despawnable>>,
) {
    for (entity, tf) in query.iter() {
        if tf.translation.x < ((-win_size.w / 2.) - DESPAWN_MARGIN)
            || tf.translation.x > ((win_size.w / 2.) + DESPAWN_MARGIN)
            || tf.translation.y > ((win_size.h / 2.) + DESPAWN_MARGIN)
            || tf.translation.y < ((-win_size.h / 2.) + DESPAWN_MARGIN)
        {
            cmds.entity(entity).despawn_recursive();
        }
    }
}
