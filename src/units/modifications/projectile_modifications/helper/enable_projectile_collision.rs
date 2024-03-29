use bevy::prelude::{Commands, EventReader, Query};

use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::projectile::Projectile;

pub fn enable_projectile_collision(
    mut commands: Commands,
    mut projectile_shot_event: EventReader<ProjectileShotEvent>,
    projectile_query: Query<&Projectile>,
) {
    for event in projectile_shot_event.iter() {
        if projectile_query.get(event.entity).is_err() {
            continue;
        };

        commands.entity(event.entity).insert(EnemyHitBoxCollider);
    }
}
