mod input;
mod ortho;
use bevy::prelude::Entity;
pub use input::camera_movement;
pub use ortho::{CustomOrthographicCameraBundle, CustomOrthographicProjection};

#[derive(Default)]
pub struct CurrentCamera {
    pub camera: Option<Entity>,
}