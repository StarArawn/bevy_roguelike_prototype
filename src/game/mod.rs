use bevy::{prelude::*, render::RenderSystem};

mod camera;
mod game_state;
mod gameplay;
mod loading;
mod map;
mod timing;

pub use game_state::GameState;
use self::camera::CustomOrthographicProjection;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(GameState::default())
            .init_resource::<timing::Timing>()
            .register_type::<CustomOrthographicProjection>()
            .add_system(timing::update.system())
            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(loading::loading.system())
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Generating)
                    .with_system(map::spawn_map_entity.system())
                        .label("spawn_map")
            )
            .add_system_set(
                SystemSet::on_update(GameState::Generating)   
                    .with_system(map::generate_map.system())
                    .after("spawn_map")
                    .with_system(gameplay::player::spawn_player.system())
                    .after("spawn_map")
            )
            .add_system_set(
                // Gameplay update
                SystemSet::on_update(GameState::MapView)
                    .label("gameplay_update")
                    .with_system(gameplay::player::movement.system())                   
            )
            .add_system_set(
                // Realtime update
                // Used for non-gameplay items that should update every frame.
                SystemSet::on_update(GameState::MapView)
                    .label("realtime_update")
                    .with_system(camera::camera_movement.system())
                    .with_system(gameplay::enemy::spawner::tick.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::BattleView)
                    .with_system(camera::camera_movement.system())
            )
            // Update visibilty between states.
            .add_system_set(
                SystemSet::on_enter(GameState::BattleView)
                    .with_system(game_state::update_visibility_for_state.system())
                    .with_system(game_state::update_camera_for_state.system())
            )
            .add_system_set(
                SystemSet::on_enter(GameState::MapView)
                    .with_system(game_state::update_visibility_for_state.system())
                    .with_system(game_state::update_camera_for_state.system())
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                bevy::render::camera::camera_system::<CustomOrthographicProjection>
                    .system()
                    .before(RenderSystem::VisibleEntities),
            )
            .add_plugin(map::MapPlugin);
    }
}
