use bevy::prelude::{Entity, Query, Res, ResMut, Time, Transform, Vec2, With};
use rand::random;

use crate::assets_handling::preload_stage_spawn_system::StageSpawnBehaviorHandle;
use crate::models::player::Player;
use crate::models::resources::world::spawn_phase_timer::SpawnPhaseTimer;
use crate::models::resources::world::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::world::spawn_timer::SpawnIntervalTimer;
use crate::models::spawner::spawn_task::SpawnTask;

pub fn spawn_scheduler_system(
    time: Res<Time>,
    mut spawn_interval_timer: ResMut<SpawnIntervalTimer>,
    mut spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    stage_spawn_behavior_handle: Res<StageSpawnBehaviorHandle>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    spawn_phase_timer.timer -= time.delta_seconds();
    if spawn_phase_timer.timer < 0.0 {
        spawn_phase_timer.current_spawn_phase += 1;
        if spawn_phase_timer.current_spawn_phase >= stage_spawn_behavior_handle.default_spawn_phase.spawn_phases.len() {
            spawn_phase_timer.current_spawn_phase = 0;
        }

        spawn_phase_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].duration;
        spawn_interval_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].spawn_interval;
    }

    spawn_interval_timer.timer -= time.delta_seconds();
    if spawn_interval_timer.timer > 0.0 {
        return;
    }
    spawn_interval_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].spawn_interval;

    for (player_entity, player_transform) in player_query.iter() {
        let total_spawn_weight: f32 = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter().map(|value| value.spawn_weight).sum();
        let random_number = random::<f32>() * total_spawn_weight;

        let mut previous_spawn_weights: f32 = 0.0;
        for enemy in stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter() {
            previous_spawn_weights += enemy.spawn_weight;

            if random_number < previous_spawn_weights {
                let random_x = random::<f32>() * 2.0 - 1.0;
                let random_y = random::<f32>() * 2.0 - 1.0;

                let direction_to_spawn = Vec2::new(random_x, random_y).normalize_or_zero();

                let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 15.0));
                spawn_task_receiver.push_new_task(SpawnTask::new(enemy.enemy_index, position_to_spawn, player_entity));
                return;
            }
        }
    }
}