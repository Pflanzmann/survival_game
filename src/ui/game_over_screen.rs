use bevy::ecs::event::Events;
use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, FlexDirection, Interaction, JustifyContent, NextState, NodeBundle, PositionType, Query, Res, ResMut, Size, State, Style, Text, TextBundle, TextStyle, UiRect, Val, With};

use crate::AppState;
use crate::models::ui::game_over::NavigationButton;

pub fn spawn_game_over_screen_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                position: UiRect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                },
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    "You Dieded".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::RED,
                    },
                ),
                ..Default::default()
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Ragequit".to_string(),
                        TextStyle {
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
            })
                .insert(NavigationButton);
        });
}

pub fn button_click_system(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<NavigationButton>)>,
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    match app_state.0 {
        AppState::GameOver | AppState::GameWon => {
            for interaction in button_query.iter() {
                match *interaction {
                    Interaction::Clicked => {
                        app_exit_events.send(bevy::app::AppExit);
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }
        AppState::MainMenu => {
            for interaction in button_query.iter() {
                match *interaction {
                    Interaction::Clicked => {
                        next_app_state.set(AppState::InGame);
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }
        AppState::Paused => {}
        AppState::Shop => {
            for interaction in button_query.iter() {
                match *interaction {
                    Interaction::Clicked => {
                        next_app_state.set(AppState::InGame);
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }
        _ => {}
    }
}