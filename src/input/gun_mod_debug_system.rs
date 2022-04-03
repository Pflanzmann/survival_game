use bevy::prelude::{Commands, Entity, Input, KeyCode, Res};
use rand::random;

use crate::{Health, Player, Query, With};
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::components::modification_components::{CurveShot, GrowShot, ModContainer, SplitShot};

pub fn gun_mod_debug_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    player_config: Res<PlayerConfigHandles>,
    mod_container_query: Query<(Entity, Option<&CurveShot>, Option<&GrowShot>, Option<&SplitShot>), With<ModContainer>>,
    player_query: Query<(Entity, Option<&Health>), With<Player>>,
) {
    //GodMode
    if input.just_pressed(KeyCode::Numpad0) {
        for (entity, health) in player_query.iter() {
            if health.is_none() {
                commands.entity(entity).insert(Health::new(player_config.player_one.health));
            } else {
                commands.entity(entity).remove::<Health>();
            }
        }
    }

    //CurvedShot
    if input.just_pressed(KeyCode::Numpad1) {
        for (entity, curved_shot, grow_shot, split_shot) in mod_container_query.iter() {
            if curved_shot.is_none() {
                commands.entity(entity).insert(CurveShot { curve_left: random() });
            } else {
                commands.entity(entity).remove::<CurveShot>();
            }
        }
    }

    //GrowShot
    if input.just_pressed(KeyCode::Numpad2) {
        for (entity, curved_shot, grow_shot, split_shot) in mod_container_query.iter() {
            if grow_shot.is_none() {
                commands.entity(entity).insert(GrowShot { grow_step: 10.0 });
            } else {
                commands.entity(entity).remove::<GrowShot>();
            }
        }
    }

    //SplitShot
    if input.just_pressed(KeyCode::Numpad3) {
        for (entity, curved_shot, grow_shot, split_shot) in mod_container_query.iter() {
            if split_shot.is_none() {
                commands.entity(entity).insert(SplitShot);
            } else {
                commands.entity(entity).remove::<SplitShot>();
            }
        }
    }
}