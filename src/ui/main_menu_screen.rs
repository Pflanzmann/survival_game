use bevy::prelude::*;

use crate::models::ui::game_over::NavigationButton;
use crate::models::ui::main_menu::MainMenuComp;

pub fn spawn_main_menu_system(
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
                    "Atomic UndersurVampire".to_string(),
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
                        "Start Game".to_string(),
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