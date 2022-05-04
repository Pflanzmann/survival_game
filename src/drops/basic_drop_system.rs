use bevy::prelude::{Commands, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};
use rand::random;

use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::collision::collider_type::ColliderType;
use crate::models::enemy::Enemy;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::items::coin::Coin;
use crate::models::items::descriptor::gold_value::GoldValue;
use crate::models::items::descriptor::heal::Heal;
use crate::models::items::descriptor::item::Item;
use crate::models::items::shop::Shop;
use crate::models::sprite_layer::SpriteLayer;

pub fn basic_drop_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<EnemyDiedEvent>,
    texture_handles: Res<TextureHandles>,
    item_handles: Res<ItemConfigHandles>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for event in enemy_died_event.iter() {
        let enemy_position = match enemy_query.get(event.enemy_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };

        let mut drop_translation = enemy_position;
        drop_translation.z = SpriteLayer::LowGroundLevel.get_layer_z();

        let random = random::<f32>() * 100.0;

        if (0.0..20.0).contains(&random) {
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
                .insert(Coin)
                .insert(Name::new("Item Coin"))
                .insert(GoldValue { gold_value: 1 })
                .insert(ColliderType::Circle(64.0));
        }

        if (20.0..40.0).contains(&random) {
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
                .insert(Heal { amount: item_handles.hot_dog.heal_amount })
                .insert(Name::new("Item Heal"))
                .insert(ColliderType::Circle(item_handles.hot_dog.sprite_custom_size_x / 2.0));
        }

        if (40.0..100.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.barrel_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.barrel.sprite_custom_size_x, item_handles.barrel.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Shop)
                .insert(Name::new("Item Barrel"))
                .insert(ColliderType::Circle(item_handles.barrel.sprite_custom_size_x / 2.0));
        }
    }
}