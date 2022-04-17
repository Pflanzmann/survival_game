use bevy::prelude::{Commands, Entity, Input, KeyCode, Query, Res, With};

use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::mod_container::ModContainer;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;

pub fn gun_mod_debug_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    player_config: Res<PlayerConfigHandles>,
    mod_container_query: Query<(Entity, Option<&CurveShot>, Option<&GrowShot>, Option<&SplitShot>, Option<&Sprinting>), With<ModContainer>>,
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
        for (entity, curved_shot, _, _, _) in mod_container_query.iter() {
            if curved_shot.is_none() {
                commands.entity(entity).insert(CurveShot);
            } else {
                commands.entity(entity).remove::<CurveShot>();
            }
        }
    }

    //GrowShot
    if input.just_pressed(KeyCode::Numpad2) {
        for (entity, _, grow_shot, _, _) in mod_container_query.iter() {
            if grow_shot.is_none() {
                commands.entity(entity).insert(GrowShot { size_step: 10.0, damage_step: 0.1 });
            } else {
                commands.entity(entity).remove::<GrowShot>();
            }
        }
    }

    //SplitShot
    if input.just_pressed(KeyCode::Numpad3) {
        for (entity, _, _, split_shot, _) in mod_container_query.iter() {
            if split_shot.is_none() {
                commands.entity(entity).insert(SplitShot);
            } else {
                commands.entity(entity).remove::<SplitShot>();
            }
        }
    }

    //Sprinting
    if input.just_pressed(KeyCode::Numpad4) {
        for (entity, _, _, _, sprinting) in mod_container_query.iter() {
            if sprinting.is_none() {
                commands.entity(entity).insert(Sprinting);
            } else {
                commands.entity(entity).remove::<Sprinting>();
            }
        }
    }
}