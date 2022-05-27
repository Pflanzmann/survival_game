use bevy::prelude::*;
use bevy::prelude::{AssetServer, Commands, Res};
use bevy::ui::FocusPolicy;

use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::ui::tooltip_window::{HoverTooltip, TooltipText, TooltipWindow};

pub fn setup_tool_tip_window(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    tooltip_window_query: Query<Entity, With<TooltipWindow>>,
    tooltip_query: Query<(&Interaction, &ToolTip), With<HoverTooltip>>,
) {
    let mut require_tooltip = false;
    for (interaction, _) in tooltip_query.iter() {
        if *interaction == Interaction::Hovered {
            require_tooltip = true;
            break;
        }
    }

    if !require_tooltip {
        for entity in tooltip_window_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    if tooltip_window_query.iter().next().is_some() {
        return;
    }

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(18.0), Val::Percent(16.0)),
            position: Rect {
                left: Val::Percent(110.0),
                top: Val::Percent(110.0),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::Center,
            ..Default::default()
        },
        image: asset_loader.load("sprites/ui/ítem_background.png").into(),
        ..Default::default()
    })
        .insert(Name::new("Tooltip Window"))
        .insert(TooltipWindow)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                focus_policy: FocusPolicy::Pass,
                style: Style {
                    size: Size { width: Val::Auto, height: Val::Auto },
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                },
                text: Text::with_section(
                    "".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 25.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            }
            ).insert(TooltipText);
        });
}

pub fn populate_tooltip_window(
    tooltip_query: Query<(&Interaction, &ToolTip)>,
    mut tooltip_window_query: Query<&mut Text, With<TooltipText>>,
) {
    for (interaction, tooltip) in tooltip_query.iter() {
        if *interaction != Interaction::Hovered {
            continue;
        }
        let mut tooltip_text = String::new();
        for (index, char) in tooltip.tooltip.chars().enumerate() {
            tooltip_text.push(char);
            if index > 0 && index % 20 == 0 {
                tooltip_text.push('\n');
            }
        }

        for mut text in tooltip_window_query.iter_mut() {
            text.sections[0].value = tooltip_text.clone();
        }
    }
}

pub fn move_tool_tip_window(
    window: Res<Windows>,
    mut tooltip_window_query: Query<(&mut Style, &Node), With<TooltipWindow>>,
) {
    for (mut style, node) in tooltip_window_query.iter_mut() {
        let window = window.get_primary().unwrap();
        let position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };

        style.position = Rect::<Val> {
            left: Val::Px(position.x),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Px(position.y - node.size.y),
        }
    }
}
