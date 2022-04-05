use bevy::prelude::{Commands, Name, Res, Sprite, SpriteBundle, Vec2, Vec3};

use crate::{Damage, FacingDirection, Health, MoveSpeed, Player, UnitSize};
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::entities::collider::collider::Collider;

pub fn setup_player_system(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    player_handles: Res<PlayerConfigHandles>,
) {
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y)),
                ..Default::default()
            },
            texture: texture_handles.player_sprite.clone(),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(MoveSpeed { move_speed: player_handles.player_one.move_speed })
        .insert(UnitSize { collider_size: Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y) })
        .insert(Collider)
        .insert(FacingDirection { facing_direction: Vec3::new(1.0, 0.0, 0.0) })
        .insert(Damage::new(player_handles.player_one.damage))
        .insert(Health { current_health: player_handles.player_one.health, max_health: player_handles.player_one.health });
}
