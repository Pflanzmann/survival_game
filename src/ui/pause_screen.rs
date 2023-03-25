use bevy::prelude::{AlignItems, AssetServer, BuildChildren, Color, Commands, DespawnRecursiveExt, Entity, FlexDirection, JustifyContent, NodeBundle, PositionType, Query, Res, Size, Style, Text, TextBundle, TextStyle, UiRect, Val, With};

use crate::models::ui::pause::PauseMenuComp;

pub fn spawn_pause_system(
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
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    "Paused".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::RED,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(PauseMenuComp);
}

pub fn close_pause_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<PauseMenuComp>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
