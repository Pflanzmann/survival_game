use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::bullet::Bullet;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collision::EnemyHitBoxCollision;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::input::player_aim_controlled::PlayerAimControlled;
use crate::models::layerable::Layerable;
use crate::models::mirror_aim_to_move_direction::MirrorAimToMoveDirection;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::psy_rock::{PsyRock, PsyRockUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::move_direction::MoveDirection;
use crate::models::sprite_move_rotation::SpriteMoveRotation;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;

pub fn apply_psy_rock_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mut bullet_shot_event: EventWriter<BulletShotEvent>,
    mod_query: Query<&PsyRock, With<Modification>>,
    owner_query: Query<(Entity, &Transform, &WeaponSlot)>,
    unit_query: Query<&Owner, With<PsyRockUnit>>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, owner_transform, owner_weapon_slot) = match owner_query.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for owner in unit_query.iter() {
            if owner_entity == owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        let bullet = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(160.0, 160.0)),
                ..Default::default()
            },
            texture: texture_handler.psy_rock_unit.clone(),
            transform: Transform::from_translation(owner_transform.translation),
            ..Default::default()
        })
            .insert(Name::new("Psy Rock"))
            .insert(PsyRockUnit)
            .insert(Owner::new(owner_entity))
            .insert(Bullet { source_entity: owner_weapon_slot.weapon_entity })

            .insert(AimDirection::default())
            .insert(PlayerAimControlled)
            .insert(MoveDirection::default())
            .insert(MirrorAimToMoveDirection)
            .insert(MoveSpeed::new(0.0))

            .insert(UnitSize::new_size(modification.unit_size))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(0.0) })
            .insert(EnemyHitBoxCollision)

            .insert(Damage::new(0.0))
            .insert(DamageInterval::new(modification.damage_interval))
            .insert(DamagedEntities::default())
            .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
            .insert(SpriteMoveRotation)

            .insert(TeleportToTargetBehavior::new(owner_entity, modification.teleport_distance, modification.teleport_proximity_min, modification.teleport_proximity_max, modification.teleport_cooldown))
            .id()
            ;

        commands.entity(owner_entity).remove::<Reload>();

        bullet_shot_event.send(BulletShotEvent { entity: bullet });
    }
}

pub fn renew_mods_for_psy_rock_system(
    mut bullet_shot_event: EventWriter<BulletShotEvent>,
    mut apply_mod_events: EventReader<ApplyModToTargetEvent>,
    unit_query: Query<(Entity, &Owner), With<PsyRockUnit>>,
) {
    for (entity, owner) in unit_query.iter() {
        for event in apply_mod_events.iter() {
            if owner.entity == event.target_entity {
                bullet_shot_event.send(BulletShotEvent { entity });
            }
        }
    }
}