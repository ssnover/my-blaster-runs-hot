use bevy::prelude::*;

use crate::resources::Controller;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connection_system);
    }
}

fn gamepad_connection_system(
    mut cmds: Commands,
    controller: Option<Res<Controller>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("Controller connected!");
                if controller.is_none() {
                    cmds.insert_resource(Controller(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Controller disconnected!");
                if let Some(Controller(old_id)) = controller.as_deref() {
                    if old_id == id {
                        cmds.remove_resource::<Controller>();
                    }
                }
            }
            _ => {}
        }
    }
}
