use bevy::prelude::{Commands, Name, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3};

use crate::{Damage, FacingDirection, Health, MoveSpeed, Player, UnitSize};
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::bundles::player_bundle::PlayerBundle;
use crate::models::collider::collider::Collider;
use crate::models::sprite_layer::SpriteLayer;

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
            transform: Transform::from_xyz(0.0, 0.0, SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert_bundle(PlayerBundle {
            player: Player,
            collider: Collider,
            unit_size: UnitSize { collider_size: Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y) },
            facing_direction: FacingDirection { facing_direction: Vec3::new(1.0, 0.0, 0.0) },
            move_speed: MoveSpeed { move_speed: player_handles.player_one.move_speed },
            damage: Damage::new(player_handles.player_one.damage),
            health: Health::new(player_handles.player_one.health),
        });
}
