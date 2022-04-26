use bevy::prelude::{Query, Res, ResMut, Time, Transform, Vec3, With};
use rand::random;

use crate::models::player::Player;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::spawn_timer::SpawnTimer;
use crate::models::spawner::spawn_task::SpawnTask;

pub fn spawn_scheduler_system(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.timer -= time.delta().as_secs_f32();
    if spawn_timer.timer > 0.0 {
        return;
    }
    spawn_timer.timer = 5.0;

    for transform in player_query.iter() {
        for _ in 0..6 {
            let random_x = random::<f32>() * 2.0 - 1.0;
            let random_y = random::<f32>() * 2.0 - 1.0;

            let direction_to_spawn = Vec3::new(random_x, random_y, 0.0).normalize_or_zero();

            let position_to_spawn = transform.translation + (direction_to_spawn * (256.0 * 5.0));
            spawn_task_receiver.push_new_task(SpawnTask::new(position_to_spawn));
        }
    }
}