use bevy::{prelude::*, render::camera::RenderLayers};

use crate::game::{camera::{CurrentCamera}};

pub fn spawn(
    mut commands: Commands,
    mut current_camera: ResMut<CurrentCamera>,
) {
    let mut ortho = OrthographicCameraBundle::new_2d();
    ortho.orthographic_projection.scale = 0.5;

    let camera_entity = commands
        .spawn()
        .insert_bundle(ortho)
        .insert(RenderLayers::layer(0))
        .id();

    current_camera.camera = Some(camera_entity);
}

pub fn destroy(
    mut commands: Commands,
    mut current_camera: ResMut<CurrentCamera>,
) {
    commands.entity(current_camera.camera.take().unwrap()).despawn_recursive();
}