use bevy::math::EulerRot;
use bevy::prelude::{Commands, EventWriter, GlobalTransform, Name, Quat, Query, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, With};
use rand::random;

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::assets_handling::preload_bullet_system::BulletConfigHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::aim_direction::AimDirection;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::bullet::Bullet;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::gun::straight_basic_shot::StraightBasicShot;
use crate::models::move_direction::MoveDirection;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::sprite_move_rotation::SpriteMoveRotation;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;
use crate::TextureHandles;

pub fn straight_basic_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    bullet_handle: Res<BulletConfigHandles>,
    mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    mut weapon_holder_query: Query<(&GlobalTransform, &AimDirection, &WeaponSlot, &mut Reload), With<StraightBasicShot>>,
) {
    for (holder_transform, holder_aim_direction, weapon_holder_slot, mut holder_reloadable) in weapon_holder_query.iter_mut() {
        if holder_aim_direction.direction.length() == 0.0 {
            continue;
        }

        if holder_reloadable.reload_timer > 0.0 {
            continue;
        }
        holder_reloadable.reload_timer = holder_reloadable.get_total_amount();

        let random_rotation: f32 = random::<f32>() * 100.0;

        let mut bullet_transform = Transform::from_xyz(holder_transform.translation.x, holder_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z());
        bullet_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, random_rotation);

        let bullet = command.spawn_bundle(SpriteBundle {
            transform: bullet_transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y)),
                ..Default::default()
            },
            texture: texture_handle.bullet_fireball.clone(),
            ..Default::default()
        })
            .insert(Name::new("Projectile"))
            .insert(Bullet { source_entity: weapon_holder_slot.weapon_entity })

            .insert(UnitSize::new_size(Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y)))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(bullet_handle.basic_bullet.sprite_custom_size_x / 2.0) })
            .insert(EnemyHitBoxCollider)

            .insert_bundle(DamageBundle::new(0.0, 60.0))

            .insert(MoveSpeed::default())
            .insert(MoveDirection { direction: holder_aim_direction.direction })

            .insert(HitLimit::new(1.0))
            .insert(TravelRange::new(2048.0))

            .insert(UnitRotation { revolutions_per_min: if random_rotation > 50.0 { 40.0 } else { -40.0 } })
            .id();

        bullet_shot_event_writer.send(BulletShotEvent { entity: bullet });

        sound_manager.queue_sound(SoundHandleChannel::Bullet(sound_handles.shoot_sound.clone()));
    }
}

