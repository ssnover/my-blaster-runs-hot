use bevy::prelude::*;

use crate::components::ScoreUi;
use crate::resources::{GameFont, PlayerScore};
use crate::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(spawn_ui_system))
            .add_system_set(
                SystemSet::on_update(GameState::MainGame).with_system(update_score_system),
            );
    }
}

fn spawn_ui_system(mut cmds: Commands, score: Res<PlayerScore>, font: Res<GameFont>) {
    cmds.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.0.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            }),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    )
    .insert(ScoreUi);
}

fn update_score_system(score: Res<PlayerScore>, mut query: Query<&mut Text, With<ScoreUi>>) {
    let mut score_text = query.get_single_mut().unwrap();
    score_text.sections[0].value = format!("Score: {}", score.0);
}
