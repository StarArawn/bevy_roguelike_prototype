use crate::game::{gameplay::scenes, GameState};
use bevy::render::camera::CameraProjection;
use bevy::{
    // input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera,
};

use super::CustomOrthographicProjection;

pub struct KeyboardConf {
    pub forward: Box<[KeyCode]>,
    pub backward: Box<[KeyCode]>,
    pub left: Box<[KeyCode]>,
    pub right: Box<[KeyCode]>,
    pub move_sensitivity: f32,
}

impl Default for KeyboardConf {
    fn default() -> Self {
        KeyboardConf {
            forward: Box::new([KeyCode::W, KeyCode::Up]),
            backward: Box::new([KeyCode::S, KeyCode::Down]),
            left: Box::new([KeyCode::A, KeyCode::Left]),
            right: Box::new([KeyCode::D, KeyCode::Right]),
            move_sensitivity: 2.0,
        }
    }
}

pub fn camera_movement(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Camera,
        &mut Transform,
        &mut CustomOrthographicProjection,
    )>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if *game_state.current() == GameState::MapView {
            game_state.set(GameState::BattleView).unwrap();
            scenes::battle::spawn(
                scenes::battle::BattleLocation::Mountains,
                &mut commands,
                &asset_server,
                &mut materials,
                &mut texture_atlases,
            );
        } else if *game_state.current() == GameState::BattleView {
            game_state.set(GameState::MapView).unwrap();
        }
        keyboard_input.update();
    }

    if *game_state.current() == GameState::BattleView {
        return;
    }

    for (mut camera, mut transform, mut projection) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = projection.scale;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) && scale < 6.0 {
            let scale = ((scale + (time.delta_seconds() * 1.5)) * 100.0).round() / 100.0;
            projection.scale = scale;
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 0.5 {
            let scale = ((scale - (time.delta_seconds() * 1.5)) * 100.0).round() / 100.0;
            projection.scale = scale;
        }

        projection.update(
            windows.get_primary().unwrap().width(),
            windows.get_primary().unwrap().height(),
        );
        camera.projection_matrix = projection.get_projection_matrix();
        camera.depth_calculation = projection.depth_calculation();

        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}
