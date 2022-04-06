use bevy::prelude::*;

use crate::{CoinCount, Cointext, TextureHandles};
use crate::models::modification_components::{CurveShot, GrowShot, ModContainer, SplitShot};
use crate::models::ui_components::BulletHud;

pub fn spawn_text_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut coin_counter: ResMut<CoinCount>,
) {
    coin_counter.number = 0;

    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(5.0),
                bottom: Val::Percent(2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Coins: ".to_string(),
            TextStyle {
                font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
        .insert(Cointext)
        .id();

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(25.0), Val::Percent(5.0)),
            position: Rect {
                left: Val::Percent(5.0),
                top: Val::Percent(2.0),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        color: Color::GREEN.into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            /*style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(5.0),
                    top: Val::Percent(2.0),
                    ..Default::default()
                },
                ..Default::default()
            },*/
            text: Text::with_section(
                "Active Mods:  ".to_string(),
                TextStyle {
                    font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            /*style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(5.0),
                    top: Val::Percent(2.0),
                    ..Default::default()
                },
                ..Default::default()
            },*/
            text: Text::with_section(
                "Modliste ".to_string(),
                TextStyle {
                    font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    })
        .insert(BulletHud)
        .id();
}

pub fn update_text_system(
    mut text_query: Query<&mut Text, With<Cointext>>,
    mut coin_counter: ResMut<CoinCount>,
) {
    if coin_counter.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:.0}", coin_counter.number);
        }
    }
}

pub fn update_bullet_hud_system(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,

    mut containter_query: Query<(Option<&SplitShot>, Option<&CurveShot>, Option<&GrowShot>), (With<ModContainer>, Changed<SplitShot>)>,
    mut hud_query: Query<Entity, With<BulletHud>>,

) {
    for entity in hud_query.iter_mut() {
        for (split, curve, grow) in containter_query.iter_mut() {
            if split.is_some() {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image : texture_handles.bullet_fireball.clone().into(),
                        style :Style{
                            size : Size::new(Val::Percent(10.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }).id();
            } else {}
        }
    }
}