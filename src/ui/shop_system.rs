use std::cmp::min;

use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::prelude::{AlignContent, AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, Entity, EventWriter, FlexDirection, FlexWrap, Handle, HorizontalAlign, Image, ImageBundle, Interaction, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, ResMut, Size, Style, Text, TextAlignment, TextBundle, TextStyle, Val, VerticalAlign, With, Without};
use rand::Rng;

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_sprite_path::ModSpritePath;
use crate::models::modifications::descriptors::price::Price;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::player::Player;
use crate::models::resources::shop_customer::ShopCustomer;
use crate::models::ui_components::game_over::NavigationButton;
use crate::models::ui_components::shop::{ShopButton, ShopMenuComp, ShopSlot, ToolTipField};
use crate::TextureHandles;

#[derive(Component)]
pub struct OriginalStyle;

#[derive(Component)]
pub struct OriginalStyle2;

#[derive(Component)]
pub struct CopyStyle;

#[derive(Component)]
pub struct CopyStyle2;

pub fn copy_style_system(
    mut commands: Commands,
    original: Query<(Entity, &Style), (With<OriginalStyle>, Changed<Style>)>,
    copy: Query<Entity, With<CopyStyle>>,
) {
    for (o_entity, origin) in original.iter() {
        for entity in copy.iter() {
            if o_entity != entity {
                commands.entity(entity).insert(origin.clone());
            }
        }
    }
}

pub fn copy_style_system2(
    mut commands: Commands,
    original: Query<(Entity, &Style), (With<OriginalStyle2>, Changed<Style>)>,
    copy: Query<Entity, With<CopyStyle2>>,
) {
    for (o_entity, origin) in original.iter() {
        for entity in copy.iter() {
            if o_entity != entity {
                commands.entity(entity).insert(origin.clone());
            }
        }
    }
}

pub fn spawn_shop_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    shop_customer: Res<ShopCustomer>,
    customer_query: Query<&ModRegister>,
    mod_query: Query<(Entity, &ModSpritePath, &Price)>,
) {
    let customer_entity = match shop_customer.customer {
        None => return,
        Some(customer) => customer,
    };

    let customer_mod_register = match customer_query.get(customer_entity) {
        Ok(mod_register) => mod_register,
        Err(_) => return,
    };

    let requested_slot_count = 12;

    let mut valid_mods: Vec<Entity> = Vec::new();
    for (entity, _, _) in mod_query.iter() {
        if !customer_mod_register.register.contains(&entity) {
            valid_mods.push(entity);
        }
    }

    let valid_mods_len = valid_mods.len() as i32;
    let mut chosen_mod_indexes: Vec<i32> = Vec::new();
    let mut chosen_mod_entities: Vec<Entity> = Vec::new();

    for _ in 0..min(valid_mods.len(), requested_slot_count) {
        let mut rnd_number: i32 = rand::thread_rng().gen_range(0..valid_mods_len);
        while chosen_mod_indexes.contains(&rnd_number) {
            rnd_number = rand::thread_rng().gen_range(0..valid_mods_len);
        }
        chosen_mod_indexes.push(rnd_number);
        if let Some(mod_entity) = valid_mods.get(rnd_number as usize) {
            chosen_mod_entities.push(*mod_entity);
        }
    }

    let mut origin: Option<Entity> = None;

    let mut populated_shop_slots: Vec<Entity> = Vec::new();
    for (index, mod_entity) in chosen_mod_entities.iter().enumerate() {
        let (mod_entity, mod_sprite_path, price) = match mod_query.get(*mod_entity) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let entity = commands.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                // min_size: Size::new(Val::Percent(18.0), Val::Auto),
                // max_size: Size::new(Val::Percent(26.0), Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::GRAY.into(),
            ..Default::default()
        })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        price.to_string(),
                        TextStyle {
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                            font_size: 40.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });

                let temp = parent.spawn_bundle(ImageBundle {
                    image: asset_loader.load(mod_sprite_path.path.as_str()).into(),
                    style: Style {
                        // size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                        position: Rect {
                            left: Val::Percent(10.0),
                            bottom: Val::Percent(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(Interaction::default())
                    .insert(ShopButton { index }).id();

                origin = Some(temp);
            }).id();

        if index == 0 {
            commands.entity(entity).insert(OriginalStyle);
            commands.entity(entity).insert(Name::new("Origin"));

            commands.entity(origin.unwrap()).insert(OriginalStyle2);
            commands.entity(origin.unwrap()).insert(Name::new("Origin"));
        } else {
            commands.entity(entity).insert(CopyStyle);
            commands.entity(entity).insert(Name::new("copy"));

            commands.entity(origin.unwrap()).insert(CopyStyle2);
            commands.entity(origin.unwrap()).insert(Name::new("copy"));
        }

        commands.entity(mod_entity).insert(ShopSlot { index });
        populated_shop_slots.push(entity);
    }

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

            //Shop Headline
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "Shop".to_string(),
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

            //Shoppable items
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(80.0), Val::Percent(40.0)),
                    position: Rect {
                        left: Val::Percent(10.0),
                        bottom: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                    },
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::WrapReverse,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::SpaceAround,
                    ..Default::default()
                },
                color: Color::WHITE.into(),
                ..Default::default()
            }).push_children(populated_shop_slots.as_slice());

            // Third UI Row (tooltip and close button)
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(80.0), Val::Percent(40.0)),
                    position: Rect {
                        left: Val::Percent(10.0),
                        bottom: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                    },
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                color: Color::WHITE.into(),
                ..Default::default()
            }).with_children(|parent| {

                //tooltipbox
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(90.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "".to_string(),
                            TextStyle {
                                font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                                font_size: 30.0,
                                color: Color::RED,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..Default::default()
                    })
                        .insert(ToolTipField);
                });

                //close Shop Button
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
                            "Leave Shop".to_string(),
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
            });
        })
        .insert(ShopMenuComp);
}

pub fn close_shop_menu_system(
    mut commands: Commands,
    slot_entity_query: Query<Entity, With<ShopSlot>>,
    shop_ui_query: Query<Entity, With<ShopMenuComp>>,
) {
    for entity in shop_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in slot_entity_query.iter() {
        commands.entity(entity).remove::<ShopSlot>();
    }
}

pub fn shop_button_system(
    mut commands: Commands,
    mut event: EventWriter<ApplyModToTargetEvent>,
    mut text_query: Query<&mut Text, With<ToolTipField>>,
    mut button_query: Query<(Entity, &mut Interaction, &ShopButton), Changed<Interaction>>,
    mod_query: Query<(Entity, &ToolTip, &ShopSlot)>,
    player_query: Query<Entity, With<Player>>,
    texture_handles: ResMut<TextureHandles>,
) {
    for (entity, interaction, shop_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for (modification_entity, _, shop_slot) in mod_query.iter() {
                    if shop_button.index != shop_slot.index { continue; }

                    for player_entity in player_query.iter() {
                        event.send(ApplyModToTargetEvent { mod_entity: modification_entity, target_entity: player_entity });
                    }

                    spawn_sold_image(&mut commands, entity, texture_handles.sold_button.clone());

                    commands.entity(modification_entity).remove::<ShopSlot>();
                }
            }

            Interaction::None => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = String::new();
                }
            }

            Interaction::Hovered => {
                for (_, modification_tooltip, shop_slot) in mod_query.iter() {
                    if shop_button.index != shop_slot.index { continue; }
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = String::from(&modification_tooltip.tooltip);
                    }
                }
            }
        }
    }
}

pub fn spawn_sold_image(
    commands: &mut Commands,
    entity: Entity,
    texture_handles: Handle<Image>,
) {
    let child = commands.spawn_bundle(ImageBundle {
        image: texture_handles.into(),
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position: Rect {
                left: Val::Percent(0.0),
                bottom: Val::Percent(0.0),
                top: Val::Percent(0.0),
                right: Val::Percent(0.0),
            },
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        ..Default::default()
    })
        .id();

    commands.entity(entity).add_child(child);
}