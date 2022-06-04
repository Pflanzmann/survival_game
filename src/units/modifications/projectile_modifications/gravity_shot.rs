use bevy::prelude::{Commands, GlobalTransform, Query, Res, Time, Vec2};

use crate::models::collision::collider_type::ColliderType;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::resources::collision::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::models::unit_push::UnitPush;
use crate::util::quad_tree::QuadData;

pub fn gravity_shot_system(
    mut commands: Commands,
    time: Res<Time>,
    solid_body_tree: Res<SolidBodyQuadTree>,
    mut projectile_query: Query<(&GlobalTransform, &mut GravityShot)>,
) {
    for (transform, mut gravity_shot) in projectile_query.iter_mut() {
        gravity_shot.pulse_timer -= time.delta_seconds();
        if gravity_shot.pulse_timer > 0.0 {
            continue;
        }
        gravity_shot.pulse_timer = gravity_shot.pulse_time;

        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();
        let size = Vec2::new(gravity_shot.radius * 2.0, gravity_shot.radius * 2.0);
        let self_position = transform.translation.truncate();
        solid_body_tree.query_entities(
            &mut check_entity_list,
            &self_position,
            &size,
        );

        let collider_type = ColliderType::Circle(gravity_shot.radius);
        for quad_data in check_entity_list.iter() {
            if collider_type.is_colliding(&self_position, &quad_data.data.collider_type, &quad_data.position) {
                commands.entity(quad_data.data.entity).insert(
                    UnitPush {
                        direction: (self_position - quad_data.position).normalize_or_zero(),
                        duration: gravity_shot.push_duration,
                        force: gravity_shot.push_force,
                    }
                );
            }
        }
    }
}