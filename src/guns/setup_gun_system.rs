use bevy::prelude::{Commands, Entity, Query, With};

use crate::{Damage, Gunnable, Player};
use crate::components::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};

pub fn setup_gun_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in player_query.iter() {
        let gun = commands.spawn()
            .insert(Gunnable)
            .insert(StraightBasicShot)
            .insert(Reloadable::new(1.0))
            .id();

        commands.entity(entity).insert(WeaponSlot { entity: gun });
    }
}