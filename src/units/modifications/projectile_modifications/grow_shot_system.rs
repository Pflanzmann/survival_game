use bevy::prelude::{Query, Res, Time, With};

use crate::models::modifications::grow_shot::GrowShot;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;

/// A system to grow the [Projectile] that has [GrowShot] applied to it.
/// This increases the [UnitSize] and [Damage] of the projectile depending on the distance traveled.
pub fn grow_shot_system(
    time: Res<Time>,
    mut projectile_query: Query<(&mut UnitSize, &mut Damage, &GrowShot), With<TravelRange>>,
) {
    for (mut unit_size, mut damage, grow_shot) in projectile_query.iter_mut() {
        damage.add_bonus_amount(grow_shot.damage_per_second * time.delta_seconds());
        unit_size.add_bonus_amount(grow_shot.size_per_second * time.delta_seconds());
    }
}
