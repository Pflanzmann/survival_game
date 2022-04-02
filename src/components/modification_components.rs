use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CurveShot {
    pub curve_left: bool,
}

#[derive(Component)]
pub struct GrowShot {
    pub grow_step: f32,
}

#[derive(Component)]
pub struct SplitShot;