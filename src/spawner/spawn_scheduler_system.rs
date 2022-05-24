use bevy::prelude::{Entity, Query, Res, ResMut, Time, Transform, Vec2, With};
use rand::random;

use crate::models::player::Player;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::spawn_timer::SpawnTimer;
use crate::models::spawner::spawn_task::SpawnTask;

pub fn spawn_scheduler_system(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    spawn_timer.timer -= time.delta().as_secs_f32();
    if spawn_timer.timer > 0.0 {
        return;
    }
    spawn_timer.timer = 5.0;

    for (player_entity, player_transform) in player_query.iter() {
        for _ in 0..5 {
            let random_x = random::<f32>() * 2.0 - 1.0;
            let random_y = random::<f32>() * 2.0 - 1.0;

            let direction_to_spawn = Vec2::new(random_x, random_y).normalize_or_zero();

            let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 5.0));
            spawn_task_receiver.push_new_task(SpawnTask::new(0, position_to_spawn, player_entity));
        }
    }
}