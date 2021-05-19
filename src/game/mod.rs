use bevy::{prelude::*, render::RenderSystem};

pub mod animation;
pub mod camera;
pub mod game_state;
pub mod gameplay;
pub mod loading;
pub mod map;
pub mod timing;
pub mod helpers;

use self::{camera::CustomOrthographicProjection, gameplay::scenes};
pub use game_state::GameState;

pub struct GamePlugin;

#[derive(Default)]
pub struct LoadingHandles(Vec<HandleUntyped>);

fn load_data(
    asset_server: Res<AssetServer>,
    mut loading_handles: ResMut<LoadingHandles>,
) {
    let load_handles = asset_server.load_folder("textures").unwrap();
    asset_server.load::<Font, &'static str>("FiraMono-Medium.ttf");

    loading_handles.0 = load_handles;
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::default())
            .init_resource::<LoadingHandles>()
            .init_resource::<timing::Timing>()
            .init_resource::<camera::CurrentCamera>()
            .register_type::<CustomOrthographicProjection>()
            .add_event::<scenes::battle::BattleEvent>()
            .add_startup_system(load_data.system())
            .add_system(timing::update.system())
            .add_system(animation::animate_sprite_system.system())
            .add_system(scenes::battle::handle_battle_events.system())
            .add_system(gameplay::stats::update_max_stats.system())
            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(loading::loading.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::SpawnMap)
                    .with_system(map::spawn_map_entity.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GenerateMap)
                    .with_system(
                        map::generate_map.system()
                    )
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GenerateRoads)
                    .with_system(
                        map::generate_road.system()
                        .label("generate_roads")
                    )
                    .with_system(gameplay::character::spawn_player.system().after("generate_roads")),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::MapView)
                    .with_system(gameplay::scenes::map_view::spawn.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MapView)
                    .with_system(gameplay::scenes::map_view::destroy.system()),
            )
            .add_system_set(
                // Gameplay update
                SystemSet::on_update(GameState::MapView)
                    .label("gameplay_update")
                    .with_system(gameplay::camera::movement.system())
                    .with_system(gameplay::character::movement.system())
                    .with_system(gameplay::character::collision::check.system())
            )
            .add_system_set(
                // Realtime update
                // Used for non-gameplay items that should update every frame.
                SystemSet::on_update(GameState::MapView)
                    .label("realtime_update")
                    .with_system(camera::camera_movement.system())
                    .with_system(gameplay::enemy::spawner::tick.system()),
            )
            // Battle View
            .add_system_set(
                SystemSet::on_update(GameState::BattleView)
                    .with_system(camera::camera_movement.system())
                    .with_system(scenes::battle::update_health_text.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::BattleView)
                    .with_system(scenes::battle::clear_battle_screen.system()),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                bevy::render::camera::camera_system::<CustomOrthographicProjection>
                    .system()
                    .before(RenderSystem::VisibleEntities),
            );
    }
}
