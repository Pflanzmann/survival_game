use std::cmp::min;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, Entity, EventWriter, FlexDirection, Handle, HorizontalAlign, Image, ImageBundle, Interaction, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, ResMut, Size, Style, Text, TextAlignment, TextBundle, TextStyle, Val, VerticalAlign, With};
use rand::Rng;

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_path::ModSpritePath;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::player::Player;
use crate::models::ui_components::{NavigationButton, ShopButton, ShopMenuComp, ShopSlot, ToolTipField};
use crate::TextureHandles;

pub fn spawn_shop_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut texture_handle: ResMut<TextureHandles>,
    mod_query: Query<(Entity, &ModName, &ModSpritePath, &ToolTip)>,
) {
    let mut shop_items_vec: Vec<(Entity, String, String, String)> = Vec::new();
    fetch_shop_slots(3, &mut shop_items_vec, mod_query);

    let (entity1, _, sprite_handler1, _) = shop_items_vec.get(0).unwrap();
    let (entity2, _, sprite_handler2, _) = shop_items_vec.get(1).unwrap();
    let (entity3, _, sprite_handler3, _) = shop_items_vec.get(2).unwrap();

    commands.entity(*entity1).insert(ShopSlot { index: 0 });
    commands.entity(*entity2).insert(ShopSlot { index: 1 });
    commands.entity(*entity3).insert(ShopSlot { index: 2 });

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
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                color: Color::WHITE.into(),
                ..Default::default()
            }).with_children(|parent| {

                //first item
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(70.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: asset_loader.load(sprite_handler1).into(),
                        style: Style {
                            size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Interaction::default())
                        .insert(ShopButton { index: 0 });

                    /*parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            name.mod_name.to_string(),
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
                    });*/
                });
                //.insert(ShopButtonOne);

                //second item
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(70.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: asset_loader.load(sprite_handler2).into(),
                        style: Style {
                            size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Interaction::default())
                        .insert(ShopButton { index: 1 });

                    /*parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            name.mod_name.to_string(),
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
                    });*/
                });


                //third item
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(70.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: asset_loader.load(sprite_handler3).into(),
                        style: Style {
                            size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Interaction::default())
                        .insert(ShopButton { index: 2 });

                    /*parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            name.mod_name.to_string(),
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
                    });*/
                });
            });

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
        .insert(ShopMenuComp)
        .id();
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
    mut texture_handles: ResMut<TextureHandles>,
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

pub fn fetch_shop_slots(
    requested_slot_count: i32,
    result_vector: &mut Vec<(Entity, String, String, String)>,
    mod_query: Query<(Entity, &ModName, &ModSpritePath, &ToolTip)>,
) {
    let mut rng = rand::thread_rng();

    let entity_length = mod_query.iter().len() as i32;
    let mut rnd_vec: Vec<i32> = Vec::new();

    for _ in 0..min(entity_length, requested_slot_count) {
        let mut rnd_number = rng.gen_range(0..entity_length);
        while rnd_vec.contains(&rnd_number) {
            rnd_number = rng.gen_range(0..entity_length);
        }
        rnd_vec.push(rnd_number);
    }

    for (index, (entity, mod_name, mod_sprite_path, tool_tip)) in mod_query.iter().enumerate() {
        let result_mod_name = mod_name.mod_name.clone();
        let result_mod_sprite_path = mod_sprite_path.path.clone();
        let result_mod_tool_tip = tool_tip.tooltip.clone();

        if rnd_vec.contains(&(index as i32)) {
            result_vector.push((entity, result_mod_name, result_mod_sprite_path, result_mod_tool_tip));
        }
    };
}

pub fn spawn_sold_image(
    commands: &mut Commands,
    entity: Entity,
    texture_handles: Handle<Image>,
) {
    let child = commands.spawn_bundle(ImageBundle {
        image: texture_handles.clone().into(),
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