use bevy::prelude::{Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};

use crate::{SpriteLayer, TextureHandles};
use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::slime::{Slime, SlimeUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn apply_slime_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Slime, With<Modification>>,
    owner_query: Query<(Entity, &Transform)>,
    unit_query: Query<&Owner, With<SlimeUnit>>,
) {
    for apply_event in apply_events.iter() {
        let _modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, owner_transform) = match owner_query.get(apply_event.target_entity) {
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

        let pos_vec = get_close_position_2d(*owner_transform, 300.0, 1000.0);

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: texture_handler.slime_unit.clone(),
            transform: Transform::from_xyz(pos_vec[0], pos_vec[1], SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(SlimeUnit)
            .insert(Owner::new(owner_entity))
            .insert(Name::new("Slime"))
            .insert(ChaseTargetBehavior { target: owner_entity, proximity: 200.0 })
            .insert(MoveDirection::default())
            .insert(MoveSpeed::new(6.0));
    }
}
