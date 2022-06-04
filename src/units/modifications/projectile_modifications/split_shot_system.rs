use bevy::prelude::{Commands, EulerRot, EventReader, EventWriter, GlobalTransform, Name, Quat, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};
use rand::random;

use crate::{SpriteLayer, TextureHandles};
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::projectile::Projectile;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::child_projectile::ChildProjectile;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::events::projectile_stopped_event::ProjectileStoppedEvent;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;

/// A system to split the [Projectile] that has [SplitShot] applied to it.
/// The shot gets split when the projectile stops.
pub fn split_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    mut projectile_shot_event_writer: EventWriter<ProjectileShotEvent>,
    mut projectile_stopped_events: EventReader<ProjectileStoppedEvent>,
    projectile_query: Query<(&GlobalTransform, &Projectile, &DamagedEntities), With<SplitShot>>,
) {
    for event in projectile_stopped_events.iter() {
        let (projectile_transform, projectile, collided_entities) = match projectile_query.get(event.projectile_entity) {
            Ok(transform) => transform,
            Err(_) => continue,
        };

        let random_rotation = random::<f32>();

        let directions = vec![
            Vec2::new(1.0 - random_rotation, random_rotation).normalize(),
            Vec2::new(-random_rotation, 1.0 - random_rotation).normalize(),
            Vec2::new(-1.0 + random_rotation, -random_rotation).normalize(),
            Vec2::new(random_rotation, -1.0 + random_rotation).normalize(),
        ];


        for direction in directions {
            let random_rotation: f32 = random::<f32>() * 100.0;
            let mut projectile_transform = Transform::from_xyz(projectile_transform.translation.x, projectile_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z());
            projectile_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, random_rotation);

            let projectile = command.spawn_bundle(SpriteBundle {
                transform: projectile_transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.projectile_fireball.clone(),
                ..Default::default()
            })
                .insert(Name::new("SplitShot Projectile"))
                .insert(*projectile)
                .insert(ChildProjectile)

                .insert(UnitSize::new_size(Vec2::new(128.0, 128.0)))
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(128.0 / 2.0) }).insert(EnemyHitBoxCollider)
                .insert(EnemyHitBoxCollider)

                .insert_bundle(DamageBundle::new(0.0, 60.0))
                .insert(collided_entities.clone())

                .insert(MoveSpeed::default())
                .insert(MoveDirection { direction })

                .insert(HitLimit::new(1.0))
                .insert(TravelRange::new(2048.0))

                .insert(UnitRotation { revolutions_per_min: if random_rotation > 50.0 { 40.0 } else { -40.0 } })
                .id();

            projectile_shot_event_writer.send(ProjectileShotEvent { entity: projectile });
        }
    }
}