use bevy::prelude::*;
use bevy_inspector_egui::egui::TextBuffer;

use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::player::Player;
use crate::models::ui::attribute_text::{AttributeWindow, BulletDamageText, BulletHitLimitText, BulletMoveSpeedText, BulletTravelRangeText, BulletUnitSizeText, DamageText, HealthText, MoveSpeedText, ReloadText, UnitSizeText};
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::weapon_slot::WeaponSlot;

pub fn spawn_attribute_window_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    //modification hud
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            position: Rect {
                left: Val::Percent(1.0),
                bottom: Val::Percent(15.0),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::ColumnReverse,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::FlexStart,
            ..Default::default()
        },
        color: Color::from([0.2, 0.2, 0.2, 0.8]).into(),
        ..Default::default()
    }).with_children(|parent| {
        info_window_tex(parent, &asset_loader, "Health: ".as_str(), HealthText);
        info_window_tex(parent, &asset_loader, "Damage: ".as_str(), DamageText);
        info_window_tex(parent, &asset_loader, "MoveSpeed: ".as_str(), MoveSpeedText);
        info_window_tex(parent, &asset_loader, "Reload: ".as_str(), ReloadText);
        info_window_tex(parent, &asset_loader, "UnitSize: ".as_str(), UnitSizeText);
        info_window_tex(parent, &asset_loader, "BulletDamage: ".as_str(), BulletDamageText);
        info_window_tex(parent, &asset_loader, "BulletMoveSpeed: ".as_str(), BulletMoveSpeedText);
        info_window_tex(parent, &asset_loader, "BulletHitLimit: ".as_str(), BulletHitLimitText);
        info_window_tex(parent, &asset_loader, "BulletTravelRange: ".as_str(), BulletTravelRangeText);
        info_window_tex(parent, &asset_loader, "BulletUnitSize: ".as_str(), BulletUnitSizeText);
    }).insert(Name::new("Attribute Window")).insert(AttributeWindow);
}

pub fn assign_attribute_value_system<
    T: Component + Attribute,
    U: Component,
>(
    mut text_query: Query<&mut Text, With<U>>,
    attribute_query: Query<&T, With<Player>>,
) {
    for attribute in attribute_query.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[1].value = attribute.get_total_amount().to_string();
            text.sections[3].value = attribute.get_base_amount().to_string();
            text.sections[5].value = attribute.get_bonus_amount().to_string();
            text.sections[7].value = attribute.get_multiplier().to_string();
        }
    }
}

pub fn assign_bullet_attribute_value_system<
    T: Component + Attribute,
    U: Component,
>(
    mut text_query: Query<&mut Text, With<U>>,
    attribute_query: Query<(Entity, &T), With<AttributeContainer>>,
    player_query: Query<&WeaponSlot, With<Player>>,
    gun_query: Query<&AttributeContainerSlot>,
) {
    for (attribute_entity, attribute) in attribute_query.iter() {
        for weapon_slot in player_query.iter() {
            let slot = match gun_query.get(weapon_slot.weapon_entity) {
                Ok(attribute) => attribute,
                Err(_) => continue,
            };

            if attribute_entity == slot.container_entity {
                for mut text in text_query.iter_mut() {
                    text.sections[1].value = attribute.get_total_amount().to_string();
                    text.sections[3].value = attribute.get_base_amount().to_string();
                    text.sections[5].value = attribute.get_bonus_amount().to_string();
                    text.sections[7].value = attribute.get_multiplier().to_string();
                }
            }
        }
    }
}

fn info_window_tex<T: Component>(
    parent: &mut bevy::prelude::ChildBuilder,
    asset_loader: &Res<AssetServer>,
    title: &str,
    component: T,
) {
    let text_size = 25.0;

    parent.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                left: Val::Percent(1.0),
                bottom: Val::Percent(2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: title.to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0.0".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: " = ".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0.0".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: " + ".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0.0".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: " * ".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0.0".to_string(),
                    style: TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: text_size,
                        color: Color::WHITE,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    }).insert(component);
}