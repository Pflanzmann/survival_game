use bevy::prelude::{AssetServer, Commands, Entity, EventReader, Query, Res, ResMut, With};
use bevy_kira_audio::Audio;

use crate::{AppStateTrigger, ToAppState};
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::shop::Shop;

pub fn barrel_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    mut state_trigger: ResMut<AppStateTrigger>,
    item_query: Query<Entity, With<Shop>>,
) {
    for event in item_pickup_event.iter() {
        let _item = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        state_trigger.state_change_trigger = ToAppState::ToShop;
        commands.entity(event.item_entity).despawn();
    }
}