use bevy::ecs::component::Component;

pub enum AnimationState {
    Idle,
    WalkSide,
    WalkUp,
    WalkDown,
}

#[derive(Component)]
pub struct CurrentAnimationState {
    pub state: AnimationState,
}

impl CurrentAnimationState {
    pub fn new () -> CurrentAnimationState{
        CurrentAnimationState{ state : AnimationState::Idle }
    }
}
