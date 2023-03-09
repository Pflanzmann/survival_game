use bevy::prelude::{BuildChildren, Color, Commands, Entity, Name, Query, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::models::player::Player;
use crate::models::ui::hud::HealthBar;

pub fn setup_player_health_bar_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let background = commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 50.0)),
                color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -125.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .insert(Name::new("HealthBar")).id();

    let health_bar = commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 50.0)),
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 20000000.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .insert(HealthBar { owner: player_entity })
        .id();

    commands.entity(background).add_child(health_bar);
    commands.entity(player_entity).add_child(background);
}