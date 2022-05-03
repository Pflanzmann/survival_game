use bevy::prelude::{BuildChildren, Color, Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::bullet::Bullet;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collision_weight::CollisionWeight;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::death_ball::{DeathBall, DeathBallUnit};
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::psy_rock::{PsyRock, PsyRockUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::player_aim_controlled::PlayerAimControlled;
use crate::models::player_move_controlled::PlayerMoveControlled;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::meele_attack_speed::MeeleAttackSpeed;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;

pub fn apply_psy_rock_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&PsyRock, With<Modification>>,
    owner_query: Query<(Entity, &WeaponSlot)>,
    turret_query: Query<&Owner, With<PsyRockUnit>>,
) {
    for apply_event in apply_events.iter() {
        let psy_rock = match mod_query.get(apply_event.mod_entity) {
            Ok(death_ball) => death_ball,
            Err(_) => continue,
        };

        let (owner_entity, owner_weapon_slot) = match owner_query.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for owner in turret_query.iter() {
            if owner_entity == owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(160.0, 160.0)),
                ..Default::default()
            },
            texture: texture_handler.psy_rock_unit.clone(),
            transform: Transform::from_xyz(0.0, 0.0, SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(PsyRockUnit)
            .insert(Owner::new(owner_entity))
            .insert(PlayerAimControlled)
            .insert(MoveDirection { direction: Vec3::default() })
            .insert(AimDirection { direction: Vec3::default() })
            .insert(MoveSpeed::new(20.0))
            .insert(Name::new("Psy Rock"))
            .insert(UnitSize { collider_size: Vec2::new(160.0, 160.0) })
            .insert(ColliderType::Circle(80.0))
            .insert(CollisionWeight { weight: 0.5 })
            .insert(MeeleAttackSpeed::new(60.0))
            .insert(Damage::new(10.0))
            .insert(Player)
            .id();


        commands.entity(owner_entity).remove::<Reload>();
    }
}
