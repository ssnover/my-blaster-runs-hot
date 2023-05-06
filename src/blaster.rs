use bevy::prelude::*;
use bevy::utils::tracing::span::AsId;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_rapier2d::{prelude::*, rapier::prelude::Translation};
use nalgebra::MatrixSliceMut1x3;

use crate::components::{AreaOfEffect, Blaster, FromEnemy, FromPlayer, Health, Lives, LivingBeing};
use crate::constants::{BLASTER_GROUP, BLASTER_SPEED};
use crate::player;
use crate::projectile_collision::LivingBeingHitEvent;
use crate::states::GameState;

pub struct BlasterFiredEvent {
    pub position: Vec2,
    pub direction: Vec2,
    pub from_player: bool,
    pub memberships: u32,
    pub filter: u32,
    pub color: Color,
}

pub struct BlasterPlugin;

impl Plugin for BlasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(on_blaster_fired)
                .with_system(destroy_blaster_on_contact)
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
    let speed = options.direction.normalize() * BLASTER_SPEED;

    cmds.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: options.color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        //Rigid Body
        .insert(RigidBody::KinematicVelocityBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::linear(speed))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            options.position.x,
            options.position.y,
            0.0,
        )))
        //Collider
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Dominance::group(-1))
        .insert(CollisionGroups::new(
            (options.memberships),
            (options.filter),
        ))
        //Custom Functionality
        .insert(Blaster { damage: 1 });
}

pub fn destroy_blaster_on_contact(
    mut commands: Commands,
    blaster_query: Query<(Entity, &Blaster)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for event in contact_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                let mut blasters = Vec::new();

                for (blaster, blaster_info) in blaster_query.iter() {
                    blasters.push(blaster.id());
                }

                if flags == &CollisionEventFlags::empty() {
                    //FIX THIS CODE THIS DOES NOT DO ANYTHING
                    for (blaster, blaster_info) in blaster_query.iter() {
                        if (first == blaster) {
                            if (!blasters.contains(&second.id())) {
                                commands.entity(blaster).despawn_recursive();
                            }
                        }
                        if (second == blaster) {
                            if (!blasters.contains(&first.id())) {
                                commands.entity(blaster).despawn_recursive();
                            }
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
    blasters: Query<(Entity, &Blaster)>,
    mut living_being: Query<(Entity), With<LivingBeing>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for event in contact_events.iter() {
        match event {
            CollisionEvent::Started(first, second, flags) => {
                let first = *first;
                let second = *second;

                if flags == &CollisionEventFlags::empty() {
                    for (blaster_entity, blaster) in blasters.iter() {
                        for (being) in living_being.iter() {
                            if ((first == blaster_entity && second == being)
                                || (first == being && second == blaster_entity))
                            {
                                send_living_being_hit.send(LivingBeingHitEvent {
                                    entity: being,
                                    damage: blaster.damage,
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
