use bevy::prelude::{AlignItems, AssetServer, BuildChildren, Color, Commands, DespawnRecursiveExt, Entity, FlexDirection, HorizontalAlign, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, Size, Style, Text, TextAlignment, TextBundle, TextStyle, Val, VerticalAlign, With};

use crate::models::ui_components::pause::PauseMenuComp;

pub fn enter_pause_system(
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
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "Paused".to_string(),
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
        })
        .insert(PauseMenuComp);
}

pub fn exit_pause_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<PauseMenuComp>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
