use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Entity, Name, Query, Res, ResMut, SpriteSheetBundle, TextureAtlasSprite, Vec2, With};

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::aim_direction::AimDirection;
use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::animation::walking_animation_component::{MoveAnimationDown, MoveAnimationSide, MoveAnimationUp};
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::item_collider::ItemCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::gold_storage::GoldStorage;
use crate::models::gun::basic_sword::BasicSword;
use crate::models::gun::straight_basic_shot::StraightBasicShot;
use crate::models::input::player_aim_controlled::PlayerAimControlled;
use crate::models::input::player_move_controlled::PlayerMoveControlled;
use crate::models::layerable::Layerable;
use crate::models::main_camera::MainCamera;
use crate::models::mod_register::ModRegister;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn setup_player_system(
    mut commands: Commands,
    player_handles: Res<PlayerConfigHandles>,
    atlas_handles: ResMut<AtlasHandles>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    for camera_entity in camera_query.iter() {
        commands.spawn(
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y)),
                    ..Default::default()
                },
                texture_atlas: atlas_handles.player_atlas.clone(),
                ..Default::default()
            },
        )
            .insert(Name::new("Player"))
            .insert(Player)

            .insert(UnitSize::new_size(Vec2::new(player_handles.player_one.sprite_custom_size_x, player_handles.player_one.sprite_custom_size_y)))
            .insert(SolidBodyCollider { offset: Vec2::new(0.0, -player_handles.player_one.sprite_custom_size_y / 4.0), collider_type: ColliderType::Circle(player_handles.player_one.sprite_custom_size_x / 4.0) })
            .insert(ColliderWeight { weight: 0.8 })
            .insert(ItemCollider)

            .insert(DamageBundle::new(player_handles.player_one.damage, 60.0))

            .insert(MoveSpeed::new(player_handles.player_one.move_speed))
            .insert(MoveDirection::default())

            .insert(AimDirection::default())
            .insert(PlayerMoveControlled)
            .insert(PlayerAimControlled)

            .insert(Health::new(player_handles.player_one.health))
            .insert(StraightBasicShot)
            // .insert(BasicSword)
            .insert(Reload::new(player_handles.player_one.reload))
            .insert(ModRegister::default())

            .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
            .insert(SpriteFlip)

            .insert(GoldStorage::default())

            .insert(IdleAnimation::new(3, 0, 0.5))
            .insert(MoveAnimationSide::new(0.0, 4, 4, 15))
            .insert(MoveAnimationUp::new(0.0, 4, 5, 15))
            .insert(MoveAnimationDown::new(0.0, 4, 3, 15))
            .insert(CurrentAnimationState::new())

            .add_child(camera_entity);
    }
}
