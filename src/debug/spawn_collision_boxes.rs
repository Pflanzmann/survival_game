use bevy::prelude::*;

use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};

#[derive(Component)]
pub struct CollisionBox;

pub fn spawn_collision_boxes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_unit_query: Query<(Entity, &ColliderType), Added<ColliderType>>,
) {
    for (entity, collider_type) in collision_unit_query.iter() {
        match collider_type {
            Circle(radius) => {
                let size = Vec2::new(*radius * 2.0, *radius * 2.0);
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_bundle(
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.8, 0.4, 0.6, 0.3]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            texture: asset_server.load("sprites/collider_circle.png"),
                            ..Default::default()
                        }
                    ).insert(CollisionBox);
                });
            }

            Rectangle(size) => {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_bundle(
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.8, 0.4, 0.6, 0.3]),
                                custom_size: Some(size.clone()),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    ).insert(CollisionBox);
                });
            }
        }
    }
}

pub fn init_collision_boxes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_unit_query: Query<(Entity, &ColliderType), With<Transform>>,
) {
    for (entity, collider_type) in collision_unit_query.iter() {
        match collider_type {
            Circle(radius) => {
                let size = Vec2::new(*radius * 2.0, *radius * 2.0);
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_bundle(
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.8, 0.4, 0.6, 0.3]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            texture: asset_server.load("sprites/collider_circle.png"),
                            ..Default::default()
                        }
                    ).insert(CollisionBox);
                });
            }

            Rectangle(size) => {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_bundle(
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.8, 0.4, 0.6, 0.3]),
                                custom_size: Some(size.clone()),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    ).insert(CollisionBox);
                });
            }
        }
    }
}


pub fn despawn_collision_boxes(
    mut commands: Commands,
    collision_unit_query: Query<Entity, With<CollisionBox>>,
) {
    for entity in collision_unit_query.iter() {
        commands.entity(entity).despawn();
    }
}
