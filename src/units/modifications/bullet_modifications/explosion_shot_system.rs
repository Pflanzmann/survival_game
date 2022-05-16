use bevy::prelude::{AssetServer, Commands, EventReader, Query, Res, Sprite, SpriteBundle, Transform, Vec2};
use rand::random;

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_solid_body_collision::EnemySolidBodyCollision;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::time_alive::TimeAlive;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;

pub fn explosion_shot_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut damaged_events: EventReader<DamagedEvent>,
    source_query: Query<(&Transform, &Damage, &ExplosionShot)>,
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

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(explosion_shot.radius * 2.0, explosion_shot.radius * 2.0)),
                ..Default::default()
            },
            texture: asset_server.load(&explosion_shot.explosion_sprite_path),
            transform: Transform::from_translation(transform.translation),
            ..Default::default()
        })
            .insert(TimeAlive { time_alive: explosion_shot.explosion_time_alive })
            .insert(DamagedEntities::default())
            .insert(Damage::new(damage.get_total_amount()))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(explosion_shot.radius) })
            .insert(EnemySolidBodyCollision)
        ;
    }
}