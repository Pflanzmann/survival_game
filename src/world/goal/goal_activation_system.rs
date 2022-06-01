use bevy::prelude::{EventWriter, GlobalTransform, Query, Res, ResMut, Time, With};

use crate::{AppStateTrigger, ToAppState};
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::events::level_finished_event::LevelFinishedEvent;
use crate::models::player::Player;
use crate::models::world::goal_activation_progress::GoalActivationProgress;

pub fn goal_activation_system(
    time: Res<Time>,
    mut level_finished_event: EventWriter<LevelFinishedEvent>,
    mut state_trigger: ResMut<AppStateTrigger>,
    player_query: Query<(&GlobalTransform, &SolidBodyCollider), With<Player>>,
    mut goal_query: Query<(&GlobalTransform, &HitBoxCollider, &mut GoalActivationProgress)>,
) {
    for (player_transform, player_collider) in player_query.iter() {
        for (goal_transform, goal_collider, mut goal_activation_progress) in goal_query.iter_mut() {
            if goal_collider.collider_type.is_colliding(
                &goal_transform.translation.truncate(),
                &player_collider.collider_type,
                &player_transform.translation.truncate(),
            ) {
                goal_activation_progress.progress += time.delta_seconds();

                println!("goal time: {}", goal_activation_progress.progress );

                if goal_activation_progress.progress > 15.0 {
                    level_finished_event.send(LevelFinishedEvent);
                    state_trigger.state_change_trigger = ToAppState::ToGameWon;
                }
            }
        }
    }
}