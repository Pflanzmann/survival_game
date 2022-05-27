use bevy::core::Name;
use bevy::prelude::{AlignItems, AssetServer, BuildChildren, Color, Commands, DespawnRecursiveExt, Entity, FlexDirection, HorizontalAlign, JustifyContent, NodeBundle, Overflow, PositionType, Query, Rect, Res, Size, Style, Text, TextAlignment, TextBundle, TextSection, TextStyle, Val, VerticalAlign, With};
use bevy::ui::AlignContent;

use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui::debug_console::{DebugConsoleHistory, DebugConsoleInput, DebugConsoleWindow, DebugFpsCounter};

pub fn setup_debug_window(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    console_history: Res<ConsoleHistory>,
) {
    let mut history_string = String::new();

    for text in console_history.log.iter().rev() {
        history_string.push_str(text);
    }

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(40.0), Val::Percent(40.0)),
                position: Rect {
                    left: Val::Percent(58.0),
                    bottom: Val::Percent(58.0),
                    top: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::from([0.2, 0.2, 0.2, 0.8]).into(),
            ..Default::default()
        })
        .insert(Name::new("Debug console"))
        .insert(DebugConsoleWindow)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(8.0)),
                    position: Rect {
                        left: Val::Percent(0.0),
                        bottom: Val::Percent(0.00),
                        top: Val::Percent(90.0),
                        right: Val::Percent(0.0),
                    },
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::FlexStart,
                    overflow: Overflow::Hidden,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                color: Color::from([0.2, 0.2, 0.2, 0.8]).into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style::default(),
                    text: Text::with_section(
                        "".to_string(),
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    ..Default::default()
                }).insert(DebugConsoleInput);
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(91.0)),
                    position: Rect {
                        left: Val::Percent(0.0),
                        bottom: Val::Percent(9.0),
                        top: Val::Percent(0.0),
                        right: Val::Percent(0.0),
                    },
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::with_section(
                    history_string,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::from([0.6, 0.6, 0.6, 1.0]),
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                ..Default::default()
            }).insert(DebugConsoleHistory);
        });
}

pub fn setup_debug_info_window(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.0), Val::Percent(40.0)),
                position: Rect {
                    left: Val::Percent(80.0),
                    bottom: Val::Percent(10.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                align_content: AlignContent::FlexStart,
                ..Default::default()
            },
            color: Color::from([0.2, 0.2, 0.2, 0.8]).into(),
            ..Default::default()
        })
        .insert(Name::new("Debug info"))
        .insert(DebugConsoleWindow)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::RowReverse,
                    position: Rect {
                        left: Val::Percent(5.0),
                        bottom: Val::Percent(90.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    alignment: TextAlignment { vertical: VerticalAlign::Top, ..Default::default() },
                    sections: vec![
                        TextSection {
                            value: "Fps: ".to_string(),
                            style: TextStyle {
                                font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                },
                ..Default::default()
            }).insert(DebugFpsCounter);
        });
}

pub fn exit_debug_console_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<DebugConsoleWindow>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}