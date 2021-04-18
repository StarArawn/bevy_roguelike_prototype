use bevy::{
    prelude::*,
    render::{
        camera::{ActiveCameras, Camera},
        render_graph::base,
    },
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Generating,
    MapView,
    BattleView,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

pub fn update_visibility_for_state(
    game_state: Res<State<GameState>>,
    mut query: Query<(&mut Visible, &GameState)>,
) {
    for (mut visible, target_game_state) in query.iter_mut() {
        if game_state.current() == target_game_state {
            visible.is_visible = true;
        } else {
            visible.is_visible = true;
        }
    }
}

pub fn update_camera_for_state(
    mut active_cameras: ResMut<ActiveCameras>,
    game_state: Res<State<GameState>>,
    mut query: Query<(&mut Camera, &GameState)>,
) {
    for (mut camera, target_game_state) in query.iter_mut() {
        if game_state.current() == target_game_state {
            camera.name = Some(base::camera::CAMERA_2D.to_string());
        } else {
            camera.name = None;
        }
    }

    for active_camera in active_cameras.iter_mut() {
        active_camera.entity = None;
    }
}
