use bevy::{prelude::*, render::camera::{Camera, CameraProjection, OrthographicProjection}};

use crate::game::{camera::{CameraData, CustomOrthographicProjection}, gameplay::character::PlayerSprite};

pub fn movement(
    time: Res<Time>,
    windows: Res<Windows>,
    player_query: Query<&Transform, (With<PlayerSprite>, Without<Camera>)>,
    mut camera_query: Query<(
        &mut CameraData,
        &mut Camera,
        &mut Transform,
        &mut OrthographicProjection,
    )>,
) {
    let mut player_position = Vec3::ZERO;
    for player_transform in player_query.iter() {
        player_position = player_transform.translation;
    }
    for (mut camera_data, mut camera, mut camera_transform, mut projection) in camera_query.iter_mut() {
        let camera_z = camera_transform.translation.z;
        
        // camera_transform.translation = camera_transform.translation.truncate().lerp(player_position.truncate(), 0.01).extend(camera_z);

        projection.update(
            windows.get_primary().unwrap().width(),
            windows.get_primary().unwrap().height(),
        );
        camera.projection_matrix = projection.get_projection_matrix();
        camera.depth_calculation = projection.depth_calculation();
    }
}