use bevy::prelude::{Assets, AssetServer, Commands, Name, Res, ResMut, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite, Transform, Vec2, Vec3};
use bevy::sprite::Sprite;

use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::items::descriptor::item::Item;
use crate::models::items::shop::Shop;
use crate::models::layerable::Layerable;
use crate::SpriteLayer;
use crate::SpriteLayer::GroundLevel;

pub fn setup_shop_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spawn_position = Vec3::new(0.0, 1000.0, GroundLevel.get_layer_z());

    let positions: Vec<Vec2> = vec![
        Vec2::new(-256.0, 256.0),
        Vec2::new(-350.0, 0.0),
        Vec2::new(0.0, 256.0),
        Vec2::new(256.0, 256.0),
        Vec2::new(350.0, 0.0),
    ];

    //Shop Keeper
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(256.0, 256.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(spawn_position),
        texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/shop_keeper.png"), Vec2::new(16.0, 20.0), 4, 6)),
        ..Default::default()
    })
        .insert(Name::new("Shop"))
        .insert(Item)
        .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
        .insert(Shop)
        .insert(HitBoxCollider {
            collider_type: ColliderType::Circle(256.0 / 2.0)
        })

        .insert(IdleAnimation::new(3, 0, 0.5))
        .insert(CurrentAnimationState::new())
    ;


    for position in positions.iter() {
        commands.spawn_bundle(
            SpriteBundle {
                transform: Transform::from_translation(position.extend(0.0) + spawn_position),
                texture: asset_server.load("sprites/crate.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
            .insert(Name::new("Crate"))
            .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
            .insert(SolidBodyCollider { offset: Vec2::new(0.0, 00.0), collider_type: ColliderType::Rectangle(Vec2::new(256.0, 256.0)) })
            .insert(ColliderWeight { weight: 1.0 })
        ;

        // commands.entity(shop_entity).add_child(crate_entity);
    }
}