use bevy::prelude::{BuildChildren, Color, Commands, Entity, Name, Query, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::HealthBar;
use crate::models::player::Player;
use crate::models::sprite_layer::SpriteLayer;

pub fn setup_health_bar(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    let player_result = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let child_healthbar = commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 50.0)),
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -125.0, SpriteLayer::UILevel.get_layer_z()),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .insert(HealthBar)
        .insert(Name::new("Player HealthBar"))
        .id();

    let child_background = commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 50.0)),
                color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -125.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .id();

    commands.entity(player_result).push_children(&[child_healthbar, child_background]);
}