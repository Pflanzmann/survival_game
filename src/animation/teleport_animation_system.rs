use bevy::core::Time;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Sprite, Transform, Without};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::behavior::teleporting_script::TeleportingScript;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn teleport_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut unit_query: Query<(Entity, &mut Transform, &mut TeleportingScript, &mut Sprite, &mut UnitSize), Without<Player>>,
) {
    for (entity, mut transform, mut teleporting, mut sprite, mut unit_size) in unit_query.iter_mut() {
        teleporting.progress += time.delta_seconds();

        let mut custom_size = match sprite.custom_size {
            Some(value) => value,
            None => continue,
        };

        if teleporting.progress < (teleporting.duration / 2.0) && custom_size.x > 0.0 {
            custom_size.x -= time.delta_seconds() / (teleporting.duration / 2.0) * unit_size.proportional_unit_size().x;
            custom_size.y += time.delta_seconds() / (teleporting.duration / 2.0) * unit_size.proportional_unit_size().y;
            sprite.custom_size = Some(custom_size);
        }

        if teleporting.progress > (teleporting.duration / 2.0) && teleporting.progress < teleporting.duration {
            if !teleporting.did_port {
                let pos_vec = teleporting.target_pos;
                transform.translation.x = pos_vec[0];
                transform.translation.y = pos_vec[1];
                teleporting.did_port = true;
                sound_manager.queue_sound(SoundHandleChannel::Misc(sound_handles.teleport_sound.clone()));
            }
            custom_size.x += time.delta_seconds() / (teleporting.duration / 2.0) * unit_size.proportional_unit_size().x;
            custom_size.y -= time.delta_seconds() / (teleporting.duration / 2.0) * unit_size.proportional_unit_size().y;
            sprite.custom_size = Some(custom_size);
        }

        if teleporting.progress > teleporting.duration {
            if !teleporting.did_port {
                let pos_vec = teleporting.target_pos;
                transform.translation.x = pos_vec[0];
                transform.translation.y = pos_vec[1];
                teleporting.did_port = true;
                sound_manager.queue_sound(SoundHandleChannel::Misc(sound_handles.teleport_sound.clone()));
            }
            unit_size.add_bonus_amount(0.0);
            commands.entity(entity).remove::<TeleportingScript>();
        }
    }
}