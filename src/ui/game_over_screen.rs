use bevy::app::Events;
use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, FlexDirection, HorizontalAlign, Interaction, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, ResMut, Size, State, Style, Text, TextAlignment, TextBundle, TextStyle, Val, VerticalAlign, With};

use crate::{AppState, AppStateTrigger, ToAppState};
use crate::models::ui_components::NavigationButton;

pub fn spawn_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                position: Rect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                },
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "You Dieded".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Ragequit".to_string(),
                        TextStyle {
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
                .insert(NavigationButton);
        })
        .id();
}

pub fn button_click_system(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut button_query: Query<&mut Interaction, (Changed<Interaction>, With<NavigationButton>)>,
    app_state: ResMut<State<AppState>>,
    mut state_trigger: ResMut<AppStateTrigger>,
) {
    match app_state.current() {
        AppState::GameOver => {
            for interaction in button_query.iter_mut() {
                match *interaction {
                    Interaction::Clicked => {
                        app_exit_events.send(bevy::app::AppExit);
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }

        AppState::Loading => {}
        AppState::InGame => {}
        AppState::Pre => {}
        AppState::MainMenu => {
            for interaction in button_query.iter_mut() {
                match *interaction {
                    Interaction::Clicked => {
                        state_trigger.state_change_trigger = ToAppState::ToInGame;
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }
        AppState::Paused => {}
        AppState::Shop => {
            for interaction in button_query.iter_mut() {
                match *interaction {
                    Interaction::Clicked => {
                        state_trigger.state_change_trigger = ToAppState::ToInGame;
                    }
                    Interaction::Hovered => {}
                    Interaction::None => {}
                }
            }
        }
    }
}