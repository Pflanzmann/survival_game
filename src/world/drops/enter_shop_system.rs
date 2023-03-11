use bevy::prelude::{Commands, Entity, EventReader, NextState, Query, Res, ResMut, With};

use crate::AppState;
use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::shop::Shop;
use crate::models::resources::shop_customer::ShopCustomer;
use crate::models::visited_shop::VisitedShop;

pub fn enter_shop_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    mut shop_customer: ResMut<ShopCustomer>,
    mut next_state: ResMut<NextState<AppState>>,
    sound_handles: Res<SoundHandles>,
    mut sound_manager: ResMut<SoundManager>,
    item_query: Query<Entity, With<Shop>>,
    visitor_query: Query<Entity, With<VisitedShop>>,
) {
    for event in item_pickup_event.iter() {
        let _item = match item_query.get(event.source_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        if visitor_query.get(event.target_entity).is_ok() {
            continue;
        }

        next_state.set(AppState::Shop);
        shop_customer.customer = Some(event.target_entity);

        sound_manager.queue_sound(SoundHandleChannel::Pickup(sound_handles.coin_pickup_sound.clone()));

        commands.entity(event.target_entity).insert(VisitedShop::new(2.0));
    }
}
