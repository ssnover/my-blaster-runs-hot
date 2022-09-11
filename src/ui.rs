use bevy::prelude::*;

use crate::components::{PlayerLivesUI, RoundUI, ScoreUi};
use crate::resources::{GameFont, PlayerLives, PlayerScore};
use crate::rounds::RoundTracker;
use crate::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame).with_system(spawn_ui_system))
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(update_score_system)
                    .with_system(update_lives_system)
                    .with_system(update_round_system),
            );
    }
}

fn spawn_ui_system(mut cmds: Commands, score: Res<PlayerScore>, font: Res<GameFont>) {
    cmds.spawn_bundle(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: font.0.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        })])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    )
    .insert(ScoreUi);

    cmds.spawn_bundle(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: font.0.clone(),
            font_size: 40.0,
            color: Color::rgb(0.0, 1.0, 0.0),
        })])
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Percent(80.0),
                bottom: Val::Percent(0.0),
            },
            ..default()
        }),
    )
    .insert(PlayerLivesUI);

    cmds.spawn_bundle(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: font.0.clone(),
            font_size: 40.0,
            color: Color::rgb(1.0, 0.0, 1.0),
        })])
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(85.0),
                right: Val::Percent(0.0),
                top: Val::Percent(80.0),
                bottom: Val::Percent(0.0),
            },
            ..default()
        }),
    )
    .insert(RoundUI);
}

fn update_score_system(score: Res<PlayerScore>, mut query: Query<&mut Text, With<ScoreUi>>) {
    let mut score_text = query.get_single_mut().unwrap();
    score_text.sections[0].value = format!("Scorrrre: {}", score.0);
}

fn update_lives_system(lives: Res<PlayerLives>, mut query: Query<&mut Text, With<PlayerLivesUI>>) {
    let mut lives_text = query.get_single_mut().unwrap();
    lives_text.sections[0].value = format!("Lives: {}", lives.0);
}

fn update_round_system(round: Res<RoundTracker>, mut query: Query<&mut Text, With<RoundUI>>) {
    let mut round_text = query.get_single_mut().unwrap();
    round_text.sections[0].value = format!("Round: {}", round.current_round.unwrap());
}
