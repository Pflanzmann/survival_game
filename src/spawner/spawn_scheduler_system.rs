use bevy::prelude::{Entity, Query, Res, ResMut, Time, Transform, Vec2, With};
use rand::{random, Rng};

use crate::assets_handling::preload_stage_spawn_system::StageSpawnBehaviorHandle;
use crate::models::player::Player;
use crate::models::resources::spawn_phase_timer::SpawnPhaseTimer;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::spawn_timer::SpawnIntervalTimer;
use crate::models::spawner::spawn_task::SpawnTask;

pub fn spawn_scheduler_system(
    time: Res<Time>,
    mut spawn_interval_timer: ResMut<SpawnIntervalTimer>,
    mut spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    stage_spawn_behavior_handle: Res<StageSpawnBehaviorHandle>,
) {
    spawn_phase_timer.timer -= time.delta().as_secs_f32();
    if spawn_interval_timer.timer < 0.0 {
        spawn_phase_timer.current_spawn_phase += 1;
        spawn_phase_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].duration;
    }

    spawn_interval_timer.timer -= time.delta().as_secs_f32();
    if spawn_interval_timer.timer > 0.0 {
        return;
    }
    spawn_interval_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].spawn_interval;

    for (player_entity, player_transform) in player_query.iter() {
        let enemy_index = rand::thread_rng().gen_range(0..stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.len());

        let random_x = random::<f32>() * 2.0 - 1.0;
        let random_y = random::<f32>() * 2.0 - 1.0;

        let direction_to_spawn = Vec2::new(random_x, random_y).normalize_or_zero();

        let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 12.0));
        spawn_task_receiver.push_new_task(SpawnTask::new(enemy_index, position_to_spawn, player_entity));
    }
}