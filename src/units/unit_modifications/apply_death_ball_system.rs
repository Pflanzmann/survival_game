use std::any::Any;

use bevy::prelude::{BuildChildren, Commands, Entity, EventReader, GlobalTransform, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With, Without};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::aim_at_closest_target_behavior::AimAtClosestTargetBehavior;
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::death_ball::DeathBall;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;

pub fn apply_death_ball_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&DeathBall, With<Modification>>,
    death_ball_owner: Query<(Entity, &WeaponSlot), Without<Modification>>,
) {
    for apply_event in apply_events.iter() {
        let death_ball = match mod_query.get(apply_event.mod_entity) {
            Ok(death_ball) => death_ball,
            Err(_) => continue,
        };

        let (owner_entity, weapon_slot) = match death_ball_owner.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let desired_pos = Vec3::new(death_ball.rotation_distance, 0.0, 0.0);

        let base = commands.spawn()
            .insert(Transform::default())
            .insert(UnitRotation { angle: death_ball.rotation_speed })
            .insert(GlobalTransform::default())
            .id();

        let child = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handler.death_ball_unit.clone(),
            transform: Transform::from_xyz(desired_pos.x, desired_pos.y, SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(WeaponSlot { weapon_entity: weapon_slot.weapon_entity })
            .insert(Name::new("DeathBall"))
            .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
            .insert(AimDirection { direction: Vec3::new(1.0, 0.0, 0.0) })
            .insert(AimAtClosestTargetBehavior)
            .insert(UnitRotation { angle: -death_ball.rotation_speed })
            .insert(Reload::new(40.0))
            .id();


        commands.entity(owner_entity).add_child(base);
        commands.entity(base).add_child(child);
    }
}
