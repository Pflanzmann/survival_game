use bevy::prelude::{Commands, EventReader, GlobalTransform, Name, Query, Res, SpriteSheetBundle, Transform};
use bevy::sprite::TextureAtlasSprite;
use rand::Rng;

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::models::animation::fade_animation::FadeAnimation;
use crate::models::collision::collider_owner::ColliderOwner;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_solid_body_collider::EnemySolidBodyCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::projectile_stopped_event::ProjectileStoppedEvent;
use crate::models::layerable::Layerable;
use crate::models::modifications::acid_puddle::{AcidPuddle, AcidPuddleOwner, AcidPuddleUnit};
use crate::models::time_alive::TimeAlive;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::SpriteLayer;

pub fn acid_puddle_system(
    mut commands: Commands,
    atlas_handle: Res<AtlasHandles>,
    mut projectile_stopped_event: EventReader<ProjectileStoppedEvent>,
    owner_query: Query<(&GlobalTransform, &AcidPuddle, &AcidPuddleOwner, &UnitSize)>,
) {
    for event in projectile_stopped_event.iter() {
        let (transform, acid_puddle, acid_puddle_owner, unit_size) = match owner_query.get(event.projectile_entity) {
            Ok(transform) => transform,
            Err(_) => continue,
        };

        let random_sprite_index: usize = rand::thread_rng().gen_range(0..6);

        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: random_sprite_index,
                custom_size: Some(unit_size.proportional_unit_size()),
                ..Default::default()
            },
            texture_atlas: atlas_handle.acid_puddle_atlas.clone(),
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(Name::new("AcidPuddle"))
            .insert(AcidPuddleUnit)

            .insert(HitBoxCollider { collider_type: ColliderType::Circle(0.0) })
            .insert(ColliderOwner(acid_puddle_owner.owner))
            .insert(EnemySolidBodyCollider)
            .insert(*unit_size)

            .insert(TimeAlive { time_alive: acid_puddle.time_alive })
            .insert(FadeAnimation { fade_time: -acid_puddle.time_alive })

            .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
        ;
    }
}