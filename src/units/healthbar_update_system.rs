use bevy::prelude::{Changed, Children, Commands, Entity, Query, With, World};
use bevy::sprite::collide_aabb::collide;

use crate::{HealthBar, Player, Transform, Without};
use crate::collision::collision_components::Collider;
use crate::components::unit_stats_components::{Damage, Enemy, Health, UnitSize};

pub fn healthbar_update_system(
    mut children_query: Query<&mut Transform, (With<HealthBar>, Without<Player>, Without<Enemy>)>,
    mut player_query: Query<(Entity, &Transform, &UnitSize, &mut Health, &mut Children), (With<Collider>, With<Player>, Without<Enemy>, Changed<Health>)>,
){
    for (player_entity, player_transform, player_size, mut player_health, mut children) in player_query.iter_mut() {
        for &child in children.iter() {
            let mut health_bar = match children_query.get_mut(child) {
                Ok(value) => value,
                Err(_) => continue
            };

            if player_health.current_health / player_health.max_health < 0.0 {
                health_bar.scale.x = 0.01
            } else {
                health_bar.scale.x = player_health.current_health / player_health.max_health;
            }
        }
    }
}

