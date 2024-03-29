use bevy::asset::Handle;
use bevy_kira_audio::AudioSource;

pub enum SoundHandleChannel {
    Pickup(Handle<AudioSource>),
    Projectile(Handle<AudioSource>),
    Misc(Handle<AudioSource>),
    Background(Handle<AudioSource>),
}