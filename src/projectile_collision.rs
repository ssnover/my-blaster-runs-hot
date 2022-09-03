use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::*;

use crate::components::{Enemy, FromPlayer, Health, Lives, LivingBeing, Player};
use crate::constants::{KNOCKBACK_POWER, PLAYER_HEALTH};
use crate::states::GameState;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(on_living_being_hit)
                .with_system(on_living_being_dead)
                .with_system(on_knock_back),
        );
    }
}

//MAYBE REWRITE THIS TO TAKE DAMAGE, HEALTH AND LIVES
pub struct LivingBeingHitEvent {
    pub entity: Entity,
    pub damage: u32,
}

pub struct LivingBeingDeathEvent {
    pub entity: Entity,
}

pub struct KnockBackEvent {
    pub entity: Entity,
    pub direction: Vec2,
}

pub fn on_living_being_hit(
    mut living_being_hit_events: EventReader<LivingBeingHitEvent>,
    mut send_living_being_death: EventWriter<LivingBeingDeathEvent>,
    mut living_being: Query<(Entity, &mut Health), With<LivingBeing>>,
) {
    for event in living_being_hit_events.iter() {
        println!("In on_living_being_hit");

        for (being, mut health) in living_being.iter_mut() {
            if (being == event.entity) {
                println!("Found the entity hit");
                println!("health before damage: {}", health.health);

                health.health = health.health.saturating_sub(event.damage);
                println!("damage: {}", event.damage);
                println!("health: {}", health.health);
            }

            if health.health == 0 {
                send_living_being_death.send(LivingBeingDeathEvent {
                    entity: event.entity,
                })
            }
        }
    }
}

//MAYBE SEND A SPECIAL DEATH EVENT FOR PLAYERS SO I CAN INITIALIZE I-FRAME SEQUENCE
pub fn on_living_being_dead(
    mut living_being_death_events: EventReader<LivingBeingDeathEvent>,
    mut commands: Commands,
    mut living_being: Query<(Entity, &mut Lives, &mut Health), With<LivingBeing>>,
) {
    for event in living_being_death_events.iter() {
        println!("In on_living_being_dead");

        for (being, mut lives, mut health) in living_being.iter_mut() {
            if (being == event.entity) {
                lives.lives_num = lives.lives_num.saturating_sub(1);
                health.health = PLAYER_HEALTH;
            }

            if lives.lives_num == 0 {
                commands.entity(being).despawn_recursive();
            }
        }
    }
}

pub fn on_knock_back(
    mut knockback_events: EventReader<KnockBackEvent>,
    mut living_being: Query<(Entity, &mut ExternalImpulse), With<LivingBeing>>,
) {
    for event in knockback_events.iter() {
        println!("In on_knock_back");

        for (being, mut ext_impulse) in living_being.iter_mut() {
            println! {"being: {}, event.entity: {}", being.id(), event.entity.id()}
            if (being == event.entity) {
                println! {"event.direction: {}", event.direction}
                ext_impulse.impulse = event.direction.normalize() * KNOCKBACK_POWER;
            }
        }
    }
}

// pub fn death_by_height(
//     mut send_death_event: EventWriter<LivingBeingDeathEvent>,
//     living_being: Query<(Entity, &RigidBodyPosition), With<LivingBeing>>,
// ) {
//     for (entity, position) in living_being.iter() {
//         if position.position.translation.y < -1. {
//             send_death_event.send(LivingBeingDeathEvent { entity })
//         }
//     }
// }
