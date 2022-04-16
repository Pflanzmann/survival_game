use bevy::prelude::{Transform, Vec2};
use rand::Rng;

pub fn get_close_position_2D(
    position: Transform
) -> Vec2 {
    let mut result = Vec2::new(0.0, 0.0);

    let mut rng = rand::thread_rng();

    let rnd_signed_x = rng.gen_range(-1..1);
    let rnd_signed_y = rng.gen_range(-1..1);
    let rnd_x = rng.gen_range(300.0..1000.0);
    let rnd_y = rng.gen_range(300.0..1000.0);

    if rnd_signed_x < 0 {
        result[0] = position.translation.x - rnd_x
    } else {
        result[0] = position.translation.x + rnd_x
    }

    if rnd_signed_y < 0 {
        result[1] = position.translation.y - rnd_y
    } else {
        result[1] = position.translation.y + rnd_y
    }
    return result;
}