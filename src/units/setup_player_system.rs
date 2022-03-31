use bevy::prelude::{Commands, Name, Res, Sprite, SpriteBundle, Vec2, Vec3};

use crate::{Collider, Damage, FacingDirection, Health, MoveSpeed, Player, UnitSize};
use crate::assets_handling::preload_texture_system::TextureHandles;

pub fn setup_player_system(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
) {
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: texture_handles.player_sprite.clone(),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(MoveSpeed { move_speed: 10.0 })
        .insert(UnitSize { collider_size: Vec2::new(256.0, 256.0) })
        .insert(Collider)
        .insert(FacingDirection { facing_direction: Vec3::new(1.0, 0.0, 0.0) })
        .insert(Damage { damage: 5.0 })
        .insert(Health { current_health: 50.0, max_health: 50.0 });
}
