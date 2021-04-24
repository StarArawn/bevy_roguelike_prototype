use bevy::{core::Time, ecs::component::Component};

pub trait Attribute : Component {
    fn update(&mut self, time: &Time);
    fn get_modified_value(&mut self) -> f32;
}