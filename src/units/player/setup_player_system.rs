use bevy::core::Timer;
use bevy::prelude::{Commands, Name, Res, ResMut, SpriteSheetBundle, TextureAtlasSprite, Vec2};

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::aim_direction::AimDirection;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::input::player_aim_controlled::PlayerAimControlled;
use crate::models::input::player_move_controlled::PlayerMoveControlled;
use crate::models::layerable::Layerable;
use crate::models::mod_register::ModRegister;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_size::UnitSize;
use crate::units::player::animate_player_system::AnimationTimer;

pub fn setup_player_system(
    mut commands: Commands,
    player_handles: Res<PlayerConfigHandles>,
    atlas_handles: ResMut<AtlasHandles>,
) {
    commands.spawn_bundle(
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y)),
                ..Default::default()
            },
            texture_atlas: atlas_handles.player_idle_atlas.clone(),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert(Player)

        .insert(UnitSize { unit_size: Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y) })
        .insert(SolidBodyCollider { offset: Vec2::new(0.0, -player_handles.player_one.sprite_custom_size_x / 4.0), collider_type: ColliderType::Circle(player_handles.player_one.sprite_custom_size_x / 4.0) })
        .insert(ColliderWeight { weight: 0.8 })

        .insert_bundle(DamageBundle::new(player_handles.player_one.damage, 60.0))

        .insert(MoveSpeed::new(player_handles.player_one.move_speed))
        .insert(MoveDirection::default())

        .insert(AimDirection::default())
        .insert(PlayerMoveControlled)
        .insert(PlayerAimControlled)

        .insert(Health::new(player_handles.player_one.health))
        .insert(ModRegister::default())

        .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
        .insert(SpriteFlip)

        .insert(AnimationTimer(Timer::from_seconds(0.3, true)));
}
