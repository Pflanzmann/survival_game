use bevy::prelude::{Commands, Entity, Query, Res, Time};

use crate::models::visited_shop::VisitedShop;

pub fn visited_shop_system(
    mut commands: Commands,
    time: Res<Time>,
    mut visited_query: Query<(Entity, &mut VisitedShop)>,
) {
    for (entity, mut visited_shop) in visited_query.iter_mut() {
        visited_shop.revisit_timer -= time.delta_seconds();

        if visited_shop.revisit_timer < 0.0 {
            commands.entity(entity).remove::<VisitedShop>();
        }
    }
}
