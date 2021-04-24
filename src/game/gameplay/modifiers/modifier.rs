use bevy::{core::Time, ecs::component::Component};

pub trait Modifier : Component {
    fn modify_tick(&mut self, time: &Time, current: f32, max: f32) -> (f32, bool);
    fn modify(&mut self, current: f32, max: f32) -> f32;
    fn get_type(&self) -> ModifierType;
}

#[derive(Debug)]
pub enum ModifierType {
    TICK, // Runs every X seconds.
    PERMA, // Always applies to calculate final attribute value until its removed
    ONCE, // Applies once and then disappears.
}