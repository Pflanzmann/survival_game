use bevy::core::Timer;
use bevy::prelude::{Commands, Name, Res, ResMut, SpriteSheetBundle, TextureAtlasSprite, Transform, Vec2, Vec3};

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::aim_direction::AimDirection;
use crate::models::bundles::player_bundle::PlayerBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::input::player_aim_controlled::PlayerAimControlled;
use crate::models::input::player_move_controlled::PlayerMoveControlled;
use crate::models::mod_register::ModRegister;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;
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
            transform: Transform::from_xyz(0.0, 0.0, SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert_bundle(PlayerBundle {
            player: Player,
            unit_size: UnitSize { collider_size: Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y) },
            aim_direction: AimDirection { direction: Vec3::default() },
            move_direction: MoveDirection { direction: Vec3::default() },
            move_speed: MoveSpeed::new(player_handles.player_one.move_speed),
            damage: Damage::new(player_handles.player_one.damage),
            health: Health::new(player_handles.player_one.health),
        })
        .insert(ModRegister::default())
        .insert(PlayerMoveControlled)
        .insert(PlayerAimControlled)
        .insert(SpriteFlip)
        .insert(DamagedEntities::default())
        .insert(DamageInterval::new(60.0))
        .insert(ColliderWeight { weight: 0.8 })
        .insert(SolidBodyCollider {
            offset: Vec3::new(0.0, -80.0, 0.0),
            collider_type: ColliderType::Circle(player_handles.player_one.sprite_custom_size_x / 3.0),
        }).insert(AnimationTimer(Timer::from_seconds(0.3, true)));
}
