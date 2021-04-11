use bevy::prelude::*;

use super::{GameState, map::get_has_map_assets};
use super::map::TilemapAtlasHandles;

// A system to determin if we have finished loading and should change states.
pub fn loading(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
) {
    if get_has_map_assets(asset_server, tilemap_atlas_handles) {
        dbg!("Changing state to generating");
        game_state.set(GameState::Generating).unwrap();
    }
}