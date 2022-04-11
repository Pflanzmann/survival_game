use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, Entity, FlexDirection, HorizontalAlign, Interaction, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, ResMut, Size, Style, Text, TextAlignment, TextBundle, TextStyle, UiCameraBundle, Val, VerticalAlign, With};
use crate::AppStateTrigger;
use crate::models::ui_components::{NavigationButton, ShopButton, ShopMenuComp};

pub fn spawn_shop_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
){
    commands.spawn_bundle(UiCameraBundle::default());

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
                    .insert(ShopButton);

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
                    .insert(ShopButton);

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
                    .insert(ShopButton);

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
    mut button_query : Query<(Entity, &mut Interaction), (Changed<Interaction>, With<ShopButton>)>,

){
    for (entity, mut interaction) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                //apply component to player/gun/bullte/whatever
                println!("shop button clicked")
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}