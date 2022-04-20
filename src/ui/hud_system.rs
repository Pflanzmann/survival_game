use bevy::prelude::*;

use crate::{GoldWallet, TextureHandles};
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::mod_container::ModContainer;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_path::ModSpritePath;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::player::Player;
use crate::models::ui_components::hud::{BulletHud, CoinText};


pub fn spawn_text_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut coin_counter: ResMut<GoldWallet>,
) {
    coin_counter.number = 0;

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
        /*text: Text::with_section(
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
        ),*/
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
        .id();

    //modification hud
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
        .insert(BulletHud);
}

pub fn update_text_system(
    mut text_query: Query<&mut Text, With<CoinText>>,
    coin_counter: Res<GoldWallet>,
) {
    if coin_counter.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[1].value = format!("{:.0}", coin_counter.number);
        }
    }
}

pub fn update_bullet_hud_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mut hud_query: Query<Entity, With<BulletHud>>,
    mut player_query: Query<(Entity, &mut ModRegister), (With<Player>, Changed<ModRegister>)>,
    mut mod_query: Query<(Entity, &ModSpritePath, &ModName), With<Modification>>,
) {
    //reset the hud whenever a change is registered
    //also have the changed<modregister> as run criteria
    for (player_entity, mod_reg) in player_query.iter() {
        for hud_entity in hud_query.iter() {
            for some in mod_reg.register.iter() {
                let (any, sprite, mod_name) = match mod_query.get(*some) {
                    Ok(value) => value,
                    Err(_) => {
                        println!("some error in mod_query get()");
                        continue;
                    }
                };
                println!("{}", mod_name.mod_name);
                commands.entity(hud_entity).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: asset_server.load(&sprite.path).into(),
                        style: Style {
                            size: Size::new(Val::Percent(10.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }).id();
            }
        }
    }
}