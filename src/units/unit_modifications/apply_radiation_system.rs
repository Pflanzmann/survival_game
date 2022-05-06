use bevy::prelude::{BuildChildren, Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Vec2, With};

use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::bullet::Bullet;
use crate::models::collision::collided_entities::DamagedEntities;
use crate::models::collision::collider_type::ColliderType;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::radiation::{Radiation, RadiationUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;
use crate::models::unit_size::UnitSize;
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
        let _modification = match mod_query.get(apply_event.mod_entity) {
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
            texture: texture_handler.radiation.clone(),
            ..Default::default()
        })
            .insert(Name::new("Radiation"))
            .insert(RadiationUnit)
            .insert(Owner::new(owner_entity))
            .insert(UnitSize { collider_size: Vec2::new(1024.0, 1024.0) })
            .insert(ColliderType::Circle(512.0))
            .insert(UnitRotation { angle: 0.1 })
            .insert(Damage::new(8.0))
            .insert(DamagedEntities::default())
            .insert(DamageInterval::new(90.0))
            .insert(Bullet { source_entity: owner_entity })
            .id();

        commands.entity(owner_entity).add_child(child);
    }
}
