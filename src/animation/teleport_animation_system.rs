use bevy::core::Time;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Sprite, Transform, Without};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::behavior::teleporting_script::TeleportingScript;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn teleport_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut unit_query: Query<(Entity, &mut Transform, &mut TeleportingScript, &mut Sprite, &mut UnitSize), Without<Player>>,
) {
    for (entity, mut transform, mut teleporting, mut sprite, mut unit_size) in unit_query.iter_mut() {
        teleporting.timer += time.delta_seconds();

        let mut custom_size = match sprite.custom_size {
            Some(value) => value,
            None => continue,
        };

        if teleporting.timer < 0.5 && custom_size.x > 0.0 {
            custom_size.x -= 5.0;
            custom_size.y += 10.0;
            sprite.custom_size = Some(custom_size);
        }

        if teleporting.timer > 0.5 && teleporting.timer < 1.0 {
            if !teleporting.did_port {
                let pos_vec = get_close_position_2d(teleporting.target_pos.x, teleporting.target_pos.y, 250.0, 400.0);
                transform.translation.x = pos_vec[0];
                transform.translation.y = pos_vec[1];
                teleporting.did_port = true;
                sound_manager.queue_sound(SoundHandleChannel::Misc(sound_handles.teleport_sound.clone()));
            }
            custom_size.x += 5.0;
            custom_size.y -= 10.0;
            sprite.custom_size = Some(custom_size);
        }

        if teleporting.timer > 1.0 {
            if !teleporting.did_port {
                let pos_vec = get_close_position_2d(teleporting.target_pos.x, teleporting.target_pos.y, 250.0, 400.0);
                transform.translation.x = pos_vec[0];
                transform.translation.y = pos_vec[1];
                teleporting.did_port = true;
                sound_manager.queue_sound(SoundHandleChannel::Misc(sound_handles.teleport_sound.clone()));
            }
            unit_size.collider_size = unit_size.collider_size;
            commands.entity(entity).remove::<TeleportingScript>();
        }
    }
}