use bevy::prelude::{Commands, Entity, EventWriter, GlobalTransform, Name, Query, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, With};
use crate::assets_handling::preload_audio_system::SoundHandles;

use crate::assets_handling::preload_bullet_system::BulletConfigHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::aim_direction::AimDirection;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::bullet::Bullet;
use crate::models::bundles::bullet_bundle::BulletBundle;
use crate::models::collision::collided_entities::CollidedEntities;
use crate::models::collision::collider_type::ColliderType;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::move_direction::MoveDirection;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::sprite_move_rotation::SpriteMoveRotation;
use crate::models::straight_basic_shot::StraightBasicShot;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;
use crate::TextureHandles;

pub fn straight_basic_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    bullet_handle: Res<BulletConfigHandles>,
    mut sound_manager : ResMut<SoundManager>,
    sound_handles : Res<SoundHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    mut weapon_holder_query: Query<(&GlobalTransform, &AimDirection, &WeaponSlot, &mut Reload)>,
    gun_query: Query<Entity, With<StraightBasicShot>>,
) {
    for (holder_transform, holder_aim_direction, weapon_holder_slot, mut holder_reloadable) in weapon_holder_query.iter_mut() {
        if holder_aim_direction.direction.length() == 0.0 {
            continue;
        }

        let gun_entity = match gun_query.get(weapon_holder_slot.weapon_entity) {
            Ok(gun) => gun,
            Err(_) => continue,
        };

        if holder_reloadable.reload_timer > 0.0 {
            continue;
        }
        holder_reloadable.reload_timer = holder_reloadable.get_total_amount();

        let bullet = command.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(holder_transform.translation.x, holder_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y)),
                ..Default::default()
            },
            texture: texture_handle.bullet_fireball.clone(),
            ..Default::default()
        })
            .insert_bundle(BulletBundle {
                bullet: Bullet { source_entity: gun_entity },
                unit_size: UnitSize { collider_size: Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y) },
                facing_direction: MoveDirection { direction: holder_aim_direction.direction },
                collider_entities: CollidedEntities::default(),
            }).insert(Name::new("Bullet"))
            .insert(SpriteMoveRotation)
            .insert(ColliderType::Circle(bullet_handle.basic_bullet.sprite_custom_size_x / 2.0))
            .id();

        bullet_shot_event_writer.send(BulletShotEvent { entity: bullet });

        sound_manager.queue_sound(SoundHandleChannel::Bullet(sound_handles.shoot_sound.clone()));
    }
}

