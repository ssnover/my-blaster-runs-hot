use bevy::prelude::{Camera2dBundle, Commands};

/// Creates the default camera for the game.
///
/// # Arguments
///
/// * `commands` - A list of commands used to modify a `World`.
pub fn spawn_ui_camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
