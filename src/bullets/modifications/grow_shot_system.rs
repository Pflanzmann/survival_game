use bevy::prelude::Query;

use crate::models::attributes::attribute::*;
use crate::models::attributes::damage::Damage;
use crate::models::modification_components::GrowShot;
use crate::models::unit_stats_components::UnitSize;

pub fn grow_shot_system(
    mut bullet_query: Query<(&mut UnitSize, &mut Damage, &GrowShot)>,
) {
    for (mut unit_size, mut damage, grow_shot) in bullet_query.iter_mut() {
        unit_size.collider_size += grow_shot.size_step;
        damage.add_bonus_amount(grow_shot.damage_step);
    }
}