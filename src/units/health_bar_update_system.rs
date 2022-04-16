use bevy::prelude::{Changed, Children, Query, Transform, With, Without};

use crate::HealthBar;
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::health::Health;

pub fn healthbar_update_system(
    mut children_query: Query<&mut Transform, (With<HealthBar>, Without<Player>, Without<Enemy>)>,
    mut player_query: Query<(&Health, &Children), (With<Collider>, With<Player>, Without<Enemy>, Changed<Health>)>,
) {
    for (player_health, children) in player_query.iter_mut() {
        for &child in children.iter() {
            let mut health_bar = match children_query.get_mut(child) {
                Ok(value) => value,
                Err(_) => continue
            };

            if player_health.get_current_health() / player_health.get_current_health() < 0.0 {
                health_bar.scale.x = 0.01
            } else {
                health_bar.scale.x = player_health.get_current_health() / player_health.get_total_amount();
            }
        }
    }
}
