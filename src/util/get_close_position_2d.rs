use bevy::prelude::Vec2;
use rand::Rng;

pub fn get_close_position_2d(
    position_x: f32,
    position_y: f32,
    prox_min: f32,
    prox_max: f32,
) -> Vec2 {
    let mut result = Vec2::new(0.0, 0.0);

    let mut rng = rand::thread_rng();

    let rnd_signed_x = rng.gen_range(-1..1);
    let rnd_signed_y = rng.gen_range(-1..1);
    let rnd_x = rng.gen_range(prox_min..prox_max);
    let rnd_y = rng.gen_range(prox_min..prox_max);

    if rnd_signed_x < 0 {
        result.x = position_x - rnd_x
    } else {
        result.x = position_x + rnd_x
    }

    if rnd_signed_y < 0 {
        result.y = position_y - rnd_y
    } else {
        result.y = position_y + rnd_y
    }
    return result;
}