use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_rapier2d::{prelude::*, rapier::prelude::Translation};
use nalgebra::MatrixSliceMut1x3;

use crate::components::{AreaOfEffect, Blaster, FromEnemy, FromPlayer, Moveable, Projectile, Size};
use crate::projectile_collision::{LivingBeing, LivingBeingHitEvent};

pub struct BlasterFiredEvent {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32, //Should make this a constant
    pub from_player: bool,
}

// pub fn on_blaster_fired(
//     mut commands: Commands,
//     mut bullet_fired_events: EventReader<BlasterFiredEvent>,
// ) {
//     for event in bullet_fired_events.iter() {
//         insert_bullet_at(commands, event)
//     }
// }

pub fn insert_bullet_at(mut cmds: Commands, options: &BlasterFiredEvent) {
    let speed = options.direction;

    let sprite = Sprite {
        color: Color::rgb(0.1, 0.1, 0.5),
        custom_size: Some(Vec2::new(20.0, 10.0)),
        ..Default::default()
    };

    cmds.spawn()
        .insert(sprite)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            options.position.x,
            options.position.y,
            0.0,
        )))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::cuboid(20.0, 10.0))
        .insert(LivingBeing)
        .insert(Blaster);
}

pub fn destroy_blaster_on_contact(
    mut commands: Commands,
    blasters: Query<Entity, With<Blaster>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for event in contact_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for blaster in blasters.iter() {
                        //Needs to be XOR because 2 bullets would be able to collide technicallydw
                        if ((first == blaster) ^ (second == blaster)) {
                            commands.entity(blaster).despawn_recursive();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn damage_on_contact(
    mut commands: Commands,
    mut send_living_being_hit: EventWriter<LivingBeingHitEvent>,
    blasters: Query<Entity, With<Blaster>>,
    living_being: Query<Entity, With<LivingBeing>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for event in contact_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for blaster in blasters.iter() {
                        for being in living_being.iter() {
                            if ((first == blaster && second == being)
                                || (first == being && second == blaster))
                            {
                                //I can add a damage component here for some future game reference
                                //Also I need to differentiate between enemy bullets and enemies + player bullets and players
                                send_living_being_hit.send(LivingBeingHitEvent { entity: being });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
