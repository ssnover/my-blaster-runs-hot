use bevy::prelude::*;
use serde::Deserialize;
use std::collections::VecDeque;
use std::path::PathBuf;

use crate::components::{Civilian, Enemy};
use crate::resources::{SpawnQueue, SpawnType};
use crate::spawn_manager::NewRoundEvent;
use crate::states::GameState;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct RoundSpawns {
    pub max_spawns: u32,
    pub number_of_civilians: u32,
    pub number_of_crabs: u32,
    pub number_of_bots: u32,
    pub number_of_tanks: u32,
    pub number_of_exploders: u32,
}

pub struct PopulateQueueEvent {}

fn parse_round_spawns(
    round_spawn_path: PathBuf,
) -> Result<Vec<RoundSpawns>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(round_spawn_path)?;
    let round_data: Vec<RoundSpawns> = reader
        .deserialize()
        .filter(|entry| entry.is_ok())
        .map(|line| line.unwrap())
        .collect();
    Ok(round_data)
}

pub struct RoundTracker {
    pub current_round: Option<u32>,
    round_data: Vec<RoundSpawns>,
}

impl RoundTracker {
    pub fn start() -> Self {
        let mut tracker = RoundTracker {
            current_round: None,
            round_data: parse_round_spawns(PathBuf::from("assets/round_data.txt")).unwrap(),
        };
        println!("{:?}", tracker.current_round_data());
        tracker.current_round = Some(0);
        tracker
    }

    pub fn next_round(&mut self) -> bool {
        if let Some(mut current_round) = self.current_round {
            current_round += 1;
            self.current_round = Some(current_round);
            if current_round as usize >= self.number_of_rounds() {
                return false;
            }
        }
        true
    }

    pub fn number_of_rounds(&self) -> usize {
        self.round_data.len()
    }

    pub fn current_round_data(&self) -> Option<RoundSpawns> {
        if let Some(current) = self.current_round {
            Some(self.round_data[current as usize])
        } else {
            None
        }
    }
}

pub struct RoundManagerPlugin;

impl Plugin for RoundManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PopulateQueueEvent>()
            .add_event::<NewRoundEvent>()
            .add_startup_system(insert_startup_resources)
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame).with_system(start_round_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame).with_system(populate_spawn_queue_system),
            );
    }
}

fn insert_startup_resources(mut cmds: Commands) {
    cmds.insert_resource(SpawnQueue(VecDeque::new()));
    cmds.insert_resource(RoundTracker::start());
}

fn start_round_system(
    mut cmds: Commands,
    mut send_populate_queue: EventWriter<PopulateQueueEvent>,
) {
    send_populate_queue.send(PopulateQueueEvent {});
    println!("here in start round");
}

fn populate_spawn_queue_system(
    mut populate_queue_events: EventReader<PopulateQueueEvent>,
    mut spawn_queue: ResMut<SpawnQueue>,
    mut round_tracker: ResMut<RoundTracker>,
) {
    if populate_queue_events.len() > 0 {
        println!("I was called");

        if (spawn_queue.len() == 0) {
            let round_data = round_tracker.current_round_data().unwrap();
            for _ in 0..round_data.number_of_civilians {
                spawn_queue.push_back(SpawnType::Civilian);
            }
            for _ in 0..round_data.number_of_crabs {
                spawn_queue.push_back(SpawnType::Crab);
            }
        }
        println!("spawn_queue.len(): {}", spawn_queue.len());
        populate_queue_events.clear();
    }
}
