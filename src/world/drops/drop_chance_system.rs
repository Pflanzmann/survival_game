use bevy::prelude::{Commands, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};
use rand::random;

use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::enemy::Enemy;
use crate::models::events::target_died_event::TargetDiedEvent;
use crate::models::items::coin::Coin;
use crate::models::items::descriptor::gold_value::GoldValue;
use crate::models::items::descriptor::heal::Heal;
use crate::models::items::descriptor::item::Item;
use crate::models::layerable::Layerable;
use crate::models::sprite_layer::SpriteLayer;

pub fn drop_chance_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<TargetDiedEvent>,
    texture_handles: Res<TextureHandles>,
    item_handles: Res<ItemConfigHandles>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for event in enemy_died_event.iter() {
        let enemy_position = match enemy_query.get(event.target_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };

        let mut drop_translation = enemy_position;
        drop_translation.z = 0.0;

        let random = random::<f32>() * 100.0;

        if (0.0..60.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.coin_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.coin.sprite_custom_size_x, item_handles.coin.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
                .insert(Coin)
                .insert(Name::new("Item Coin"))
                .insert(GoldValue { gold_value: 1 })
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(item_handles.coin.sprite_custom_size_x / 2.0) })
                .insert(ColliderWeight { weight: 0.0 });
        }

        if (80.0..100.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.hot_dog_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.hot_dog.sprite_custom_size_x, item_handles.hot_dog.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
                .insert(Heal { amount: item_handles.hot_dog.heal_amount })
                .insert(Name::new("Item Heal"))
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(item_handles.hot_dog.sprite_custom_size_x / 2.0) })
                .insert(ColliderWeight { weight: 0.0 });
        }
    }
}