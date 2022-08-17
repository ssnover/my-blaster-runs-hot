use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_rapier2d::{prelude::*, rapier::prelude::Translation};
use nalgebra::MatrixSliceMut1x3;

use crate::components::{AreaOfEffect, Blaster, FromEnemy, FromPlayer, Projectile};
use crate::projectile_collision::{LivingBeing, LivingBeingHitEvent};
use crate::states::GameState;

pub struct BlasterFiredEvent {
    pub position: Vec2,
    pub direction: Vec2,
    pub from_player: bool,
}

pub struct BlasterPlugin;

impl Plugin for BlasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(on_blaster_fired)
                //.with_system(destroy_blaster_on_contact)
                .with_system(damage_on_contact),
        );
    }
}

pub fn on_blaster_fired(
    mut commands: Commands,
    mut bullet_fired_events: EventReader<BlasterFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        insert_blaster_at(&mut commands, event)
    }
}

pub fn insert_blaster_at(cmds: &mut Commands, options: &BlasterFiredEvent) {
    let speed = options.direction * 2.0;

    cmds.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(50.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::linear(speed))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            options.position.x,
            options.position.y,
            0.0,
        )))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::cuboid(20.0, 10.0))
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

//WOULD HAVE TO SEND DAMAGE AND LIVES IN THE EVENT SEND
//DAMAGE SHOULD BE IN WEAPON DATA
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
