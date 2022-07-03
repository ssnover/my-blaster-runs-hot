use bevy::prelude::*;

use crate::components::{FromEnemy, FromPlayer, Moveable, NormalBlasterFire, Velocity, Size};

pub fn create_blaster_shot(
    cmds: &mut Commands,
    position: Vec3,
    direction: Vec2,
    color: Color,
    from_player: bool,
) {
    let mut entity_cmds = cmds.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(20., 20.)),
            ..Default::default()
        },
        transform: Transform {
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    });
    entity_cmds
        .insert(Size(Vec2::new(50.,50.)))
        .insert(NormalBlasterFire)
        .insert(Velocity::from(direction))
        .insert(Moveable {
            solid: false,
            speed_multiplier: 1.5,
        });
    if from_player {
        entity_cmds.insert(FromPlayer);
    } else {
        entity_cmds.insert(FromEnemy);
    }
}
