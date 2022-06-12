use std::cmp::min;

use bevy::prelude::{AlignContent, AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, Entity, EventWriter, FlexDirection, FlexWrap, ImageBundle, Interaction, JustifyContent, Name, NodeBundle, PositionType, Query, Res, Size, Style, Text, TextBundle, TextStyle, UiRect, Val, With};
use bevy::ui::FocusPolicy;
use rand::Rng;

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::gold_storage::GoldStorage;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_sprite_path::SpriteHandle;
use crate::models::modifications::descriptors::price::Price;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::player::Player;
use crate::models::resources::shop_customer::ShopCustomer;
use crate::models::ui::navigation_button::NavigationButton;
use crate::models::ui::shop::{ShopButton, ShopMenuComp, ShopSlot, ToolTipField};
use crate::models::ui::tooltip_window::TooltipText;

pub fn spawn_shop_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    shop_customer: Res<ShopCustomer>,
    customer_query: Query<&ModRegister>,
    mod_query: Query<(Entity, &SpriteHandle, &Price, &ToolTip)>,
) {
    let customer_entity = match shop_customer.customer {
        None => return,
        Some(customer) => customer,
    };

    let customer_mod_register = match customer_query.get(customer_entity) {
        Ok(mod_register) => mod_register,
        Err(_) => return,
    };

    let requested_slot_count = 5;

    let mut valid_mods: Vec<Entity> = Vec::new();
    for (entity, _, _, _) in mod_query.iter() {
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

    let mut populated_shop_slots: Vec<Entity> = Vec::new();
    for (index, mod_entity) in chosen_mod_entities.iter().enumerate() {
        let (mod_entity, mod_sprite_path, price, tooltip) = match mod_query.get(*mod_entity) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let entity = commands.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                min_size: Size::new(Val::Auto, Val::Percent(51.0)),
                max_size: Size::new(Val::Auto, Val::Percent(75.0)),
                padding: UiRect {
                    left: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    right: Val::Percent(1.0),
                },
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: asset_loader.load("sprites/ui/ítem_background.png").into(),
            background_color: Color::GRAY.into(),
            ..Default::default()
        })
            .insert(Interaction::default())
            .insert(ShopButton { index, modification_entity: mod_entity, price: price.0 })
            .insert(tooltip.clone())
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    focus_policy: FocusPolicy::Pass,
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        price.to_string(),
                        TextStyle {
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                            font_size: 40.0,
                            color: Color::RED,
                        },
                    ),
                    ..Default::default()
                });

                parent.spawn(ImageBundle {
                    focus_policy: FocusPolicy::Pass,
                    image: mod_sprite_path.handle.clone().into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            })
            .id();

        populated_shop_slots.push(entity);
    }

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
            //   background_color: asset_loader.load("sprites/ui/ítem_background.png").into(),
            ..Default::default()
        })
        .with_children(|parent| {

            //Shop Headline
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    "Shop".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                ),
                ..Default::default()
            });

            //Shoppable items
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(80.0), Val::Percent(40.0)),
                    position: UiRect {
                        left: Val::Percent(10.0),
                        bottom: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                    },
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::SpaceAround,
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..Default::default()
            }).push_children(populated_shop_slots.as_slice());

            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..Default::default()
            })

                //Tooltip
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(25.0), Val::Percent(50.0)),
                            position: UiRect {
                                left: Val::Percent(10.0),
                                ..Default::default()
                            },
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::NoWrap,
                            align_content: AlignContent::Center,
                            ..Default::default()
                        },
                        //  background_color: asset_loader.load("sprites/ui/ítem_background.png").into(),
                        ..Default::default()
                    })
                        .insert(Name::new("Tooltip Window"))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                focus_policy: FocusPolicy::Pass,
                                style: Style {
                                    size: Size { width: Val::Auto, height: Val::Auto },
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..Default::default()
                                },
                                text: Text::from_section(
                                    "".to_string(),
                                    TextStyle {
                                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                                        font_size: 25.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                ..Default::default()
                            }
                            ).insert(TooltipText);
                        });

                    // Close button
                    parent.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(7.0), Val::Percent(30.0)),
                            position: UiRect {
                                left: Val::Percent(33.0),
                                ..Default::default()
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceEvenly,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        image: asset_loader.load("sprites/ui/shop_close.png").into(),
                        ..Default::default()
                    })
                        .insert(NavigationButton);
                });
        }).insert(ShopMenuComp);
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
    shop_customer: Res<ShopCustomer>,
    mut event: EventWriter<ApplyModToTargetEvent>,
    mut text_query: Query<&mut Text, With<ToolTipField>>,
    mut button_query: Query<(Entity, &mut Interaction, &ShopButton), Changed<Interaction>>,
    mut player_query: Query<(Entity, &mut GoldStorage), With<Player>>,
) {
    for (button_entity, interaction, shop_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let (player_entity, mut gold_storage) = match player_query.get_mut(shop_customer.customer.unwrap()) {
                    Ok(player) => player,
                    Err(_) => continue,
                };

                if gold_storage.number < shop_button.price {
                    continue;
                }

                commands.entity(button_entity).despawn_recursive();

                gold_storage.number -= shop_button.price;
                event.send(ApplyModToTargetEvent { mod_entity: shop_button.modification_entity, target_entity: player_entity });
            }

            Interaction::None => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = String::new();
                }
            }

            Interaction::Hovered => {}
        }
    }
}
