use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ShopButton {
    pub index : usize,
}

#[derive(Component)]
pub struct ShopSlot {
    pub index : usize
}

#[derive(Component)]
pub struct ShopMenuComp;


#[derive(Component)]
pub struct ToolTipField;