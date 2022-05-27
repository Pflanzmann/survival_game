use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct ShopButton {
    pub index: usize,
    pub modification_entity: Entity,
    pub price: i32,
}

#[derive(Component)]
pub struct ShopSlot {
    pub index: usize,
}

#[derive(Component)]
pub struct ShopMenuComp;


#[derive(Component)]
pub struct ToolTipField;