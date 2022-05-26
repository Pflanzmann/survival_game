use bevy::prelude::*;

use crate::models::gold_storage::GoldStorage;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_sprite_path::ModSpritePath;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;
use crate::models::ui_components::hud::{BulletHud, CoinText};

pub fn spawn_text_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    // coin counter
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
        text: Text {
            sections: vec![
                TextSection {
                    value: "Coins: ".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(CoinText)
        .insert(Name::new("CoinText"));

    //modification hud
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(18.0), Val::Percent(16.0)),
            position: Rect {
                left: Val::Percent(41.0),
                top: Val::Percent(2.0),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::WrapReverse,
            align_content: AlignContent::FlexStart,
            ..Default::default()
        },
        color: Color::from([0.2, 0.2, 0.2, 0.2]).into(),
        ..Default::default()
    })
        .insert(BulletHud)
        .insert(Name::new("BulletHud"));
}

pub fn update_text_system(
    mut text_query: Query<&mut Text, With<CoinText>>,
    gold_storage_query: Query<&GoldStorage, (Changed<GoldStorage>, With<Player>)>,
) {
    for gold_storage in gold_storage_query.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[1].value = gold_storage.number.to_string();
        }
    }
}

pub fn update_bullet_hud_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hud_query: Query<Entity, With<BulletHud>>,
    player_query: Query<&ModRegister, (With<Player>, Changed<ModRegister>)>,
    mod_query: Query<&ModSpritePath, With<Modification>>,
) {
    for _ in player_query.iter() {
        for entity in hud_query.iter() {
            commands.entity(entity).despawn_descendants();
        }
    }
    for mod_reg in player_query.iter() {
        for hud_entity in hud_query.iter() {
            for mod_entity in mod_reg.register.iter() {
                let sprite = match mod_query.get(*mod_entity) {
                    Ok(value) => value,
                    Err(_) => continue
                };

                commands.entity(hud_entity).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: asset_server.load(&sprite.path).into(),
                        style: Style {
                            size: Size::new(Val::Percent(20.0), Val::Percent(40.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            }
        }
    }
}