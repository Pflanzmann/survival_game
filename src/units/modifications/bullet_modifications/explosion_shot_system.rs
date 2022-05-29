use bevy::prelude::{Commands, EventReader, GlobalTransform, Query, Res, SpriteSheetBundle, Transform, Vec2};
use bevy::sprite::TextureAtlasSprite;
use rand::random;

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::models::animation::animation_state::AnimationState::Idle;
use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_solid_body_collider::EnemySolidBodyCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::time_alive::TimeAlive;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;

pub fn explosion_shot_system(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    mut damaged_events: EventReader<DamagedEvent>,
    source_query: Query<(&GlobalTransform, &Damage, &ExplosionShot)>,
) {
    for event in damaged_events.iter() {
        let (transform, damage, explosion_shot) = match source_query.get(event.source_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        let chance = random::<f32>();
        if chance > explosion_shot.explosion_chance {
            continue;
        }

        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(explosion_shot.radius * 2.0, explosion_shot.radius * 2.0)),
                ..Default::default()
            },
            texture_atlas: atlas_handles.explosion_atlas.clone(),
            transform: Transform::from_translation(transform.translation),
            ..Default::default()
        })
            .insert(IdleAnimation::new(0.0, 30, 0, explosion_shot.explosion_time_alive))
            .insert(CurrentAnimationState { state: Idle })
            .insert(TimeAlive { time_alive: explosion_shot.explosion_time_alive })
            .insert(DamagedEntities::default())
            .insert(Damage::new(damage.get_total_amount()))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(explosion_shot.radius) })
            .insert(EnemySolidBodyCollider)
        ;
    }
}