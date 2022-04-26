use bevy::prelude::*;

use crate::models::unit_size::UnitSize;

#[derive(Component)]
pub struct CollisionBox;

pub fn spawn_collision_boxes(
    mut commands: Commands,
    collision_unit_query: Query<(Entity, &UnitSize), Added<UnitSize>>,
) {
    for (entity, unit_size) in collision_unit_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from([0.8, 0.4, 0.6, 0.3]),
                        custom_size: Some(unit_size.collider_size),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).insert(CollisionBox);
        });
    }
}

pub fn init_collision_boxes(
    mut commands: Commands,
    collision_unit_query: Query<(Entity, &UnitSize), With<Transform>>,
) {
    for (entity, unit_size) in collision_unit_query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from([0.8, 0.4, 0.6, 0.3]),
                        custom_size: Some(unit_size.collider_size),
                        ..Default::default()
                    },
                    ..Default::default()
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
