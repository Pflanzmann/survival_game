use bevy::prelude::{*};
use bevy::sprite::collide_aabb::collide;
use crate::{Collider, Health, Player, UnitSize};
use crate::components::item_components::Item;
use crate::components::unit_stats_components::Enemy;

pub fn item_player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &UnitSize, &mut Health, &mut Children), (With<Collider>, With<Player>, Without<Enemy>)>,
    mut item_query: Query<(Entity, &Transform, &UnitSize), With<Item>>
){
    for (player_entity, player_transform, player_size, mut player_health, mut children) in player_query.iter_mut() {
        for (item_entity, item_transform, item_size) in item_query.iter() {
            if collide(
                item_transform.translation,
                item_size.collider_size,
                player_transform.translation,
                player_size.collider_size,
            ).is_some(){
                commands.entity(item_entity).despawn();
            }
        }
    }
}