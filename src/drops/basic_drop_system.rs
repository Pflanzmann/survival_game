use bevy::prelude::{AssetServer, EventReader, Res, Sprite, SpriteBundle, Vec2};
use crate::{Commands, Transform, Vec3};
use crate::components::event_components::EnemyDiedEvent;

pub fn basic_drop_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<EnemyDiedEvent>,
    asset_server: Res<AssetServer>,
){
    for event in enemy_died_event.iter(){

        let mut drop_translation = event.death_position;
        drop_translation.z += 1.0;

        commands.spawn_bundle(
            SpriteBundle{
                transform: Transform::from_translation(drop_translation),
                texture: asset_server.load("basic_drop.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}