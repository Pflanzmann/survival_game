use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, Entity, FlexDirection, HorizontalAlign, ImageBundle, Interaction, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, ResMut, Size, Style, Text, TextAlignment, TextBundle, TextStyle, UiCameraBundle, Val, VerticalAlign, With};

use crate::{AppStateTrigger, EventWriter, Player, SpriteBundle};
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::modification_attributes2::{ModName, ModSpriteHandler, ToolTip};
use crate::models::modification_attributes::modification::Modification;
use crate::models::modification_components::ModContainerSlot;
use crate::models::ui_components::{NavigationButton, ShopButtonOne, ShopButtonThree, ShopButtonTwo, ShopEntityOne, ShopMenuComp, ToolTipField};

pub fn spawn_shop_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mod_query: Query<(Entity, &ModName, &ModSpriteHandler, &ToolTip), With<Modification>>,
) {
    let mut some: Option<(Entity, &ModName, &ModSpriteHandler, &ToolTip)> = Option::None;

    for (entity, name, spritehandler, tooltip) in mod_query.iter() {
        some = Option::Some((entity, name, spritehandler, tooltip));
        break;
    };

    let (entity, name, spritehandler, tooltip) = some.unwrap();

    commands.entity(entity).insert(ShopEntityOne);

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
            parent.spawn_bundle( NodeBundle{
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
            }).with_children(|parent|{

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
                        image: spritehandler.sprite.clone().into(),
                        style: Style {
                            size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Interaction::default())
                        .insert(ShopButtonOne);

                    parent.spawn_bundle(TextBundle {
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
                    });
                })
                    .insert(ShopButtonOne);

                //second item
                parent.spawn_bundle(ButtonBundle{
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                    .insert(ShopButtonTwo);

                //third item
                parent.spawn_bundle(ButtonBundle{
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                    .insert(ShopButtonThree);
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
    mut commands : Commands,
    my_query : Query<Entity, With<ShopMenuComp>>
){
    for entity in my_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn shop_button_system(
    mut commands: Commands,
    mut event: EventWriter<ApplyModToTargetSystem>,
    mut text_query: Query<&mut Text, With<ToolTipField>>,
    mut button_query: Query<&mut Interaction, (Changed<Interaction>, With<ShopButtonOne>)>,
    mut mod_query: Query<(Entity, &ToolTip), With<ShopEntityOne>>,
    mut player_query: Query<(Entity, &ModContainerSlot)>,
){
    for mut interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for (modification_entity, modification_tooltip) in mod_query.iter() {
                    for (player_entity, mod_container_slot) in player_query.iter() {
                        event.send(ApplyModToTargetSystem { mod_entity: modification_entity, target_entity: player_entity })
                    }
                    commands.entity(modification_entity).remove::<ShopEntityOne>();
                }
            }

            Interaction::None => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = String::new();
                }
            }

            Interaction::Hovered => {
                for (modification_entity, modification_tooltip) in mod_query.iter() {
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = String::from(&modification_tooltip.tooltip);
                    }
                }
            }
        }
    }
}