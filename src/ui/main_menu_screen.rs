use bevy::prelude::*;

use crate::models::ui_components::game_over::NavigationButton;
use crate::models::ui_components::main_menu::MainMenuComp;


pub fn spawn_main_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default()).insert(Name::new("UiCamera"));

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
                    "Atomic UndersurVampire".to_string(),
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
                        "Start Game".to_string(),
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
        .insert(MainMenuComp);
}

pub fn close_main_menu_system(
    mut commands: Commands,
    my_query: Query<Entity, With<MainMenuComp>>,
) {
    for entity in my_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}