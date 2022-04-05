use bevy::prelude::{Query, With};

use crate::{Damage, UnitSize};
use crate::entities::modification_components::{GrowShot, ModContainerSlot};

pub fn grow_shot_system(
    mut bullet_query: Query<(&mut UnitSize, &mut Damage, &GrowShot)>,
) {
    for (mut unit_size, mut damage, grow_shot) in bullet_query.iter_mut() {
        unit_size.collider_size += grow_shot.size_step;
        damage.bonus_damage += grow_shot.damage_step;
    }
}