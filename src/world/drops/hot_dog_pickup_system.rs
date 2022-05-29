use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, Query, Res, ResMut, With};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::collision::item_collider::ItemCollider;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::descriptor::heal::Heal;
use crate::models::unit_attributes::health::Health;

pub fn hot_dog_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    sound_handles: Res<SoundHandles>,
    mut sound_manager: ResMut<SoundManager>,
    mut player_query: Query<&mut Health, With<ItemCollider>>,
    item_query: Query<&Heal>,
) {
    for event in item_pickup_event.iter() {
        let item_heal = match item_query.get(event.source_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        let mut player_health = match player_query.get_mut(event.target_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        player_health.heal(item_heal.amount);

        sound_manager.queue_sound(SoundHandleChannel::Pickup(sound_handles.coin_pickup_sound.clone()));

        commands.entity(event.source_entity).despawn_recursive();
    }
}