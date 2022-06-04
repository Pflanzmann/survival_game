use bevy::prelude::{BuildChildren, Commands, EventReader, Name, Query, Res, SpriteSheetBundle, TextureAtlasSprite, Vec2, With};

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::projectile::Projectile;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::burning_shot::BurningShot;
use crate::models::time_alive::TimeAlive;

/// A system to split the [Projectile] that has [SplitShot] applied to it.
/// The shot gets split when the projectile stops.
pub fn burning_shot_system(
    mut command: Commands,
    atlas_handle: Res<AtlasHandles>,
    mut damaged_events: EventReader<DamagedEvent>,
    projectile_query: Query<(&Projectile, &BurningShot)>,
    already_burned_targets_query: Query<With<BurningShot>>
) {
    for event in damaged_events.iter() {
        let (projectile, burning_shot) = match projectile_query.get(event.source_entity) {
            Ok(transform) => transform,
            Err(_) => continue,
        };

        command.entity(event.target_entity)
            .with_children(|parent| {
            parent.spawn_bundle(
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(128.0, 256.0)),
                        ..Default::default()
                    },
                    texture_atlas: atlas_handle.burning_atlas.clone(),
                    ..Default::default()
                },
            )
                .insert(Name::new("Burning Zone"))
                .insert_bundle(DamageBundle::new(2.0, 60.0))
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(500.0) })
                .insert(EnemyHitBoxCollider)
                .insert(TimeAlive { time_alive: 5.0 })

                .insert(CurrentAnimationState { state: AnimationState::Idle })
                .insert(IdleAnimation::new(6, 0, 5.0));
        });
    }
}