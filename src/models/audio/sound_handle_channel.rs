use bevy::asset::Handle;
use bevy_kira_audio::AudioSource;

pub enum SoundHandleChannel {
    Pickup(Handle<AudioSource>),
    Bullet(Handle<AudioSource>),
    Misc(Handle<AudioSource>),
    Background(Handle<AudioSource>)
}