use std::f32::consts::PI;
use std::iter::repeat;
use std::thread::spawn;

use bevy::prelude::{Entity, Mat2, Query, Res, ResMut, Time, Transform, Vec2, With};
use rand::random;

use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::resources::world::spawn_interval_timer::SpawnIntervalTimer;
use crate::models::resources::world::spawn_phase_timer::SpawnPhaseTimer;
use crate::models::resources::world::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::spawner::spawn_pattern::SpawnPattern;
use crate::models::spawner::spawn_stage::SpawnStage;
use crate::models::spawner::spawn_task::SpawnTask;

pub fn spawn_scheduler_system(
    time: Res<Time>,
    mut spawn_interval_timer: ResMut<SpawnIntervalTimer>,
    mut spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    spawn_stage: Res<SpawnStage>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_count_query: Query<With<Enemy>>,
) {
    spawn_phase_timer.timer -= time.delta_seconds();
    if spawn_phase_timer.timer < 0.0 {
        spawn_phase_timer.current_spawn_phase += 1;
        if spawn_phase_timer.current_spawn_phase >= spawn_stage.spawn_phases.len() {
            spawn_phase_timer.current_spawn_phase = 0;
        }

        spawn_phase_timer.timer = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].duration;
        spawn_interval_timer.timer = 0.0;
    }

    spawn_interval_timer.timer -= time.delta_seconds();
    if spawn_interval_timer.timer > 0.0 {
        return;
    }
    spawn_interval_timer.timer = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].spawn_interval;

    let enemies_missing = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].minimum_enemy_amount - enemy_count_query.iter().count();
    ;

    match spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].pattern {
        SpawnPattern::Random { .. } => {
            random_spawn_pattern(spawn_phase_timer, spawn_task_receiver, spawn_stage, player_query, enemies_missing)
        }
        SpawnPattern::Circle { .. } => {
            circle_spawn_pattern(spawn_phase_timer, spawn_task_receiver, spawn_stage, player_query)
        }
        SpawnPattern::Grouped { .. } => {
            grouped_spawn_pattern(spawn_phase_timer, spawn_task_receiver, spawn_stage, player_query)
        }
        SpawnPattern::Directional { .. } => {
            directional_spawn_pattern(spawn_phase_timer, spawn_task_receiver, spawn_stage, player_query, enemies_missing)
        }
    }
}

fn random_spawn_pattern(
    spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    spawn_stage: Res<SpawnStage>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_to_spawn: usize,
) {
    let mut enemies_to_spawn = enemies_to_spawn;

    println!("repeat: {}", enemies_to_spawn);
    for (player_entity, player_transform) in player_query.iter() {
        while enemies_to_spawn > 0 {
            enemies_to_spawn -= 1;

            let total_spawn_weight: f32 = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter().map(|value| value.spawn_weight).sum();
            let random_number = random::<f32>() * total_spawn_weight;

            let mut previous_spawn_weights: f32 = 0.0;
            let mut enemy_index = 0;
            'enemy_loop: for enemy in spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter() {
                previous_spawn_weights += enemy.spawn_weight;

                if random_number < previous_spawn_weights {
                    enemy_index = enemy.enemy_index;
                    break 'enemy_loop;
                }
            };

            let random_x = random::<f32>() * 2.0 - 1.0;
            let random_y = random::<f32>() * 2.0 - 1.0;

            let direction_to_spawn = Vec2::new(random_x, random_y).normalize_or_zero();

            let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 15.0));
            spawn_task_receiver.push_new_task(SpawnTask::new(enemy_index, position_to_spawn, player_entity));
        }
    }
}

fn circle_spawn_pattern(
    spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    spawn_stage: Res<SpawnStage>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let spawn_angle = match spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].pattern {
        SpawnPattern::Circle { spawn_angle_in_degree } => spawn_angle_in_degree,
        _ => { return; }
    };

    let mut current_angle = spawn_angle;

    for (player_entity, player_transform) in player_query.iter() {
        let total_spawn_weight: f32 = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter().map(|value| value.spawn_weight).sum();
        let random_number = random::<f32>() * total_spawn_weight;

        let mut previous_spawn_weights: f32 = 0.0;
        let mut enemy_index = 0;
        'enemy_loop: for enemy in spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter() {
            previous_spawn_weights += enemy.spawn_weight;

            if random_number < previous_spawn_weights {
                enemy_index = enemy.enemy_index;
                break 'enemy_loop;
            }
        };

        while current_angle < 360.0 {
            let direction = Vec2::new(0.0, 1.0);

            let angle_radians = -current_angle * PI / 180.0;
            let rotation_matrix = Mat2::from_angle(angle_radians);

            let direction_to_spawn = (rotation_matrix * direction).normalize_or_zero();

            let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 15.0));
            spawn_task_receiver.push_new_task(SpawnTask::new(enemy_index, position_to_spawn, player_entity));
            current_angle += spawn_angle;
        }
    }
}

fn grouped_spawn_pattern(
    spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    spawn_stage: Res<SpawnStage>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let enemy_amount = match spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].pattern {
        SpawnPattern::Grouped { enemy_amount } => enemy_amount,
        _ => { return; }
    };

    for (player_entity, player_transform) in player_query.iter() {
        let total_spawn_weight: f32 = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter().map(|value| value.spawn_weight).sum();
        let random_number = random::<f32>() * total_spawn_weight;

        let mut previous_spawn_weights: f32 = 0.0;
        for enemy in spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter() {
            previous_spawn_weights += enemy.spawn_weight;

            if random_number < previous_spawn_weights {
                let random_x = random::<f32>() * 2.0 - 1.0;
                let random_y = random::<f32>() * 2.0 - 1.0;

                let direction_to_spawn = Vec2::new(random_x, random_y).normalize_or_zero();

                let box_position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 15.0));

                for index in 0..enemy_amount {
                    let x_offset = index % 5;
                    let y_offset = index / 5;
                    let offset_direction = Vec2::new(x_offset as f32, y_offset as f32) * 64.0;

                    let position_to_spawn = box_position_to_spawn + offset_direction;

                    spawn_task_receiver.push_new_task(SpawnTask::new(enemy.enemy_index, position_to_spawn, player_entity));
                }
                return;
            }
        }
    }
}

fn directional_spawn_pattern(
    spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    spawn_stage: Res<SpawnStage>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_to_spawn: usize,
) {
    let horizontal = match spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].pattern {
        SpawnPattern::Directional { horizontal } => horizontal,
        _ => { return; }
    };

    let mut enemies_to_spawn = enemies_to_spawn;

    while enemies_to_spawn > 0 {
        enemies_to_spawn -= 1;

        for (player_entity, player_transform) in player_query.iter() {
            let total_spawn_weight: f32 = spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter().map(|value| value.spawn_weight).sum();
            let random_number = random::<f32>() * total_spawn_weight;

            let mut previous_spawn_weights: f32 = 0.0;
            for enemy in spawn_stage.spawn_phases[spawn_phase_timer.current_spawn_phase].enemies.iter() {
                previous_spawn_weights += enemy.spawn_weight;

                if random_number < previous_spawn_weights {
                    let direction_to_spawn = if horizontal {
                        let random_x = if random::<bool>() { 1.0 } else { -1.0 };
                        let random_y = random::<f32>() * 2.0 - 1.0;

                        Vec2::new(random_x, random_y).normalize_or_zero()
                    } else {
                        let random_x = random::<f32>() * 2.0 - 1.0;
                        let random_y = if random::<bool>() { 1.0 } else { -1.0 };

                        Vec2::new(random_x, random_y).normalize_or_zero()
                    };

                    let position_to_spawn = player_transform.translation.truncate() + (direction_to_spawn * (256.0 * 15.0));
                    spawn_task_receiver.push_new_task(SpawnTask::new(enemy.enemy_index, position_to_spawn, player_entity));
                    return;
                }
            }
        }
    }
}