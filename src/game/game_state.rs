use bevy::{ecs::component::Component, prelude::*};

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

// pub trait EntityRenderingState : Component {
//     fn spawn(
//         &mut self,
//         game_state: &GameState,
//         commands: &mut Commands,
//         asset_server: &Res<AssetServer>,
//         materials: &mut ResMut<Assets<ColorMaterial>>,
//         texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     );

//     fn entity(&self) -> Entity where Self: 'static;
// }

// pub fn spawn_rendering_entities(
//     mut commands: Commands,
//     game_state: Res<State<GameState>>,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     mut query: Query<&mut Box<dyn EntityRenderingState>>,
// ) {
//     if game_state.is_changed() {
//         dbg!("Spawned the rendering entities");
//         for mut game_state_rendering in query.iter_mut() {
//             game_state_rendering.spawn(game_state.current(), &mut commands, &asset_server, &mut materials, &mut texture_atlases);
//         }
//     }
// }

// pub fn destroy_rendering_entities(
//     mut commands: Commands,
//     game_state: Res<State<GameState>>,
//     mut query: Query<&Box<dyn EntityRenderingState>>,
// ) {
//     if game_state.is_changed() {
//         dbg!("Destroyed the rendering entities");
//         for game_state_rendering in query.iter_mut() {
//             commands.entity(game_state_rendering.entity()).despawn_recursive();
//         }
//     }
// }