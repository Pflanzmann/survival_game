use bevy::prelude::{Commands, Query, Res, Time, Transform, Vec2};

use crate::models::collision::collider_type::ColliderType;
use crate::models::modifications::magnet::Magnet;
use crate::models::resources::collision::item_collision_quad_tree::{ItemCollisionQuadTree, ItemData};
use crate::models::unit_push::UnitPush;
use crate::util::quad_tree::QuadData;

pub fn magnet_system(
    mut commands: Commands,
    time: Res<Time>,
    item_tree: Res<ItemCollisionQuadTree>,
    mut magnet_query: Query<(&Transform, &mut Magnet)>,
) {
    for (transform, mut magnet) in magnet_query.iter_mut() {
        magnet.pulse_timer -= time.delta_seconds();
        if magnet.pulse_timer > 0.0 {
            continue;
        }
        magnet.pulse_timer = magnet.pulse_time;

        let mut check_entity_list: Vec<QuadData<ItemData>> = Vec::new();
        let size = Vec2::new(magnet.radius * 2.0, magnet.radius * 2.0);
        let self_position = transform.translation.truncate();
        item_tree.query_entities(
            &mut check_entity_list,
            &self_position,
            &size,
        );

        let collider_type = ColliderType::Circle(magnet.radius);
        for quad_data in check_entity_list.iter() {
            if collider_type.is_colliding(&self_position, &quad_data.data.collider_type, &quad_data.position) {
                commands.entity(quad_data.data.entity).insert(
                    UnitPush {
                        direction: (self_position - quad_data.position).normalize_or_zero(),
                        duration: magnet.push_duration,
                        force: magnet.push_force,
                    }
                );
            }
        }
    }
}