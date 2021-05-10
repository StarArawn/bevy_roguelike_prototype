use bevy::{asset::LoadState, prelude::*};
use super::map::TilemapAtlasHandles;
use super::{map::get_has_map_assets, GameState};

// A system to determin if we have finished loading and should change states.
pub fn loading(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
    textures: Res<Assets<Texture>>,
) {
    asset_server.load::<Font, &'static str>("FiraMono-Medium.ttf");

    if asset_server.get_group_load_state(textures.iter().map(|(handle_id, _)| handle_id)) == LoadState::Loaded &&
        get_has_map_assets(asset_server, tilemap_atlas_handles)
    {
        game_state.set(GameState::SpawnMap).unwrap();
    }
}
