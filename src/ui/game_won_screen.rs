use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Color, Commands, FlexDirection, HorizontalAlign, JustifyContent, NodeBundle, PositionType, Query, Rect, Res, Size, Style, Text, TextAlignment, TextBundle, TextStyle, Val, VerticalAlign, With};

use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::player::Player;
use crate::models::ui::game_over::NavigationButton;
use crate::models::ui::game_won_screen::GameWonScreen;

pub fn spawn_game_won_screen_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    player_query: Query<&ModRegister, With<Player>>,
    mod_query: Query<(&ToolTip, &ModName), With<Modification>>,
) {
    let mut text_to_show = String::new();

    for mod_register in player_query.iter() {
        for mod_entity in mod_register.register.iter() {
            if let Ok((tooltip, mod_name)) = mod_query.get(*mod_entity) {
                text_to_show.push_str(mod_name.mod_name.as_str());
                text_to_show.push_str(" - ");
                text_to_show.push_str(tooltip.tooltip.as_str());
                text_to_show.push('\n');
            }
        }
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
            image: asset_loader.load("sprites/ui/ítem_background.png").into(),
            ..Default::default()
        })
        .insert(GameWonScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "You wonned the game!!".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    text_to_show,
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                ..Default::default()
            });

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
                        "Successfully leave the ".to_string(),
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
}