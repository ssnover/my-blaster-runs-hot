use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
    AreaOfEffect, FromEnemy, FromPlayer, Moveable, Projectile, Size, Velocity,
};

pub struct BlasterFiredEvent {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32, //Should make this a constant
}

pub fn on_blaster_fired(
    mut commands: Commands,
    mut bullet_fired_events: EventReader<BulletFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        insert_bullet_at(&mut commands, event)
    }
}

pub fn insert_bullet_at(commands: &mut Commands, options: &BulletFiredEvent) {
    let speed = options.direction;

    let rigid_body = RigidBodyBundle {
        position: options.position,
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(
                options.speed * options.direction.x,
                options.speed * options.direction.y,
            ),
        },
        activation: RigidBodyActivation::cannot_sleep(),
        forces: RigidBodyForce {
            gravity_scale: 0.,
            ..Default::default()
        },
        ..Default::default()
    };

    let collider = CollisionBundle {
        shape: ColliderShape::rectangle(10.,10.),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    };

    let sprite = SpriteBundle {
        sprite: Sprite::new(Vec2::new(10., 10.)),
        ..Default::default()
    };

    commands
    .spawn_bundle(sprite)
    .insert_bundle(rigid_body)
    .insert_bundle(collider)
    .insert(RigidBodyPositionSync::Discrete)
    .insert(Blaster);
}

pub fn destroy_bullter_on_contact(
    mut commands: Commands,
    blasters: Query<Entity, With<Blaster>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for blaster in blasters.iter() {
                //Needs to be XOR because 2 bullets would be able to collide technicallydw
                if (h1.entity() == bullet ^ h2.entity() == bullet) {
                    commands.entity(blaster).despawn_recursive();
                }
            }
        }
    }
}

pub fn damage_on_contact(
    mut send_living_being_hit: EventWriter<LivingBeingHitEvent>,
    blasters: Query<Entity, With<Blaster>>,
    living_being: Query<Entity, With<LivingBeing>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_event.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for blaster in blasters.iter(){
                for being in living_being.iter() {
                    if ((h1.entity() == blaster && h2.entity() == being) 
                    || (h1.entity() == being && h2.entity() == blaster)) {
                        send_living_being_hit.send(LivingBeingHitEvent {entity: being});
                    }
                }
            }
        }
    }
}