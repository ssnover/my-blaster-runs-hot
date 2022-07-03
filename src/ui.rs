use bevy::prelude::*;

use crate::components::ScoreUi;
use crate::resources::{GameFont, PlayerScore};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_ui_system)
            .add_system(update_score_system);
    }
}

fn spawn_ui_system(mut cmds: Commands, score: Res<PlayerScore>, font: Res<GameFont>) {
    cmds.spawn_bundle(UiCameraBundle::default());
    cmds.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text::with_section(
            format!("Score: {}", score.0),
            TextStyle {
                font: font.0.clone(),
                font_size: 20.,
                color: Color::GREEN,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..Default::default()
            },
        ),
        ..Default::default()
    })
    .insert(ScoreUi);
}

fn update_score_system(score: Res<PlayerScore>, mut query: Query<&mut Text, With<ScoreUi>>) {
    let mut score_text = query.get_single_mut().unwrap();
    score_text.sections[0].value = format!("Score: {}", score.0);
}
