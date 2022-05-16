use bevy::prelude::{BuildChildren, Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collision::EnemyHitBoxCollision;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::radiation::{Radiation, RadiationUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::TextureHandles;

pub fn apply_radiation_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Radiation, With<Modification>>,
    owner_query: Query<Entity>,
    unit_query: Query<&Owner, With<RadiationUnit>>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let owner_entity = match owner_query.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for unit_owner in unit_query.iter() {
            if owner_entity == unit_owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        let child = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            texture: texture_handler.radiation.clone(),
            ..Default::default()
        })
            .insert(Name::new("Radiation"))
            .insert(RadiationUnit)
            .insert(Owner::new(owner_entity))

            .insert(UnitSize::new_size(modification.unit_size))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(0.0) })
            .insert(EnemyHitBoxCollision)

            .insert(Damage::new(modification.damage))
            .insert(DamageInterval::new(modification.damage_interval))
            .insert(DamagedEntities::default())
            .id();

        commands.entity(owner_entity).add_child(child);
    }
}
