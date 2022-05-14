use bevy::prelude::{Commands, EventReader, Res, ResMut};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::damaged_effect::DamagedEffect;
use crate::models::events::damaged_event::DamagedEvent;

pub fn apply_damage_component_system(
    mut commands: Commands,
    mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut enemy_damaged_event: EventReader<DamagedEvent>,
) {
    for event in enemy_damaged_event.iter() {
        commands.entity(event.target_entity).insert(DamagedEffect::new());
        sound_manager.queue_sound(SoundHandleChannel::Misc(sound_handles.hit_sound.clone()));
    }
}
