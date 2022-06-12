use bevy::prelude::{Changed, Query, Transform, With};

use crate::models::player::Player;
use crate::models::ui::health_bar::HealthBar;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;

pub fn healthbar_update_system(
    mut health_bar_query: Query<(&mut Transform, &HealthBar)>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    for (mut health_bar_transform, health_bar) in health_bar_query.iter_mut() {
        let player_health = match player_query.get(health_bar.owner) {
            Ok(value) => value,
            Err(_) => continue
        };

        if player_health.get_current_health() / player_health.get_current_health() < 0.0 {
            health_bar_transform.scale.x = 0.01
        } else {
            health_bar_transform.scale.x = player_health.get_current_health() / player_health.get_total_amount();
        }
    }
}