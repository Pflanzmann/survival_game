use bevy::prelude::*;

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;

#[derive(Component)]
pub struct CollisionBox;

pub fn spawn_collision_boxes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_unit_query: Query<(Entity, &SolidBodyCollider), Added<SolidBodyCollider>>,
    hit_box_collider_query: Query<(Entity, &HitBoxCollider), Added<HitBoxCollider>>,
) {
    for (entity, solid_collider) in collision_unit_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                match solid_collider.collider_type {
                    Circle(radius) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([1.0, 0.2, 0.8, 0.4]),
                                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(solid_collider.offset.extend(2.0)),
                            texture: asset_server.load("sprites/collider_circle.png"),
                            ..Default::default()
                        }
                    }

                    Rectangle(size) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([1.0, 0.2, 0.8, 0.4]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(solid_collider.offset.extend(2.0)),
                            ..Default::default()
                        }
                    }
                }
            ).insert(CollisionBox);
        });
    }

    for (entity, hit_box_collider) in hit_box_collider_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                match hit_box_collider.collider_type {
                    Circle(radius) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.4, 1.0, 1.0, 0.4]),
                                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                                ..Default::default()
                            },
                            texture: asset_server.load("sprites/collider_circle.png"),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..Default::default()
                        }
                    }

                    Rectangle(size) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.4, 1.0, 1.0, 0.4]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..Default::default()
                        }
                    }
                }
            ).insert(CollisionBox);
        });
    }
}

pub fn init_collision_boxes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    solid_body_collider_query: Query<(Entity, &SolidBodyCollider), With<Transform>>,
    hit_box_collider_query: Query<(Entity, &HitBoxCollider), With<Transform>>,
) {
    for (entity, solid_body_collider) in solid_body_collider_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                match solid_body_collider.collider_type {
                    Circle(radius) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([1.0, 0.2, 0.8, 0.4]),
                                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(solid_body_collider.offset.extend(2.0)),
                            texture: asset_server.load("sprites/collider_circle.png"),
                            ..Default::default()
                        }
                    }

                    Rectangle(size) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([1.0, 0.2, 0.8, 0.4]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(solid_body_collider.offset.extend(2.0)),
                            ..Default::default()
                        }
                    }
                }
            ).insert(CollisionBox);
        });
    }

    for (entity, hit_box_collider) in hit_box_collider_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                match hit_box_collider.collider_type {
                    Circle(radius) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.4, 1.0, 1.0, 0.4]),
                                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                                ..Default::default()
                            },
                            texture: asset_server.load("sprites/collider_circle.png"),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..Default::default()
                        }
                    }

                    Rectangle(size) => {
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::from([0.4, 1.0, 1.0, 0.4]),
                                custom_size: Some(size),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..Default::default()
                        }
                    }
                }
            ).insert(CollisionBox);
        });
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
