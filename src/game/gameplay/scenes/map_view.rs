use bevy::{prelude::*, render::camera::RenderLayers};
use bevy_tilemap::prelude::*;

use crate::game::{camera::{CurrentCamera, CustomOrthographicCameraBundle}};

pub fn spawn(
    mut commands: Commands,
    mut current_camera: ResMut<CurrentCamera>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    let camera_entity = commands
        .spawn()
        .insert_bundle(CustomOrthographicCameraBundle::new_2d())
        .insert(RenderLayers::layer(0))
        .id();

    current_camera.camera = Some(camera_entity);

    for mut tilemap in tilemap_query.iter_mut() {
        let map_width = tilemap.width().unwrap() as i32;
        let map_height = tilemap.height().unwrap() as i32;
        let half_map_width = map_width / 2;
        let half_map_height = map_height / 2;

        for x in -half_map_width..half_map_width {
            for y in -half_map_height..half_map_height {
                tilemap.spawn_chunk((x, y)).unwrap();
            }
        }
    }
}

pub fn destroy(
    mut commands: Commands,
    mut current_camera: ResMut<CurrentCamera>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    commands.entity(current_camera.camera.take().unwrap()).despawn_recursive();

    for mut tilemap in tilemap_query.iter_mut() {
        let map_width = tilemap.width().unwrap() as i32;
        let map_height = tilemap.height().unwrap() as i32;
        let half_map_width = map_width / 2;
        let half_map_height = map_height / 2;

        for x in -half_map_width..half_map_width {
            for y in -half_map_height..half_map_height {
                tilemap.despawn_chunk((x, y)).unwrap();
            }
        }
    }
}