use bevy::prelude::*;

mod camera;
mod game_state;
mod map;

pub use game_state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(GameState::Generating)
            .add_system(camera::camera_movement.system())
            .add_plugin(map::MapPlugin);
    }
}