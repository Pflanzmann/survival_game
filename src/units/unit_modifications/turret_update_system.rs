use bevy::prelude::{Commands, Entity, Name, Query, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Vec3, With, Without};
use rand::Rng;

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::modifications::turret::Turret;
use crate::models::turret_components::{TurretOwner, TurretUint};
use crate::models::weapon_slot::WeaponSlot;

pub fn turret_update_system(
    mut commands: Commands,
    mut texture_handler: ResMut<TextureHandles>,
    turret_owner: Query<(Entity, &Transform, &WeaponSlot), With<Turret>>,
    mut turret_query: Query<(Entity, &TurretOwner, &mut Transform), (With<TurretUint>, Without<Turret>)>,
) {
    let mut turret_exists = false;

    for (player_entity, player_transform, weapon_slot) in turret_owner.iter() {
        for (turret_entity, owner, mut turret_transform) in turret_query.iter_mut() {
            if player_entity == owner.owner {
                // later count turrets -> right now check if turret exists
                turret_exists = true;
            }

            //port to player
            //Vec3::distance()
            //let player_pos = Vec2::new(player_transform.translation.x,player_transform.translation.y);
            let distance = player_transform.translation.distance(turret_transform.translation.clone());
            println!("{}", distance);

            if distance > 2000.0 {
                let mut rng = rand::thread_rng();
                let mut current_transform = *player_transform;

                let mut rnd_signed_x = rng.gen_range(-1..1);
                let mut rnd_signed_y = rng.gen_range(-1..1);
                let mut rnd_x = rng.gen_range(300.0..1000.0);
                let mut rnd_y = rng.gen_range(300.0..1000.0);

                if rnd_signed_x < 0 {
                    current_transform.translation.x -= rnd_x
                }else {
                    current_transform.translation.x += rnd_x
                }

                if rnd_signed_y < 0 {
                    current_transform.translation.y -= rnd_y
                }else {
                    current_transform.translation.y += rnd_y
                }

                turret_transform.translation.x = current_transform.translation.x;
                turret_transform.translation.y = current_transform.translation.y;
            }

        }

        if !turret_exists {
            let mut turret_transform = *player_transform;

            let mut rng = rand::thread_rng();

            let mut rnd_signed_x = rng.gen_range(-1..1);
            let mut rnd_signed_y = rng.gen_range(-1..1);
            let mut rnd_x = rng.gen_range(300.0..1000.0);
            let mut rnd_y = rng.gen_range(300.0..1000.0);

            if rnd_signed_x < 0 {
                turret_transform.translation.x -= rnd_x
            }else {
                turret_transform.translation.x += rnd_x
            }

            if rnd_signed_y < 0 {
                turret_transform.translation.y -= rnd_y
            }else {
                turret_transform.translation.y += rnd_y
            }
            // spawn turret
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handler.turret_unit.clone(),
                transform: Transform::from_xyz(turret_transform.translation.x, turret_transform.translation.y, SpriteLayer::GroundLevel.get_layer_z()),
                ..Default::default()
            })
                .insert(TurretUint)
                .insert(TurretOwner{owner: player_entity})
                .insert(WeaponSlot {weapon_entity: weapon_slot.weapon_entity})
                .insert(Name::new("Turret"))
                .insert(AimDirection{direction: Vec3::new(1.0,0.0,0.0)})
            ;
        }
    }
}