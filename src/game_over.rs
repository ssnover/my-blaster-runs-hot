use crate::components::ScoreUi;
use crate::main_menu::{ButtonActive, UIAssets};
use crate::resources::PlayerScore;
use crate::states::GameState;

use bevy::app::AppExit;
use bevy::{prelude::*, ui::FocusPolicy};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(setup_menu)
                .with_system(despawn_all_non_ui))
            .add_system_set(SystemSet::on_pause(GameState::GameOver).with_system(despawn_menu))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(handle_quit_button),
            );
    }
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for entity in button_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn handle_quit_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Children, &mut ButtonActive, &Interaction),
        Changed<Interaction>,
    >,
    mut image_query: Query<&mut UiImage>,
    mut state: ResMut<State<GameState>>,
    mut app_exit: EventWriter<AppExit>,
    ui_assets: Res<UIAssets>,
) {
    for (children, mut active, interaction) in interaction_query.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Clicked => {
                if active.0 {
                    image.0 = ui_assets.button_pressed.clone();
                    app_exit.send(AppExit)
                }
            }
            Interaction::Hovered | Interaction::None => {
                image.0 = ui_assets.button.clone();
            }
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    score: Res<PlayerScore>,
    mut query: Query<&mut Text, With<ScoreUi>>,
) {
    let ui_assets = UIAssets {
        font: assets.load("FiraSans-Bold.ttf"),
        button: assets.load("button.png"),
        button_pressed: assets.load("button_pressed.png"),
    };

    let mut score_text = query.get_single_mut().unwrap();
    score_text.sections[0].value = format!("Score: {}", score.0);

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format!("Score: {}", score.0),
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 60.0,
                color: Color::GREEN,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform{   
            translation: Vec3::new(-100.0,-100.0,0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ButtonActive(true))
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    image: ui_assets.button.clone().into(),
                    ..Default::default()
                })
                .insert(FocusPolicy::Pass)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit Game",
                            TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        focus_policy: FocusPolicy::Pass,
                        ..Default::default()
                    });
                });
        });
    commands.insert_resource(ui_assets);
}

fn despawn_all_non_ui( mut commands: Commands, query: Query< Entity , Without<ScoreUi> >,){
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_all( mut commands: Commands, query: Query< Entity>, ){
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
