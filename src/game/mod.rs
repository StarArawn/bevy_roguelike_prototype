use bevy::prelude::*;

mod camera;
mod game_state;
mod map;
mod player;
mod loading;

pub use game_state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(GameState::default())
            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(loading::loading.system())
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Generating)
                    .with_system(map::spawn_map_entity.system())
            )    
            .add_system(map::generate_map.system())
            .add_system(camera::camera_movement.system())
            .add_plugin(map::MapPlugin);
    }
}