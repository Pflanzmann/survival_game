use std::collections::VecDeque;
use bevy::prelude::Resource;

use crate::models::spawner::spawn_task::SpawnTask;

#[derive(Default, Resource)]
pub struct SpawnTaskReceiver {
    task_queue: VecDeque<SpawnTask>,
}

impl SpawnTaskReceiver {
    pub fn push_new_task(&mut self, spawn_task: SpawnTask) {
        self.task_queue.push_back(spawn_task);
    }

    pub fn consume_task(&mut self) -> Option<SpawnTask> {
        self.task_queue.pop_front()
    }
}