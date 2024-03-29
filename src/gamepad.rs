use bevy::prelude::*;

use crate::resources::Controller;
use crate::states::GameState;
pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainGame).with_system(gamepad_connection_system),
        );
    }
}

fn gamepad_connection_system(
    mut cmds: Commands,
    controller: Option<Res<Controller>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent {
        gamepad,
        event_type,
    } in gamepad_evr.iter()
    {
        match event_type {
            GamepadEventType::Connected => {
                println!("Controller connected!");
                if controller.is_none() {
                    cmds.insert_resource(Controller(*gamepad));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Controller disconnected!");
                if let Some(Controller(old_id)) = controller.as_deref() {
                    if old_id == gamepad {
                        cmds.remove_resource::<Controller>();
                    }
                }
            }
            _ => {}
        }
    }
}
