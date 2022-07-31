use bevy::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

use crate::components::{Civilian, Enemy};
use crate::resources::{SpawnQueue, SpawnType};

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct RoundSpawns {
    pub max_spawns: u32,
    pub number_of_civilians: u32,
    pub number_of_crabs: u32,
    pub number_of_bots: u32,
    pub number_of_tanks: u32,
    pub number_of_exploders: u32,
}

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
        tracker.next_round();
        tracker
    }

    pub fn next_round(&mut self) -> bool {
        if let Some(mut current_round) = self.current_round {
            current_round += 1;
            if current_round as usize >= self.number_of_rounds() {
                return false;
            }
        } else {
            self.current_round = Some(0)
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
        app.add_startup_system_to_stage(StartupStage::PostStartup, start_round_system)
            .add_system(round_manager_system);
    }
}

fn start_round_system(mut cmds: Commands) {
    cmds.insert_resource(RoundTracker::start());
}

fn round_manager_system(
    mut spawn_queue: ResMut<SpawnQueue>,
    mut round_tracker: ResMut<RoundTracker>,
    query: Query<(), Or<(With<Enemy>, With<Civilian>)>>,
) {
    let spawn_count: usize = query.iter().count();

    if spawn_queue.len() == 0 && spawn_count == 0 {
        if round_tracker.next_round() {
            let round_data = round_tracker.current_round_data().unwrap();
            for _ in 0..round_data.number_of_civilians {
                spawn_queue.push_back(SpawnType::Civilian);
            }
            for _ in 0..round_data.number_of_crabs {
                spawn_queue.push_back(SpawnType::Crab);
            }
        }
    }
}
